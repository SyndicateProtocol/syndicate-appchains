//! The Batcher service for the sequencer.

use crate::{
    config::BatcherConfig,
    metrics::BatcherMetrics,
    sequencing_batch::{compress_batch, uncompressed_batch, SequencingBatch},
};
use alloy::{
    network::EthereumWallet,
    primitives::{keccak256, Bytes},
    providers::{Provider, ProviderBuilder, WalletProvider},
    rpc::client::RpcClient,
    signers::local::PrivateKeySigner,
    transports::{layers::RetryBackoffLayer, TransportError},
};
use contract_bindings::synd::syndicate_sequencing_chain::SyndicateSequencingChain::SyndicateSequencingChainInstance;
use derivative::Derivative;
use eyre::{eyre, Result};
use redis::{aio::ConnectionManager, Client as ValkeyClient};
use shared::{
    service_start_utils::{
        cache_health_check_handler, start_http_server_with_aux_handlers, MetricsState,
    },
    tracing::SpanKind,
    types::FilledProvider,
};
use std::{
    collections::VecDeque,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant},
};
use synd_maestro::{valkey::streams::consumer::StreamConsumer, valkey_metrics::ValkeyMetrics};
use tokio::task::JoinHandle;
use tracing::{debug, error, info, instrument, trace, warn};

/// Batcher service
#[derive(Derivative)]
#[derivative(Debug)]
struct Batcher {
    /// Whether compression is enabled (default: true)
    compression_enabled: bool,
    /// The max batch size for the batcher (default: 90KB)
    max_batch_size: usize,
    /// The Stream consumer for the batcher
    stream_consumer: StreamConsumer,
    /// The sequencing contract provider for the batcher
    sequencing_contract_instance: SyndicateSequencingChainInstance<FilledProvider>,
    /// The chain ID for the batcher
    chain_id: u64,
    /// The timeout for the batcher (default: 200ms)
    timeout: Duration,
    /// Metrics
    metrics: BatcherMetrics,
    /// Outstanding transactions that didn't fit in the last batch
    outstanding_txs: Vec<Bytes>,
    /// Whether to wait for the receipt of the batch submission
    wait_for_receipt: bool,
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
pub async fn run_batcher(config: &BatcherConfig) -> Result<JoinHandle<()>> {
    let client = ValkeyClient::open(config.valkey_url.as_str()).map_err(|e| {
        eyre!("Failed to open Valkey client: {}. Valkey URL: {}", e, config.valkey_url)
    })?;
    let conn = ConnectionManager::new(client).await.map_err(|e| {
        eyre!("Failed to get Valkey connection: {}. Valkey URL: {}", e, config.valkey_url)
    })?;

    // Start metrics and health endpoints
    let mut metrics_state = MetricsState::default();
    let registry = &mut metrics_state.registry;
    let metrics = BatcherMetrics::new(registry);
    let valkey_metrics = ValkeyMetrics::new(registry);

    let cache_health_handler = cache_health_check_handler(conn.clone());
    tokio::spawn(start_http_server_with_aux_handlers(
        metrics_state,
        config.port,
        Some(cache_health_handler.clone()),
        Some(cache_health_handler), // same /ready behavior as /health
    ));

    let sequencing_contract_instance = create_sequencing_contract_instance(config).await?;

    let stream_consumer = StreamConsumer::new(
        conn.clone(),
        config.chain_id,
        "0-0".to_string(),
        Arc::new(valkey_metrics),
    );
    let mut batcher = Batcher::new(config, stream_consumer, sequencing_contract_instance, metrics);

    let handle = tokio::spawn({
        async move {
            loop {
                debug!("Batcher reading and batching transactions at time {:?}", Instant::now());
                if let Err(e) = batcher.process_transactions().await {
                    error!("Batcher error: {e:?}");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    });
    info!("Batcher job started");
    Ok(handle)
}

async fn create_sequencing_contract_instance(
    config: &BatcherConfig,
) -> Result<SyndicateSequencingChainInstance<FilledProvider>, TransportError> {
    let signer = PrivateKeySigner::from_str(&config.private_key)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {err}"));

    let rpc_client = {
        RetryBackoffLayer::new(
            config.rpc_max_retries,
            config.rpc_initial_backoff_ms,
            config.rpc_compute_units_per_second,
        )
        .with_avg_unit_cost(config.rpc_compute_units_avg_request_cost);
        RpcClient::builder().layer(())
    }
    .connect(config.sequencing_rpc_url.as_str())
    .await?;

    let sequencing_provider =
        ProviderBuilder::new().wallet(EthereumWallet::from(signer)).connect_client(rpc_client);
    Ok(SyndicateSequencingChainInstance::new(config.sequencing_address, sequencing_provider))
}

impl Batcher {
    const fn new(
        config: &BatcherConfig,
        stream_consumer: StreamConsumer,
        sequencing_contract_instance: SyndicateSequencingChainInstance<FilledProvider>,
        metrics: BatcherMetrics,
    ) -> Self {
        Self {
            compression_enabled: config.compression_enabled,
            max_batch_size: config.max_batch_size.as_u64() as usize,
            stream_consumer,
            sequencing_contract_instance,
            chain_id: config.chain_id,
            timeout: config.timeout,
            wait_for_receipt: config.wait_for_receipt,
            metrics,
            outstanding_txs: Vec::new(),
        }
    }

    #[instrument(
        skip(self),
        err,
        fields(
            otel.kind = ?SpanKind::Consumer,
            chain_id = %self.chain_id
        )
    )]
    async fn process_transactions(&mut self) -> Result<(), BatchError> {
        let start = Instant::now();

        let batch = self.read_and_batch_transactions().await?;
        if batch.is_empty() {
            trace!(%self.chain_id, "No transactions available to batch.");
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

    #[instrument(
        skip(self),
        err,
        fields(
            otel.kind = ?SpanKind::Consumer,
            chain_id = %self.chain_id
        )
    )]
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
            let incoming_txs = self.stream_consumer.recv(1, Duration::from_millis(100)).await?;

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
                    // We need to discard it, or this loop will get stuck trying
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

    #[instrument(
        skip_all,
        err,
        fields(
            otel.kind = ?SpanKind::Producer,
            chain_id = %self.chain_id,
            batch_size = %batch.len()
        )
    )]
    async fn send_batch(&self, batch: SequencingBatch) -> Result<(), BatchError> {
        debug!(
            %self.chain_id, "Sending Batch - size: {} bytes",
            batch.len()
        );

        let transaction_request = match batch {
            SequencingBatch::Compressed(batch) => self
                .sequencing_contract_instance
                .processTransaction(Bytes::from(batch))
                .into_transaction_request(),
            SequencingBatch::Uncompressed(batch) => self
                .sequencing_contract_instance
                .processTransactionsBulk(batch.iter().map(|tx| Bytes::from(tx.clone())).collect())
                .into_transaction_request(),
        };

        let pending_tx = self
            .sequencing_contract_instance
            .provider()
            .send_transaction(transaction_request.clone())
            .await
            .map_err(|e| BatchError::SendBatchFailed(e.to_string()))?;

        if self.wait_for_receipt {
            let receipt = pending_tx
                .get_receipt()
                .await
                .map_err(|e| BatchError::SendBatchFailed(e.to_string()))?;
            warn!(%self.chain_id, "Batch submitted and receipt received: {:?}", receipt);
            if !receipt.status() {
                error!(%self.chain_id, "Batch submission failed. tx: {:?}, receipt: {:?}", transaction_request, receipt);
                return Err(BatchError::SendBatchFailed("Batch submission failed".to_string()));
            }
        } else {
            debug!(%self.chain_id, "Batch submitted");
        }

        // Record the wallet balance in the background
        self.record_wallet_balance();

        Ok(())
    }

    fn record_wallet_balance(&self) {
        let provider = self.sequencing_contract_instance.provider().clone();
        let metrics = self.metrics.clone();
        tokio::spawn(async move {
            let wallet_address = provider.default_signer_address();

            match provider.get_balance(wallet_address).await {
                Ok(balance) => {
                    metrics.record_wallet_balance(balance.to());
                }
                Err(e) => {
                    error!("Failed to get wallet balance: {:?}", e);
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        node_bindings::{Anvil, AnvilInstance},
        primitives::{Address, U256},
        providers::ext::AnvilApi,
        transports::mock::Asserter,
    };
    use prometheus_client::registry::Registry;
    use reqwest;
    use std::sync::Arc;
    use synd_maestro::{
        valkey::streams::producer::{CheckFinalizationResult, StreamProducer},
        valkey_metrics::ValkeyMetrics,
    };
    use test_utils::{docker::start_valkey, port_manager::PortManager, wait_until};
    use url::Url;

    // Create a mock provider that always succeeds
    async fn create_mock_contract(
        anvil: Option<&AnvilInstance>,
    ) -> SyndicateSequencingChainInstance<FilledProvider> {
        let mock_address = Address::from([0; 20]); // Use a dummy address

        let signer = PrivateKeySigner::from_str(
            "0xafdfd9c3d2095ef696594f6cedcae59e72dcd697e2a7521b1578140422a4f890",
        )
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {err}"));

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
            ProviderBuilder::new()
                .wallet(EthereumWallet::from(signer))
                .connect_mocked_client(asserter)
        };
        SyndicateSequencingChainInstance::new(mock_address, mock_provider)
    }

    fn test_config() -> BatcherConfig {
        BatcherConfig {
            max_batch_size: byte_unit::Byte::from_u64(1024),
            valkey_url: "dummy".to_string(),
            chain_id: 1,
            compression_enabled: true,
            timeout: Duration::from_millis(200),
            private_key: "0xafdfd9c3d2095ef696594f6cedcae59e72dcd697e2a7521b1578140422a4f890"
                .to_string(),
            sequencing_rpc_url: Url::parse("http://localhost:8545").unwrap(),
            ..Default::default()
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_read_transactions() {
        let mut config = test_config();
        let (_valkey, valkey_url) = start_valkey().await.unwrap();
        config.valkey_url = valkey_url.clone();

        let conn = ConnectionManager::new(redis::Client::open(valkey_url.as_str()).unwrap())
            .await
            .unwrap();
        let chain_id = 1;
        let valkey_metrics = Arc::new(ValkeyMetrics::default());
        let stream_consumer =
            StreamConsumer::new(conn.clone(), chain_id, "0-0".to_string(), valkey_metrics);
        let producer = StreamProducer::new(
            conn,
            chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
            Arc::new(ValkeyMetrics::default()),
        )
        .await;

        let test_data1 = b"test transaction data 1".to_vec();
        producer.enqueue_transaction(&test_data1).await.unwrap();
        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        let sequencing_contract_instance = create_mock_contract(None).await;
        let mut batcher =
            Batcher::new(&config, stream_consumer, sequencing_contract_instance, metrics);

        let result = batcher.read_and_batch_transactions().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_compressed_batch_returns_error_if_too_large() {
        let mut config = test_config();
        config.max_batch_size = byte_unit::Byte::from_u64(1); // force failure
        config.compression_enabled = false;
        let (_valkey, valkey_url) = start_valkey().await.unwrap();
        config.valkey_url = valkey_url.clone();
        let valkey_metrics = Arc::new(ValkeyMetrics::default());

        let conn = ConnectionManager::new(redis::Client::open(valkey_url.as_str()).unwrap())
            .await
            .unwrap();
        let chain_id = 1;
        let stream_consumer =
            StreamConsumer::new(conn.clone(), chain_id, "0-0".to_string(), valkey_metrics);
        let producer = StreamProducer::new(
            conn,
            chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
            Arc::new(ValkeyMetrics::default()),
        )
        .await;

        let test_data1 = b"test transaction data 1".to_vec();
        producer.enqueue_transaction(&test_data1).await.unwrap();
        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        let sequencing_contract_instance = create_mock_contract(None).await;
        let mut batcher =
            Batcher::new(&config, stream_consumer, sequencing_contract_instance, metrics);

        let result = batcher.read_and_batch_transactions().await;
        assert!(result.is_ok(), "result: {result:?}");
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_read_transactions_no_data() {
        let mut config = test_config();
        let (_valkey, valkey_url) = start_valkey().await.unwrap();
        config.valkey_url = valkey_url.clone();
        let valkey_metrics = Arc::new(ValkeyMetrics::default());

        let conn = ConnectionManager::new(redis::Client::open(valkey_url.as_str()).unwrap())
            .await
            .unwrap();
        let stream_consumer =
            StreamConsumer::new(conn, config.chain_id, "0-0".to_string(), valkey_metrics);

        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        let sequencing_contract_instance = create_mock_contract(None).await;
        let mut batcher =
            Batcher::new(&config, stream_consumer, sequencing_contract_instance, metrics);
        let result = batcher.read_and_batch_transactions().await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_multiple_txs() {
        let mut config = test_config();
        let (_valkey, valkey_url) = start_valkey().await.unwrap();
        config.valkey_url = valkey_url.clone();
        let valkey_metrics = Arc::new(ValkeyMetrics::default());

        let conn = ConnectionManager::new(redis::Client::open(valkey_url.as_str()).unwrap())
            .await
            .unwrap();
        let stream_consumer =
            StreamConsumer::new(conn.clone(), config.chain_id, "0-0".to_string(), valkey_metrics);
        let producer = StreamProducer::new(
            conn,
            config.chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
            Arc::new(ValkeyMetrics::default()),
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
        let sequencing_contract_instance = create_mock_contract(Some(&anvil)).await;
        let mut batcher =
            Batcher::new(&config, stream_consumer, sequencing_contract_instance, metrics);

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

        let (_valkey, valkey_url) = start_valkey().await.unwrap();
        config.valkey_url = valkey_url.clone();
        let valkey_metrics = Arc::new(ValkeyMetrics::default());

        let conn = ConnectionManager::new(redis::Client::open(valkey_url.as_str()).unwrap())
            .await
            .unwrap();
        let stream_consumer =
            StreamConsumer::new(conn.clone(), config.chain_id, "0-0".to_string(), valkey_metrics);
        let producer = StreamProducer::new(
            conn,
            config.chain_id,
            Duration::from_secs(60),
            Duration::from_secs(60),
            0,
            |_| async { CheckFinalizationResult::Done },
            Arc::new(ValkeyMetrics::default()),
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
        let sequencing_contract_instance = create_mock_contract(Some(&anvil)).await;
        let mut batcher =
            Batcher::new(&config, stream_consumer, sequencing_contract_instance, metrics);

        // Run the batcher and verify it sends the batch
        let _handle = tokio::spawn(async move {
            loop {
                batcher.process_transactions().await.unwrap();
            }
        });

        wait_until!(metrics_clone.total_txs_processed.get() == 20, Duration::from_secs(10));
        drop(anvil);
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let (valkey, valkey_url) = start_valkey().await.unwrap();
        let config = BatcherConfig {
            max_batch_size: byte_unit::Byte::from_u64(1024),
            valkey_url,
            chain_id: 1,
            compression_enabled: true,
            timeout: Duration::from_millis(200),
            private_key: "0xafdfd9c3d2095ef696594f6cedcae59e72dcd697e2a7521b1578140422a4f890"
                .to_string(),
            sequencing_rpc_url: Url::parse("http://localhost:8545").unwrap(),
            rpc_max_retries: 10,
            port: PortManager::instance().next_port().await,
            sequencing_address: Address::ZERO,
            ..Default::default()
        };

        let _handle = run_batcher(&config).await.unwrap();

        let url = format!("http://0.0.0.0:{}/health", config.port);

        let client = reqwest::Client::new();

        // Should succeed
        wait_until!(
            client.get(&url).send().await.is_ok_and(|resp| resp.status() == 200),
            Duration::from_secs(2)
        );

        // Close Valkey container and test failure
        drop(valkey);

        wait_until!(
            client.get(&url).send().await.is_ok_and(|resp| resp.status() == 500),
            Duration::from_secs(2)
        );
    }
}
