//! The Batcher service for the sequencer.

use crate::{
    config::BatcherConfig,
    metrics::BatcherMetrics,
    sequencing_batch::{compress_batch, uncompressed_batch, SequencingBatch},
};
use alloy::{
    network::EthereumWallet,
    primitives::{keccak256, Address, Bytes},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    transports::TransportError,
};
use contract_bindings::synd::syndicatesequencerchain::SyndicateSequencerChain::SyndicateSequencerChainInstance;
use derivative::Derivative;
use eyre::{eyre, Result};
use redis::Client as RedisClient;
use shared::types::FilledProvider;
use std::{
    collections::VecDeque,
    str::FromStr,
    time::{Duration, Instant},
};
use synd_maestro::redis::streams::consumer::StreamConsumer;
use tokio::task::JoinHandle;
use tracing::{debug, error, info};

/// Batcher service
#[derive(Derivative)]
#[derivative(Debug)]
struct Batcher {
    /// Whether compression is enabled
    compression_enabled: bool,
    /// The max batch size for the batcher
    max_batch_size: usize,
    /// The Redis consumer for the batcher
    redis_consumer: StreamConsumer,
    /// The sequencing contract provider for the batcher
    sequencing_contract_provider: SyndicateSequencerChainInstance<(), FilledProvider>,
    /// The chain ID for the batcher
    chain_id: u64,
    /// The timeout for the batcher
    timeout: Duration,
    /// Metrics
    metrics: BatcherMetrics,
    /// Outstanding transactions that didn't fit in the last batch
    outstanding_txs: Vec<Bytes>,
}

#[derive(Debug, thiserror::Error)]
enum BatchError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] eyre::Report),

    #[error("Failed to send batch: {0}")]
    SendBatchFailed(String),
}

/// Run the batcher service. Starts the server and listens for batch requests.
pub async fn run_batcher(
    config: &BatcherConfig,
    sequencing_contract_address: Address,
    metrics: BatcherMetrics,
) -> Result<JoinHandle<()>> {
    let client = RedisClient::open(config.redis_url.as_str()).map_err(|e| {
        eyre!("Failed to open Redis client: {}. Redis URL: {}", e, config.redis_url)
    })?;
    let conn = client.get_multiplexed_async_connection().await.map_err(|e| {
        eyre!("Failed to get Redis connection: {}. Redis URL: {}", e, config.redis_url)
    })?;
    let redis_consumer = StreamConsumer::new(conn, config.chain_id, "0-0".to_string());

    let sequencing_contract_provider =
        create_sequencing_contract_provider(config, sequencing_contract_address).await?;

    let mut batcher = Batcher::new(config, redis_consumer, sequencing_contract_provider, metrics);

    let handle = tokio::spawn({
        async move {
            loop {
                debug!("Batcher reading and batching transactions at time {:?}", Instant::now());
                if let Err(e) = batcher.process_transactions().await {
                    error!("Batcher error: {:?}", e);
                }
            }
        }
    });
    info!("Batcher job started");
    Ok(handle)
}

async fn create_sequencing_contract_provider(
    config: &BatcherConfig,
    sequencing_contract_address: Address,
) -> Result<SyndicateSequencerChainInstance<(), FilledProvider>, TransportError> {
    let signer = PrivateKeySigner::from_str(&config.private_key)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err));
    let sequencer_provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .connect(config.sequencing_rpc_url.as_str())
        .await?;
    Ok(SyndicateSequencerChainInstance::new(sequencing_contract_address, sequencer_provider))
}

impl Batcher {
    const fn new(
        config: &BatcherConfig,
        redis_consumer: StreamConsumer,
        sequencing_contract_provider: SyndicateSequencerChainInstance<(), FilledProvider>,
        metrics: BatcherMetrics,
    ) -> Self {
        Self {
            compression_enabled: config.compression_enabled,
            max_batch_size: config.max_batch_size.as_u64() as usize,
            redis_consumer,
            sequencing_contract_provider,
            chain_id: config.chain_id,
            timeout: config.timeout,
            metrics,
            outstanding_txs: Vec::new(),
        }
    }

    async fn read_and_batch_transactions(&mut self) -> Result<SequencingBatch> {
        let start = Instant::now();
        let mut txs = vec![];
        let mut batch = SequencingBatch::Compressed(vec![]); // Initialize with empty compressed batch
        let mut uncompressed_size = 0;

        'outer: loop {
            if start.elapsed() >= self.timeout {
                debug!(%self.chain_id, "Read timeout reached. Stopping transaction read.");
                break;
            }

            // TODO (SEQ-842): Configurable max msg count
            // NOTE: If msg count is >1 we need to handle edge cases where not all transactions fit
            // in the batch
            let incoming_txs = self.redis_consumer.recv(1, Duration::from_millis(100)).await?;

            // Combine outstanding transactions with incoming transactions
            let mut pending_txs: VecDeque<Bytes> = std::mem::take(&mut self.outstanding_txs)
                .into_iter()
                .chain(incoming_txs.into_iter().map(|(tx, _)| Bytes::from(tx)))
                .collect();

            // Process transactions until one doesn't fit
            while let Some(tx_bytes) = pending_txs.front() {
                let proposed_batch = match self.compression_enabled {
                    true => compress_batch(&txs, tx_bytes)?,
                    false => uncompressed_batch(&txs, tx_bytes),
                };

                if proposed_batch.len() > self.max_batch_size {
                    // If the current txs vector is empty, that means the transaction we are trying
                    // to add is too large to fit in a single batch by itself.
                    // We need to discard it or this loop will get stuck trying
                    // to add the same transaction over and over.
                    if txs.is_empty() {
                        let bad_tx = pending_txs.pop_front().unwrap_or_default();
                        error!(%self.chain_id, tx_hash=%keccak256(bad_tx), "Transaction is too large to fit in the batch. Discarding");
                        continue;
                    }

                    // Stop consuming
                    // Save all remaining transactions
                    self.outstanding_txs = pending_txs.into();
                    break 'outer;
                }

                uncompressed_size += tx_bytes.len();
                debug!(
                    %self.chain_id, ?tx_bytes, batch_size = %proposed_batch.len(), "Adding transaction to batch",
                );
                batch = proposed_batch;
                if let Some(tx) = pending_txs.pop_front() {
                    txs.push(tx);
                }
            }
        }
        self.metrics.record_batch_transactions(txs.len());
        self.metrics.record_compression_ratio(uncompressed_size, batch.len());
        Ok(batch)
    }

    async fn process_transactions(&mut self) -> Result<(), BatchError> {
        let start = Instant::now();

        let batch = self.read_and_batch_transactions().await?;
        if batch.is_empty() {
            debug!(%self.chain_id, "No transactions available to batch.");
            return Ok(());
        }

        let submission_start = Instant::now();
        if let Err(e) = self.send_batch(batch).await {
            // Don't reset the compressor here, because we want to retry the same batch
            error!(%self.chain_id, "Failed to send batch: {:?}", e);
            return Err(e);
        }
        self.metrics.record_submission_latency(submission_start.elapsed());
        self.metrics.record_processing_time(start.elapsed());

        Ok(())
    }

    async fn send_batch(&self, batch: SequencingBatch) -> Result<(), BatchError> {
        debug!(
            %self.chain_id, "Batch sent - size: {} bytes",
            batch.len()
        );

        let _ = match batch {
            SequencingBatch::Compressed(batch) => {
                self.sequencing_contract_provider
                    .processTransactionRaw(Bytes::from(batch))
                    .send()
                    .await
            }
            SequencingBatch::Uncompressed(batch) => {
                self.sequencing_contract_provider
                    .processBulkTransactions(
                        batch.iter().map(|tx| Bytes::from(tx.clone())).collect(),
                    )
                    .send()
                    .await
            }
        }
        .map_err(|e| BatchError::SendBatchFailed(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        node_bindings::{Anvil, AnvilInstance},
        primitives::U256,
        providers::ext::AnvilApi,
        transports::mock::Asserter,
    };
    use prometheus_client::registry::Registry;
    use synd_maestro::redis::streams::producer::{CheckFinalizationResult, StreamProducer};
    use test_utils::{docker::start_redis, wait_until};
    use url::Url;

    // Create a mock provider that always succeeds
    async fn create_mock_contract(
        anvil: Option<&AnvilInstance>,
    ) -> SyndicateSequencerChainInstance<(), FilledProvider> {
        let mock_address = Address::from([0; 20]); // Use a dummy address

        let signer = PrivateKeySigner::from_str(
            "0xafdfd9c3d2095ef696594f6cedcae59e72dcd697e2a7521b1578140422a4f890",
        )
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err));

        let mock_provider = if let Some(anvil) = anvil {
            let signer_address =
                Address::from_str("0x06A85356DCb5b307096726FB86A78c59D38e08ee").unwrap();
            let provider = ProviderBuilder::new()
                .wallet(EthereumWallet::from(signer))
                .connect(anvil.endpoint_url().as_str())
                .await
                .unwrap();

            // Set balance at 100 ETH
            let balance = U256::from(100) * U256::from(10).pow(U256::from(18));
            provider.anvil_set_balance(signer_address, balance).await.unwrap();
            provider
        } else {
            let asserter = Asserter::new();
            ProviderBuilder::new().wallet(EthereumWallet::from(signer)).on_mocked_client(asserter)
        };
        SyndicateSequencerChainInstance::new(mock_address, mock_provider)
    }

    fn test_config() -> BatcherConfig {
        BatcherConfig {
            max_batch_size: byte_unit::Byte::from_u64(1024),
            redis_url: "dummy".to_string(),
            chain_id: 1,
            compression_enabled: true,
            timeout: Duration::from_millis(200),
            private_key: "0xafdfd9c3d2095ef696594f6cedcae59e72dcd697e2a7521b1578140422a4f890"
                .to_string(),
            sequencing_rpc_url: Url::parse("http://localhost:8545").unwrap(),
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_read_transactions() {
        let mut config = test_config();
        let (_redis, redis_url) = start_redis().await.unwrap();
        config.redis_url = redis_url.clone();

        let conn = redis::Client::open(redis_url.as_str())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let chain_id = 1;
        let redis_consumer = StreamConsumer::new(conn.clone(), chain_id, "0-0".to_string());
        let producer = StreamProducer::new(
            conn,
            chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
        )
        .await;

        let test_data1 = b"test transaction data 1".to_vec();
        producer.enqueue_transaction(&test_data1).await.unwrap();
        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        let mut batcher =
            Batcher::new(&config, redis_consumer, create_mock_contract(None).await, metrics);

        let result = batcher.read_and_batch_transactions().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_compressed_batch_returns_error_if_too_large() {
        let mut config = test_config();
        config.max_batch_size = byte_unit::Byte::from_u64(1); // force failure
        config.compression_enabled = false;
        let (_redis, redis_url) = start_redis().await.unwrap();
        config.redis_url = redis_url.clone();

        let conn = redis::Client::open(redis_url.as_str())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let chain_id = 1;
        let redis_consumer = StreamConsumer::new(conn.clone(), chain_id, "0-0".to_string());
        let producer = StreamProducer::new(
            conn,
            chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
        )
        .await;

        let test_data1 = b"test transaction data 1".to_vec();
        producer.enqueue_transaction(&test_data1).await.unwrap();
        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        let mut batcher =
            Batcher::new(&config, redis_consumer, create_mock_contract(None).await, metrics);

        let result = batcher.read_and_batch_transactions().await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_read_transactions_no_data() {
        let mut config = test_config();
        let (_redis, redis_url) = start_redis().await.unwrap();
        config.redis_url = redis_url.clone();

        let conn = redis::Client::open(redis_url.as_str())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let redis_consumer = StreamConsumer::new(conn, config.chain_id, "0-0".to_string());

        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        let mut batcher =
            Batcher::new(&config, redis_consumer, create_mock_contract(None).await, metrics);
        let result = batcher.read_and_batch_transactions().await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_multiple_txs() {
        let mut config = test_config();
        let (_redis, redis_url) = start_redis().await.unwrap();
        config.redis_url = redis_url.clone();

        let conn = redis::Client::open(redis_url.as_str())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let redis_consumer = StreamConsumer::new(conn.clone(), config.chain_id, "0-0".to_string());
        let producer = StreamProducer::new(
            conn,
            config.chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
        )
        .await;

        // Add 100 test transactions of ~50KB each
        // Create a 50KB transaction by repeating the pattern
        let base_data = b"test transaction data";
        let mut test_data = Vec::with_capacity(50 * 1024); // 50KB
        while test_data.len() < 50 * 1024 {
            test_data.extend_from_slice(base_data);
        }
        test_data.truncate(50 * 1024); // Ensure exact 50KB
        for _ in 0..100 {
            producer.enqueue_transaction(&test_data).await.unwrap();
        }

        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);
        let metrics_clone = metrics.clone();

        let anvil = Anvil::new().spawn();
        let mut batcher = Batcher::new(
            &config,
            redis_consumer,
            create_mock_contract(Some(&anvil)).await,
            metrics,
        );

        // Run the batcher and verify it sends the batch
        let _handle = tokio::spawn(async move {
            loop {
                batcher.process_transactions().await.unwrap();
            }
        });

        wait_until!(metrics_clone.total_txs_processed.get() == 100, Duration::from_secs(10));
        drop(anvil);
    }

    #[tokio::test]
    async fn test_multiple_txs_uncompressed() {
        let mut config = test_config();
        config.compression_enabled = false;
        config.max_batch_size = byte_unit::Byte::from_u64(51 * 1024); // 51KB

        let (_redis, redis_url) = start_redis().await.unwrap();
        config.redis_url = redis_url.clone();

        let conn = redis::Client::open(redis_url.as_str())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let redis_consumer = StreamConsumer::new(conn.clone(), config.chain_id, "0-0".to_string());
        let producer = StreamProducer::new(
            conn,
            config.chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
        )
        .await;

        // Add 20 test transactions of ~10KB each
        // Create a 10KB transaction by repeating the pattern
        let base_data = b"test transaction data";
        let mut test_data = Vec::with_capacity(10 * 1024); // 10KB
        while test_data.len() < 10 * 1024 {
            test_data.extend_from_slice(base_data);
        }
        test_data.truncate(10 * 1024); // Ensure exact 10KB
        for _ in 0..20 {
            producer.enqueue_transaction(&test_data).await.unwrap();
        }

        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);
        let metrics_clone = metrics.clone();

        let anvil = Anvil::new().spawn();
        let mut batcher = Batcher::new(
            &config,
            redis_consumer,
            create_mock_contract(Some(&anvil)).await,
            metrics,
        );

        // Run the batcher and verify it sends the batch
        let _handle = tokio::spawn(async move {
            loop {
                batcher.process_transactions().await.unwrap();
            }
        });

        wait_until!(metrics_clone.total_txs_processed.get() == 20, Duration::from_secs(10));
        drop(anvil);
    }
}
