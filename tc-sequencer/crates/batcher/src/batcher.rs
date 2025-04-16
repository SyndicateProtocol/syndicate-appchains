//! The Batcher service for the Maestro.

use crate::config::BatcherConfig;
use alloy::primitives::Bytes;
use eyre::{eyre, Error, Result};
use shared::zlib_compression::compress_transactions;
use std::{sync::Arc, time::Duration};
use tc_client::tc_client::TCClient;
use tracing::{error, info};
// Implemented in Daniils PR
#[allow(missing_docs)]
#[derive(Debug, Clone)]

pub struct StreamManager {}
impl StreamManager {
    async fn read_next_batch(
        &self,
        _key: String,
        _batch_size: usize,
    ) -> Result<Option<Vec<Bytes>>> {
        Ok(Some(vec![])) // dummy for now
    }
}

/// Batcher service
#[derive(Debug, Clone)]
struct Batcher {
    /// The transactions in batch for the batcher
    transactions_in_batch: usize,
    /// The max batch size for the batcher
    max_batch_size: usize,
    /// The Redis client for the batcher
    redis_client: StreamManager,
    /// The stream key for the batcher
    stream_key: String,
    /// The sequencer client for the batcher
    tc_client: TCClient,
    /// The polling interval for the batcher
    polling_interval: Duration,
    /// The chain ID for the batcher
    chain_id: u64,
}

/// Run the batcher service. Starts the server and listens for batch requests.
pub async fn run_batcher(config: &BatcherConfig, tc_client: TCClient) -> Result<()> {
    let batcher = Arc::new(Batcher::new(config, tc_client));

    tokio::spawn({
        let batcher = Arc::clone(&batcher);

        async move {
            loop {
                if let Err(e) = batcher.read_and_batch_transactions().await {
                    panic!("Error while reading and batching transactions: {:?}", e);
                }
                tokio::time::sleep(batcher.polling_interval).await;
            }
        }
    });
    info!("Batcher job started with {:?} poll interval", config.polling_interval);
    Ok(())
}

impl Batcher {
    /// Create a new instance of the Maestro service
    fn new(config: &BatcherConfig, tc_client: TCClient) -> Self {
        let redis_client = StreamManager {};
        Self {
            transactions_in_batch: config.transactions_in_batch,
            max_batch_size: config.max_batch_size,
            redis_client,
            stream_key: "txs:".to_string(),
            tc_client,
            polling_interval: config.polling_interval,
            chain_id: config.chain_id,
        }
    }
    async fn read_and_batch_transactions(&self) -> Result<(), Error> {
        let stream_group = format!("{}{}", self.stream_key, self.chain_id);
        match self.redis_client.read_next_batch(stream_group, self.transactions_in_batch).await {
            Ok(Some(transactions)) => {
                info!("Batch read successfully");
                let batch = self.batch_transactions(transactions);
                if let Err(e) = self.compress_and_send_batch(batch.clone()).await {
                    error!("Error sending batch: {}", e);
                    return Err(e);
                }
                Ok(())
            }
            Ok(None) => {
                info!("No batch found");
                Ok(())
            }
            Err(e) => {
                error!("Error reading batch from Redis: {}", e);
                Err(eyre!("failed to read batch"))
            }
        }
    }

    fn batch_transactions(&self, transactions: Vec<Bytes>) -> Vec<Bytes> {
        transactions.into_iter().take(self.transactions_in_batch).collect()
    }

    async fn compress_and_send_batch(&self, batch: Vec<Bytes>) -> Result<()> {
        let compressed_batch = compress_transactions(&batch)?;

        // Check if the batch is too large
        if compressed_batch.len() > self.max_batch_size {
            error!(
                "Compressed batch size ({}) exceeds maximum allowed ({})",
                compressed_batch.len(),
                self.max_batch_size
            );
            return Err(eyre!("batch is too large"));
        }

        // Send the batch to the sequencer
        match self.send_batch_to_sequencer(compressed_batch.clone()).await {
            Ok(_) => {
                info!(
                    "Successfully sent batch of {} txs ({} bytes)",
                    batch.len(),
                    compressed_batch.len()
                );
                Ok(())
            }
            Err(e) => {
                error!("Failed to send batch to sequencer: {:?}", e);
                Err(eyre!("Sending batch failed: {:?}", e))
            }
        }
    }

    async fn send_batch_to_sequencer(&self, data: Bytes) -> Result<()> {
        self.tc_client.process_transaction(data).await?;
        Ok(())
    }
}
