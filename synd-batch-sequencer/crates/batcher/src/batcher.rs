//! The Batcher service for the sequencer.

use crate::{config::BatcherConfig, metrics::BatcherMetrics};
use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use byte_unit::Byte;
use contract_bindings::metabased::walletpoolwrappermodule::WalletPoolWrapperModule::{
    self, WalletPoolWrapperModuleInstance,
};
use derivative::Derivative;
use eyre::{eyre, Result};
use maestro::redis::{consumer::StreamConsumer, producer::StreamProducer};
use prometheus_client::registry::Registry;
use redis::Client as RedisClient;
use shared::{additive_compression::AdditiveCompressor, types::FilledProvider};
use std::{
    mem::take,
    pin::pin,
    str::FromStr,
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tc_client::tc_client::{TCClient, BATCH_FUNCTION_SIGNATURE};
use tokio::{sync::Semaphore, task::JoinHandle};
use tracing::{debug, error, info};

/// Batcher service
#[derive(Derivative)]
#[derivative(Debug)]
struct Batcher {
    /// The max batch size for the batcher
    max_batch_size: Byte,
    /// The Redis consumer for the batcher
    redis_consumer: StreamConsumer,
    /// The batch sender
    #[derivative(Debug = "ignore")]
    batch_sender: BatchSender,
    /// The compressor for the batcher
    compressor: AdditiveCompressor,
    /// The chain ID for the batcher
    chain_id: u64,
    /// The timeout for the batcher
    timeout: Duration,
    /// Submission Semaphore
    submission_semaphore: Arc<Semaphore>,
    /// Metrics
    metrics: BatcherMetrics,
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

    let wallet_pool = create_wallet_pool(config, wallet_pool_address);
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
    info!("Batcher job started with");
    Ok(handle)
}

fn create_wallet_pool(
    config: &BatcherConfig,
    wallet_pool_address: Address,
) -> WalletPoolWrapperModuleInstance<(), FilledProvider> {
    let signer = PrivateKeySigner::from_str(&config.private_key)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err));
    let sequencer_provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .on_http(config.sequencing_rpc_url.clone());
    WalletPoolWrapperModule::new(wallet_pool_address, sequencer_provider)
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
            compressor: AdditiveCompressor::default(),
            chain_id: config.chain_id,
            timeout: config.timeout,
            submission_semaphore: Arc::new(Semaphore::new(1)), //Only one submission at a time
            metrics,
        }
    }
    /// reads transactions from redis and returns a compressed batch
    async fn read_transactions(&mut self) -> Result<()> {
        let start = Instant::now();
        loop {
            if start.elapsed() >= self.timeout {
                debug!(%self.chain_id, "Read timeout reached. Stopping transaction read.");
                break;
            }

            // TODO (SEQ-842): Configurable max msg count
            // NOTE: If msg count is >1 we need to handle edge cases where not all transactions fit
            // in the batch
            let transactions = self.redis_consumer.recv(1, Duration::from_millis(100)).await?;

            if transactions.is_empty() {
                debug!("No transactions available to batch.");
                break;
            }
            // let original_size = 0;
            // let compressed = Bytes::new();

            for (txn_bytes, _) in transactions {
                let txn = Bytes::from(txn_bytes);

                let compressed = self.compressor.clone_and_compress_with_txn(&txn)?;

                if compressed.len() as u64 > self.max_batch_size.as_u64() {
                    // Stop consuming and let the caller finalize the batch
                    // let num_txs = self.compressor.num_transactions();
                    // self.metrics.record_batch_transactions(num_txs);
                    // self.metrics.record_compression_ratio(original_size, compressed.len());
                    return Ok(());
                }
                // original_size += txn.len();

                debug!(
                    %self.chain_id, "Adding transaction to batch: {:?} - compressed size: {}",
                    txn,
                    compressed.len()
                );
                self.compressor.push_with_precompressed(&txn, compressed)?;
            }
        }
        Ok(())
    }

    async fn read_and_batch_transactions(&mut self) -> Result<(), BatchError> {
        let start = Instant::now();
        // Ensure only one submission runs at a time by holding the semaphore permit
        let semaphore = Arc::clone(&self.submission_semaphore);
        let _permit = match semaphore.try_acquire() {
            Ok(permit) => permit,
            Err(_) => {
                debug!(%self.chain_id, "Submission in flight, skipping read.");
                return Ok(());
            }
        };

        // _permit is held until the end of the function

        self.read_transactions().await?;

        if self.compressor.is_empty() {
            debug!(%self.chain_id, "No transactions available to batch.");
            return Ok(());
        }

        let submission_start = Instant::now();
        if let Err(e) = self.send_compressed_batch().await {
            // Don't reset the compressor here, because we want to retry the same batch
            error!(%self.chain_id, "Failed to send compressed batch: {:?}", e);
            return Err(e);
        }
        self.metrics.record_submission_latency(submission_start.elapsed());

        self.metrics.record_processing_time(start.elapsed());
        self.metrics.record_batch_timestamp();

        self.compressor.reset(); // Reset for next round
        Ok(())
    }

    async fn send_compressed_batch(&mut self) -> Result<(), BatchError> {
        let num_transactions = self.compressor.num_transactions();
        debug!(%self.chain_id, "Sending compressed batch with {} transactions", num_transactions);
        let compressed = take(&mut self.compressor).finish()?;

        let compressed_len = compressed.len();
        let max_size = self.max_batch_size.as_u64() as usize;

        if compressed_len >= max_size {
            error!(%self.chain_id, "Compressed batch size ({}) exceeds limit ({})", compressed_len, max_size);
            return Err(BatchError::BatchTooLarge(compressed_len, max_size));
        }

        (self.batch_sender)(compressed.clone()).await?;

        debug!(
            %self.chain_id, "Batch sent: {} txs, compressed size: {} bytes",
            num_transactions,
            compressed_len
        );
        self.metrics.increment_total_txs_processed(num_transactions);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maestro::redis::producer::StreamProducer;
    use tc_client::config::TCEndpoint;
    use test_utils::{docker::start_redis, wait_until};
    use url::Url;

    fn create_noop_sender() -> BatchSender {
        Box::new(|_data| Box::pin(async move { Ok(()) }))
    }

    fn test_config() -> BatcherConfig {
        BatcherConfig {
            max_batch_size: Byte::from_u64(1024),
            redis_url: "dummy".to_string(),
            chain_id: 1,
            timeout: Duration::from_millis(200),
            private_key: "0xafdfd9c3d2095ef696594f6cedcae59e72dcd697e2a7521b1578140422a4f890"
                .to_string(),
            sequencing_rpc_url: Url::parse("http://localhost:8545").unwrap(),
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_read_transactions_adds_to_compressor() {
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
        producer.enqueue_transaction(test_data1.clone()).await.unwrap();
        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);
        let mut batcher = Batcher::new(&config, redis_consumer, create_noop_sender(), metrics);

        let result = batcher.read_transactions().await;
        assert!(result.is_ok());
        assert!(!batcher.compressor.is_empty(), "Compressor should not be empty after read");
    }

    #[tokio::test]
    async fn test_send_compressed_batch_returns_error_if_too_large() {
        let config = BatcherConfig {
            max_batch_size: Byte::from_u64(1), // force failure
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

        let mut batcher = Batcher::new(&config, redis_consumer, create_noop_sender(), metrics);

        batcher.compressor.clone_and_compress_with_txn(&Bytes::from(vec![2, 3, 4])).unwrap();

        let result = batcher.send_compressed_batch().await;
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
        let result = batcher.read_transactions().await;

        assert!(result.is_ok());
        assert!(batcher.compressor.is_empty(), "Compressor should be empty");
    }

    #[tokio::test]
    async fn test_submission_in_flight_skips_batching() {
        let config = test_config();
        let (_redis, redis_url) = start_redis().await.unwrap();

        let conn = redis::Client::open(redis_url.as_str())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let redis_consumer = StreamConsumer::new(conn, config.chain_id, "0-0".to_string());

        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        let mut batcher = Batcher::new(&config, redis_consumer, create_noop_sender(), metrics);

        let semaphore = Arc::clone(&batcher.submission_semaphore);
        let _guard = semaphore.try_acquire().expect("should acquire permit");

        let result = batcher.read_and_batch_transactions().await;
        assert!(result.is_ok(), "Expected batching to skip and return Ok");
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
        test_data.truncate(50 * 1024); // Ensure exact 50KiB
        for _ in 0..100 {
            producer.enqueue_transaction(test_data.clone()).await.unwrap();
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

        // TODO remove
        tokio::time::sleep(Duration::from_secs(10)).await;
        println!("Total txs processed: {}", metrics_clone.total_txs_processed.get());

        wait_until!(metrics_clone.total_txs_processed.get() == 100, Duration::from_secs(10));
    }
}
