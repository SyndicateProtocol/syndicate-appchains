//! The Batcher service for the Maestro.

use crate::config::BatcherConfig;
use alloy::primitives::Bytes;
use eyre::{eyre, Result};
use shared::zlib_compression::compress_transactions;
use std::{sync::Arc, time::Duration};
use tc_client::tc_client::{TCClient, BATCH_FUNCTION_SIGNATURE};
use tracing::{debug, error, info};
// TODO (SEQ-772): Redis interface
#[allow(missing_docs)]
#[derive(Debug, Clone)]

pub struct StreamManager {}
impl StreamManager {
    async fn new(_stream_key: String, _chain_id: u64) -> Result<Self> {
        Ok(Self {})
    }
    async fn recv(&self) -> Result<Option<Bytes>> {
        let data = vec![0u8; 512]; // 512 bytes of zeros
        Ok(Some(Bytes::from(data)))
    }
}

/// Batcher service
#[derive(Debug, Clone)]
struct Batcher {
    /// The max batch size for the batcher
    max_batch_size: usize,
    /// The Redis client for the batcher
    redis_client: StreamManager,
    /// The sequencer client for the batcher
    tc_client: TCClient,
    /// The polling interval for the batcher
    polling_interval: Duration,
}

/// Run the batcher service. Starts the server and listens for batch requests.
pub async fn run_batcher(config: &BatcherConfig, tc_client: TCClient) -> Result<()> {
    // TODO (SEQ-772): Connect to real Redis interface
    let redis_client = StreamManager::new(config.redis_url.clone(), config.chain_id).await?;
    let batcher = Arc::new(Batcher::new(config, tc_client, redis_client));
    let polling_interval = batcher.polling_interval;

    tokio::spawn({
        let batcher = Arc::clone(&batcher);

        async move {
            loop {
                if let Err(e) = batcher.read_and_batch_transactions().await {
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
    /// Create a new instance of the Maestro service
    const fn new(config: &BatcherConfig, tc_client: TCClient, redis_client: StreamManager) -> Self {
        Self {
            max_batch_size: config.max_batch_size,
            redis_client,
            tc_client,
            polling_interval: config.polling_interval,
        }
    }
    async fn read_transactions(&self) -> Result<Vec<Bytes>> {
        let mut batch: Vec<Bytes> = Vec::new();
        info!("Reading transactions from Redis");

        while let Some(txn) = self.redis_client.recv().await? {
            {
                // Estimate size *before* pushing to the real batch
                let compressed_size_if_added = {
                    let mut candidate = batch.clone();
                    candidate.push(txn.clone());
                    compress_transactions(&candidate)?.len()
                };

                if compressed_size_if_added >= self.max_batch_size {
                    break;
                }
                info!(
                    "Adding transaction to batch: {:?} - size: {}",
                    txn, compressed_size_if_added
                );
                batch.push(txn);
            }
        }

        Ok(batch)
    }

    async fn read_and_batch_transactions(&self) -> Result<()> {
        let batch = self.read_transactions().await?;
        if batch.is_empty() {
            debug!("No transactions available to batch.");
            return Ok(());
        }
        info!("Batching {} transactions", batch.len());

        self.compress_and_send_batch(batch.clone()).await
    }

    async fn compress_and_send_batch(&self, batch: Vec<Bytes>) -> Result<()> {
        let compressed = compress_transactions(&batch)?;

        // Check if the batch is too large
        if compressed.len() > self.max_batch_size {
            error!(
                "Compressed batch size ({}) exceeds limit ({})",
                compressed.len(),
                self.max_batch_size
            );
            return Err(eyre!(
                "Compressed batch size {} exceeds limit {}",
                compressed.len(),
                self.max_batch_size
            ));
        }

        self.send_batch_to_sequencer(compressed.clone()).await?;
        info!("Batch sent: {} txs, compressed size: {} bytes", batch.len(), compressed.len());
        Ok(())
    }

    async fn send_batch_to_sequencer(&self, data: Bytes) -> Result<()> {
        self.tc_client
            .process_transaction(data, Some(BATCH_FUNCTION_SIGNATURE.to_string()))
            .await?;
        Ok(())
    }
}
