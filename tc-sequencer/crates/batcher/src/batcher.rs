//! The Batcher service for the sequencer.

use crate::config::BatcherConfig;
use alloy::primitives::Bytes;
use byte_unit::Byte;
use eyre::{eyre, Result};
use maestro::redis::consumer::StreamConsumer;
use redis::Client as RedisClient;
use shared::additive_compression::AdditiveCompressor;
use std::{mem::take, time::Duration};
use tc_client::tc_client::{TCClient, BATCH_FUNCTION_SIGNATURE};
use tokio::task::JoinHandle;
use tracing::{debug, error, info};
/// Batcher service
#[derive(Debug)]
struct Batcher {
    /// The max batch size for the batcher
    max_batch_size: Byte,
    /// The Redis consumer for the batcher
    redis_consumer: StreamConsumer,
    /// The sequencer client for the batcher
    tc_client: TCClient,
    /// The compressor for the batcher
    compressor: AdditiveCompressor,
    /// The polling interval for the batcher
    polling_interval: Duration,
    /// The chain ID for the batcher
    chain_id: u64,
}

#[derive(Debug, thiserror::Error)]
enum BatchError {
    #[error("Compressed batch too large: {0} bytes (limit: {1} bytes)")]
    BatchTooLarge(usize, usize),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] eyre::Report),
}

/// Run the batcher service. Starts the server and listens for batch requests.
pub async fn run_batcher(config: &BatcherConfig, tc_client: TCClient) -> Result<JoinHandle<()>> {
    let client = RedisClient::open(config.redis_url.as_str())
        .map_err(|e| eyre!("Failed to open Redis client: {}", e))?;
    let conn = client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| eyre!("Failed to get Redis connection: {}", e))?;
    let redis_consumer = StreamConsumer::new(conn, config.chain_id, "0-0".to_string());
    let mut batcher = Batcher::new(config, tc_client, redis_consumer);
    let polling_interval = config.polling_interval;

    let handle = tokio::spawn({
        async move {
            loop {
                if let Err(e) = batcher.read_and_batch_transactions().await {
                    error!("Batcher error: {:?}", e);
                }
                // In theory this could be removed, but we want to wait a reasonable amount of time
                // to batch as many transactions as possible.
                tokio::time::sleep(polling_interval).await;
            }
        }
    });
    info!("Batcher job started with {:?} poll interval", config.polling_interval);
    Ok(handle)
}

impl Batcher {
    fn new(config: &BatcherConfig, tc_client: TCClient, redis_consumer: StreamConsumer) -> Self {
        Self {
            max_batch_size: config.max_batch_size,
            redis_consumer,
            tc_client,
            compressor: AdditiveCompressor::default(),
            polling_interval: config.polling_interval,
            chain_id: config.chain_id,
        }
    }
    async fn read_transactions(&mut self) -> Result<()> {
        loop {
            // TODO (SEQ-842): Configurable max msg count
            let transactions = self.redis_consumer.recv(1, self.polling_interval).await?;

            if transactions.is_empty() {
                debug!("No transactions available to batch.");
                break;
            }

            for (txn_bytes, _) in transactions {
                let txn = Bytes::from(txn_bytes);

                let compressed = self.compressor.clone_and_compress_with_txn(&txn)?;

                if compressed.len() as u64 >= self.max_batch_size.as_u64() {
                    // Stop consuming and let the caller finalize the batch
                    return Ok(());
                }
                debug!(
                    "[Chain ID: {}] - Adding transaction to batch: {:?} - compressed size: {}",
                    self.chain_id,
                    txn,
                    compressed.len()
                );
                self.compressor.push_with_precompressed(&txn, compressed)?;
            }
        }
        Ok(())
    }

    async fn read_and_batch_transactions(&mut self) -> Result<(), BatchError> {
        self.read_transactions().await?;

        if self.compressor.is_empty() {
            debug!("[Chain ID: {}] - No transactions available to batch.", self.chain_id);
            return Ok(());
        }

        self.send_compressed_batch().await
    }

    async fn send_compressed_batch(&mut self) -> Result<(), BatchError> {
        let num_transactions = self.compressor.num_transactions();
        let compressed = take(&mut self.compressor).finish()?;
        self.compressor.reset(); // Reset for next round

        let compressed_len = compressed.len();
        let max_size = self.max_batch_size.as_u64() as usize;

        if compressed_len >= max_size {
            error!("Compressed batch size ({}) exceeds limit ({})", compressed_len, max_size);
            return Err(BatchError::BatchTooLarge(compressed_len, max_size));
        }

        self.send_batch_to_sequencer(compressed.clone()).await?;
        debug!(
            "[Chain ID: {}] - Batch sent: {} txs, compressed size: {} bytes",
            self.chain_id,
            num_transactions,
            compressed.len()
        );
        Ok(())
    }

    async fn send_batch_to_sequencer(&self, data: Bytes) -> Result<()> {
        debug!("[Chain ID: {}] - Sending batch to sequencer", self.chain_id);
        self.tc_client.process_transaction(data, BATCH_FUNCTION_SIGNATURE.to_string()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Address, Bytes};
    use maestro::redis::producer::StreamProducer;
    use tc_client::config::{TCConfig, TCEndpoint};
    use test_utils::docker::start_redis;
    use tokio::time::Duration;

    const TC_CONFIG: TCConfig = TCConfig {
        tc_endpoint: TCEndpoint::Staging,
        tc_project_id: String::new(),
        tc_api_key: String::new(),
        wallet_pool_address: Address::ZERO,
        sequencing_address: Address::ZERO,
    };

    fn test_config() -> BatcherConfig {
        BatcherConfig {
            max_batch_size: Byte::from_u64(1024),
            redis_url: "dummy".to_string(),
            chain_id: 1,
            polling_interval: Duration::from_millis(10),
            sequencer_client_url: "dummy".to_string(),
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_read_transactions_adds_to_compressor() {
        let mut config = test_config();
        // Start Redis container
        let (_redis, redis_url) = start_redis().await.unwrap();
        config.redis_url = redis_url.clone();

        // Connect to Redis
        let conn = redis::Client::open(redis_url.as_str())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let chain_id = 1;
        let redis_consumer = StreamConsumer::new(conn.clone(), chain_id, "0-0".to_string());
        let producer =
            StreamProducer::new(conn, chain_id, Duration::from_secs(60), Duration::from_secs(60));

        // Send transaction
        let test_data1 = b"test transaction data 1".to_vec();
        producer.enqueue_transaction(test_data1.clone()).await.unwrap();

        let tc_client = TCClient::new(&TC_CONFIG).unwrap();
        let mut batcher = Batcher::new(&config, tc_client, redis_consumer);

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
        // Start Redis container
        let (_redis, redis_url) = start_redis().await.unwrap();

        // Connect to Redis
        let conn = redis::Client::open(redis_url.as_str())
            .unwrap()
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let redis_consumer = StreamConsumer::new(conn, 1, "0-0".to_string());
        let tc_client = TCClient::new(&TC_CONFIG).unwrap();
        let mut batcher = Batcher::new(&config, tc_client, redis_consumer);

        // Insert dummy data
        batcher.compressor.clone_and_compress_with_txn(&Bytes::from(vec![2, 3, 4])).unwrap();

        let result = batcher.send_compressed_batch().await;
        assert!(result.is_err());
    }
}
