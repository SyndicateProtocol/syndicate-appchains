//! The Batcher service for the sequencer.

use crate::{batch_compression::compress_batch, config::BatcherConfig, metrics::BatcherMetrics};
use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    transports::TransportError,
};
use contract_bindings::synd::walletpoolwrappermodule::WalletPoolWrapperModule::{
    self, WalletPoolWrapperModuleInstance,
};
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
use tc_client::tc_client::{TCClient, BATCH_FUNCTION_SIGNATURE};
use tokio::task::JoinHandle;
use tracing::{debug, error, info};

/// Batcher service
#[derive(Derivative)]
#[derivative(Debug)]
struct Batcher {
    /// The max batch size for the batcher
    max_batch_size: byte_unit::Byte,
    /// The Redis consumer for the batcher
    redis_consumer: StreamConsumer,
    /// The batch sender
    #[derivative(Debug = "ignore")]
    batch_sender: BatchSender,
    /// The chain ID for the batcher
    chain_id: u64,
    /// The timeout for the batcher
    timeout: Duration,
    /// Metrics
    metrics: BatcherMetrics,
    /// Outstanding transactions that didn't fit in the last batch
    outstanding_txs: Vec<Bytes>,
}
type BatchSender = Box<
    dyn Fn(
            Bytes,
        )
            -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), BatchError>> + Send>>
        + Send
        + Sync,
>;

#[derive(Debug, thiserror::Error)]
enum BatchError {
    #[error("Compressed batch too large: {0} bytes (limit: {1} bytes)")]
    BatchTooLarge(usize, usize),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] eyre::Report),

    #[error("Failed to send batch: {0}")]
    SendBatchFailed(String),
}

fn create_tc_batch_sender(tc_client: TCClient, chain_id: u64) -> BatchSender {
    Box::new(move |data| {
        let tc_client = tc_client.clone();
        Box::pin(async move {
            debug!(%chain_id, "Sending batch to TC");
            let result =
                tc_client.send_transaction(data, BATCH_FUNCTION_SIGNATURE.to_string()).await;
            if let Err(e) = result {
                return Err(BatchError::SendBatchFailed(e.to_string()));
            }
            Ok(())
        })
    })
}

fn create_l2_batch_sender(
    wallet_pool: WalletPoolWrapperModuleInstance<(), FilledProvider>,
    sequencing_contract_address: Address,
    chain_id: u64,
) -> BatchSender {
    Box::new(move |data| {
        let wallet_pool = wallet_pool.clone();
        Box::pin(async move {
            debug!(%chain_id, "Sending batch to L2");
            let result =
                wallet_pool.processTransactionRaw(sequencing_contract_address, data).send().await;
            info!("Batch sent result: {:?}", result);
            if let Err(e) = result {
                return Err(BatchError::SendBatchFailed(e.to_string()));
            }
            Ok(())
        })
    })
}

/// Run the batcher service. Starts the server and listens for batch requests.
pub async fn run_batcher(
    config: &BatcherConfig,
    tc_client: Option<TCClient>,
    wallet_pool_address: Address,
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

    let wallet_pool = create_wallet_pool(config, wallet_pool_address).await?;
    let chain_id = config.chain_id;

    let batch_sender: BatchSender = tc_client.map_or_else(
        || create_l2_batch_sender(wallet_pool, sequencing_contract_address, chain_id),
        |tc_client| create_tc_batch_sender(tc_client, chain_id),
    );

    let mut batcher = Batcher::new(config, redis_consumer, batch_sender, metrics);

    let handle = tokio::spawn({
        async move {
            loop {
                debug!("Batcher reading and batching transactions at time {:?}", Instant::now());
                if let Err(e) = batcher.read_and_batch_transactions().await {
                    error!("Batcher error: {:?}", e);
                }
            }
        }
    });
    info!("Batcher job started");
    Ok(handle)
}

async fn create_wallet_pool(
    config: &BatcherConfig,
    wallet_pool_address: Address,
) -> Result<WalletPoolWrapperModuleInstance<(), FilledProvider>, TransportError> {
    let signer = PrivateKeySigner::from_str(&config.private_key)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err));
    let sequencer_provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .connect(config.sequencing_rpc_url.as_str())
        .await?;
    Ok(WalletPoolWrapperModule::new(wallet_pool_address, sequencer_provider))
}

impl Batcher {
    fn new(
        config: &BatcherConfig,
        redis_consumer: StreamConsumer,
        batch_sender: BatchSender,
        metrics: BatcherMetrics,
    ) -> Self {
        Self {
            max_batch_size: config.max_batch_size,
            redis_consumer,
            batch_sender,
            chain_id: config.chain_id,
            timeout: config.timeout,
            metrics,
            outstanding_txs: Vec::new(),
        }
    }

    async fn read_and_compress_transactions(&mut self) -> Result<Vec<u8>> {
        let start = Instant::now();
        let mut txs = vec![];
        let mut compressed = Vec::<u8>::new();
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
                let compressed_next = compress_batch(&txs, tx_bytes)?;

                if compressed_next.len() as u64 > self.max_batch_size.as_u64() {
                    // Stop consuming
                    // Save all remaining transactions
                    self.outstanding_txs = pending_txs.into();
                    break 'outer;
                }
                uncompressed_size += tx_bytes.len();
                debug!(
                    %self.chain_id, "Adding transaction to batch: {:?} - compressed size: {}",
                    tx_bytes,
                    compressed_next.len()
                );
                compressed = compressed_next;
                if let Some(tx) = pending_txs.pop_front() {
                    txs.push(tx);
                }
            }
        }
        self.metrics.record_batch_transactions(txs.len());
        self.metrics.record_compression_ratio(uncompressed_size, compressed.len());
        Ok(compressed)
    }

    async fn read_and_batch_transactions(&mut self) -> Result<(), BatchError> {
        let start = Instant::now();

        let bytes = self.read_and_compress_transactions().await?;

        if bytes.is_empty() {
            debug!(%self.chain_id, "No transactions available to batch.");
            return Ok(());
        }

        let submission_start = Instant::now();
        if let Err(e) = self.send_compressed_batch(bytes).await {
            // Don't reset the compressor here, because we want to retry the same batch
            error!(%self.chain_id, "Failed to send compressed batch: {:?}", e);
            return Err(e);
        }
        self.metrics.record_submission_latency(submission_start.elapsed());
        self.metrics.record_processing_time(start.elapsed());

        Ok(())
    }

    async fn send_compressed_batch(&self, bytes: Vec<u8>) -> Result<(), BatchError> {
        let max_size = self.max_batch_size.as_u64() as usize;

        if bytes.len() >= max_size {
            error!(%self.chain_id, "Compressed batch size ({}) exceeds limit ({})", bytes.len(), max_size);
            return Err(BatchError::BatchTooLarge(bytes.len(), max_size));
        }

        debug!(
            %self.chain_id, "Batch sent - compressed size: {} bytes",
            bytes.len()
        );
        (self.batch_sender)(Bytes::from(bytes)).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prometheus_client::registry::Registry;
    use synd_maestro::redis::streams::producer::StreamProducer;
    use test_utils::{docker::start_redis, wait_until};
    use url::Url;

    fn create_noop_sender() -> BatchSender {
        Box::new(|_data| Box::pin(async move { Ok(()) }))
    }

    fn test_config() -> BatcherConfig {
        BatcherConfig {
            max_batch_size: byte_unit::Byte::from_u64(1024),
            redis_url: "dummy".to_string(),
            chain_id: 1,
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
        let producer =
            StreamProducer::new(conn, chain_id, Duration::from_secs(60), Duration::from_secs(60));

        let test_data1 = b"test transaction data 1".to_vec();
        producer.enqueue_transaction(&test_data1).await.unwrap();
        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);
        let mut batcher = Batcher::new(&config, redis_consumer, create_noop_sender(), metrics);

        let result = batcher.read_and_compress_transactions().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_compressed_batch_returns_error_if_too_large() {
        let config = BatcherConfig {
            max_batch_size: byte_unit::Byte::from_u64(1), // force failure
            ..test_config()
        };
        let (_redis, redis_url) = start_redis().await.unwrap();

        let conn = redis::Client::open(redis_url.as_str())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let redis_consumer = StreamConsumer::new(conn, 1, "0-0".to_string());

        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        let batcher = Batcher::new(&config, redis_consumer, create_noop_sender(), metrics);

        let result = batcher.send_compressed_batch(vec![2, 3, 4]).await;
        assert!(result.is_err());
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

        let mut batcher = Batcher::new(&config, redis_consumer, create_noop_sender(), metrics);
        let result = batcher.read_and_compress_transactions().await;

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
        );

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

        let mut batcher = Batcher::new(&config, redis_consumer, create_noop_sender(), metrics);

        // Run the batcher and verify it sends the batch
        let _handle = tokio::spawn(async move {
            loop {
                batcher.read_and_batch_transactions().await.unwrap();
            }
        });

        wait_until!(metrics_clone.total_txs_processed.get() == 100, Duration::from_secs(10));
    }
}
