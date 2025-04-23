//! The Batcher service for the sequencer.

use crate::config::BatcherConfig;
use alloy::primitives::Bytes;
use eyre::{eyre, Result};
use shared::additive_compression::AdditiveCompressor;
use std::{mem::take, sync::Arc};
use tc_client::tc_client::{TCClient, BATCH_FUNCTION_SIGNATURE};
use tokio::sync::Mutex;
use tracing::{debug, error, info};
// TODO (SEQ-772): Redis interface
#[allow(missing_docs)]
#[derive(Debug, Clone)]

pub struct StreamManager {}
impl StreamManager {
    async fn new(_stream_key: String, _chain_id: u64) -> Result<Self> {
        Ok(Self {})
    }
    async fn recv(&self, _max_msg_count: usize) -> Result<Vec<(Vec<u8>, String)>> {
        let data = vec![0u8; 512]; // 512 bytes of zeros
        Ok(vec![(data, String::new())])
    }
}

/// Batcher service
#[derive(Debug)]
struct Batcher {
    /// The max batch size for the batcher
    max_batch_size: usize,
    /// The Redis client for the batcher
    redis_client: StreamManager,
    /// The sequencer client for the batcher
    tc_client: TCClient,
    /// The compressor for the batcher
    compressor: AdditiveCompressor,
}

/// Run the batcher service. Starts the server and listens for batch requests.
pub async fn run_batcher(config: &BatcherConfig, tc_client: TCClient) -> Result<()> {
    let redis_client = StreamManager::new(config.redis_url.clone(), config.chain_id).await?;
    let batcher = Arc::new(Mutex::new(Batcher::new(config, tc_client, redis_client)));
    let polling_interval = config.polling_interval;

    tokio::spawn({
        let batcher = Arc::clone(&batcher);

        async move {
            loop {
                let result = {
                    let mut batcher = batcher.lock().await;
                    batcher.read_and_batch_transactions().await
                };
                if let Err(e) = result {
                    error!("Batcher error: {:?}", e);
                }
                tokio::time::sleep(polling_interval).await;
            }
        }
    });
    info!("Batcher job started with {:?} poll interval", config.polling_interval);
    Ok(())
}

impl Batcher {
    fn new(config: &BatcherConfig, tc_client: TCClient, redis_client: StreamManager) -> Self {
        Self {
            max_batch_size: config.max_batch_size,
            redis_client,
            tc_client,
            compressor: AdditiveCompressor::default(),
        }
    }
    async fn read_transactions(&mut self) -> Result<()> {
        loop {
            // TODO: Configurable max msg count
            let transactions = self.redis_client.recv(1).await?;
            if transactions.is_empty() {
                break;
            }

            for (txn_bytes, _) in transactions {
                let txn = Bytes::from(txn_bytes);

                let size_if_added = self.compressor.peek_push_size(&txn)?;

                if size_if_added >= self.max_batch_size {
                    return Ok(());
                }
                debug!(
                    "Adding transaction to batch: {:?} - compressed size: {}",
                    txn, size_if_added
                );
                self.compressor.try_push(&txn)?;
            }
        }
        Ok(())
    }

    async fn read_and_batch_transactions(&mut self) -> Result<()> {
        self.read_transactions().await?;
        if self.compressor.is_empty() {
            debug!("No transactions available to batch.");
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
        let redis_client = StreamManager::new("dummy".into(), 1).await.unwrap();
        let tc_client = TCClient::new(&TC_CONFIG).unwrap();
        let mut batcher = Batcher::new(&config, tc_client, redis_client);

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
        let redis_client = StreamManager::new("dummy".into(), 1).await.unwrap();
        let tc_client = TCClient::new(&TC_CONFIG).unwrap();
        let mut batcher = Batcher::new(&config, tc_client, redis_client);

        // Insert dummy data
        batcher.compressor.try_push(&Bytes::from(vec![2, 3, 4])).unwrap();

        let result = batcher.send_compressed_batch().await;
        assert!(result.is_err());
    }
}
