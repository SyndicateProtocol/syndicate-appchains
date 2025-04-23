//! The Batcher service for the sequencer.

use crate::config::BatcherConfig;
use alloy::primitives::Bytes;
use eyre::{eyre, Result};
use maestro::redis::consumer::StreamConsumer;
use redis::Client as RedisClient;
use shared::additive_compression::AdditiveCompressor;
use std::mem::take;
use tc_client::tc_client::{TCClient, BATCH_FUNCTION_SIGNATURE};
use tokio::task::JoinHandle;
use tracing::{debug, error, info};
/// Batcher service
#[derive(Debug)]
struct Batcher {
    /// The max batch size for the batcher
    max_batch_size: usize,
    /// The Redis consumer for the batcher
    redis_consumer: StreamConsumer,
    /// The sequencer client for the batcher
    tc_client: TCClient,
    /// The compressor for the batcher
    compressor: AdditiveCompressor,
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
        }
    }
    async fn read_transactions(&mut self) -> Result<()> {
        loop {
            // TODO: Configurable max msg count
            let transactions = self.redis_consumer.recv(1).await?;

            // This should never happen. Just a sanity check to catch it.
            if transactions.is_empty() {
                error!("No transactions available to batch.");
                break;
            }

            for (txn_bytes, _) in transactions {
                let txn = Bytes::from(txn_bytes);

                let compressed = self.compressor.clone_and_compress_with_txn(&txn)?;

                if compressed.len() >= self.max_batch_size {
                    // Stop consuming and let the caller finalize the batch
                    return Ok(());
                }
                debug!(
                    "Adding transaction to batch: {:?} - compressed size: {}",
                    txn,
                    compressed.len()
                );
                self.compressor.push_with_precompressed(&txn, compressed)?;
            }
        }
        Ok(())
    }

    async fn read_and_batch_transactions(&mut self) -> Result<()> {
        self.read_transactions().await?;

        // This should never happen. Just a sanity check to catch it.
        if self.compressor.is_empty() {
            error!("No transactions available to batch.");
            return Ok(());
        }

        self.send_compressed_batch().await
    }

    async fn send_compressed_batch(&mut self) -> Result<()> {
        let num_transactions = self.compressor.num_transactions();
        let compressed = take(&mut self.compressor).finish()?;
        self.compressor = AdditiveCompressor::default(); // Reset for next round

        if compressed.len() > self.max_batch_size {
            let error_msg = format!(
                "Compressed batch size ({}) exceeds limit ({})",
                compressed.len(),
                self.max_batch_size
            );
            error!(error_msg);
            return Err(eyre!(error_msg));
        }

        self.send_batch_to_sequencer(compressed.clone()).await?;
        debug!("Batch sent: {} txs, compressed size: {} bytes", num_transactions, compressed.len());
        Ok(())
    }

    async fn send_batch_to_sequencer(&self, data: Bytes) -> Result<()> {
        debug!("Sending batch to sequencer");
        self.tc_client
            .process_transaction(data, Some(BATCH_FUNCTION_SIGNATURE.to_string()))
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Address, Bytes};
    use tc_client::config::{TCConfig, TCEndpoint};
    use test_utils::docker::start_redis;

    const TC_CONFIG: TCConfig = TCConfig {
        tc_endpoint: TCEndpoint::Staging,
        tc_project_id: String::new(),
        tc_api_key: String::new(),
        wallet_pool_address: Address::ZERO,
        sequencing_address: Address::ZERO,
    };

    fn test_config() -> BatcherConfig {
        BatcherConfig {
            max_batch_size: 1024,
            redis_url: "dummy".to_string(),
            chain_id: 1,
            polling_interval: std::time::Duration::from_millis(10),
            sequencer_client_url: "dummy".to_string(),
        }
    }

    #[tokio::test]
    async fn test_read_transactions_adds_to_compressor() {
        let config = test_config();
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

        let result = batcher.read_transactions().await;
        assert!(result.is_ok());
        assert!(!batcher.compressor.is_empty(), "Compressor should not be empty after read");
    }

    #[tokio::test]
    async fn test_send_compressed_batch_returns_error_if_too_large() {
        let config = BatcherConfig {
            max_batch_size: 1, // force failure
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
