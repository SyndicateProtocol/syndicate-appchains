//! The `ingestor` module  handles block polling from a remote Ethereum chain and forwards them to a
//! consumer using a channel

use crate::{config::ChainIngestorConfig, eth_client::EthClient};
use common::types::BlockAndReceipts;
use eyre::{eyre, Error};
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tracing::{debug, error};

/// Polls and ingests blocks from an Ethereum chain
#[derive(Debug)]
pub struct Ingestor {
    client: EthClient,
    current_block_number: u64,
    sender: Sender<BlockAndReceipts>,
    polling_interval: Duration,
}

impl Ingestor {
    /// Creates a new `Ingestor` instance.
    ///
    /// # Arguments
    /// - `rpc_url`: The RPC endpoint URL of the Ethereum chain.
    /// - `start_block`: The block number to start polling from.
    /// - `buffer_size`: The size of the channel buffer for blocks.
    /// - `polling_interval`: The time interval between each block polling.
    ///
    /// # Returns
    /// A tuple containing the `Ingestor` instance and a `Receiver` for consuming blocks.
    pub async fn new(
        config: ChainIngestorConfig,
    ) -> Result<(Self, Receiver<BlockAndReceipts>), Error> {
        let client = EthClient::new(&config.rpc_url).await?;
        let (sender, receiver) = channel(config.buffer_size);
        Ok((
            Self {
                client,
                current_block_number: config.start_block,
                sender,
                polling_interval: config.polling_interval(),
            },
            receiver,
        ))
    }

    /// Retrieves a block by its number.
    ///
    /// # Arguments
    /// - `block_number`: The block number to retrieve.
    ///
    /// # Returns
    /// The block and its receipts.
    async fn get_block_and_receipts(&self, block_number: u64) -> Result<BlockAndReceipts, Error> {
        let block = self.client.get_block_by_number(block_number).await?;
        let receipts = self.client.get_block_receipts(block_number).await?;
        debug!("Got block: {:?}", block.number);

        Ok(BlockAndReceipts { block, receipts })
    }

    /// Sends the retrieved block to the consumer and updates the current block number.
    ///
    /// # Arguments
    /// - `block_and_receipts`: The block and its receipts to be sent to the consumer.
    async fn push_block_and_receipts(
        &mut self,
        block_and_receipts: BlockAndReceipts,
    ) -> Result<(), Error> {
        if block_and_receipts.block.number != self.current_block_number {
            return Err(eyre!("Block number mismatch"));
        }
        self.sender.send(block_and_receipts.clone()).await?;
        self.current_block_number += 1;
        Ok(())
    }

    /// Starts the polling process.
    ///
    /// Polls for new blocks and receipts at the specified interval and sends them to the consumer.
    pub async fn start_polling(&mut self) -> Result<(), Error> {
        debug!("Starting polling");

        let mut interval = tokio::time::interval(self.polling_interval);

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    match self.get_block_and_receipts(self.current_block_number).await {
                        Ok(block_and_receipts) => {
                            debug!("Pushing block: {:?}", block_and_receipts.block.number);
                            if let Err(err) = self.push_block_and_receipts(block_and_receipts).await {
                                error!("Failed to push block and receipts: {:?}, retrying...", err);
                            }
                        }
                        Err(err) => {
                            error!("Failed to fetch block and receipts: {:?}, retrying...", err);
                        }
                    }
                }
                // Respond to cancellation
                _ = tokio::task::yield_now() => {
                    debug!("Polling task was cancelled");
                    break;
                }
            }
        }

        debug!("Polling stopped");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ChainIngestorConfig, IngestionPipelineConfig};
    use alloy::primitives::B256;
    use common::types::{Block, BlockAndReceipts};
    use eyre::Result;
    use std::str::FromStr;

    fn test_config() -> IngestionPipelineConfig {
        IngestionPipelineConfig {
            sequencing: ChainIngestorConfig {
                buffer_size: 100,
                polling_interval_secs: 1,
                rpc_url: "https://sequencing.io".into(),
                start_block: 19486923,
            }
            .into(),
            settlement: ChainIngestorConfig {
                buffer_size: 100,
                polling_interval_secs: 1,
                rpc_url: "https://settlement.io".into(),
                start_block: 19486923,
            }
            .into(),
        }
    }

    fn get_dummy_block_and_receipts(number: u64) -> BlockAndReceipts {
        let block: Block = Block {
            hash: B256::from_str(
                "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            number,
            parent_hash: B256::from_str(
                "0234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
            )
            .unwrap(),
            logs_bloom: "0xLog".to_string(),
            transactions_root: "0xTra".to_string(),
            state_root: "0xSta".to_string(),
            receipts_root: "0xRec".to_string(),
            timestamp: 1000000000,
            transactions: vec![],
        };
        BlockAndReceipts { block, receipts: vec![] }
    }

    #[tokio::test]
    async fn test_ingestor_new() -> Result<(), Error> {
        let start_block = 19486923;
        let buffer_size = 100;
        let polling_interval = Duration::from_secs(1);
        let config = test_config();

        let (ingestor, receiver) = Ingestor::new(config.sequencing.into()).await?;

        assert_eq!(ingestor.current_block_number, start_block);
        assert_eq!(receiver.capacity(), buffer_size);
        assert_eq!(ingestor.polling_interval, polling_interval);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_block_and_receipts() -> Result<(), Error> {
        let start_block = 19486923;
        let polling_interval = Duration::from_secs(1);

        let (sender, mut receiver) = channel(10);
        let client = EthClient::new(test_config().sequencing.sequencing_rpc_url.as_str()).await?;
        let mut ingestor =
            Ingestor { client, current_block_number: start_block, sender, polling_interval };

        let block = get_dummy_block_and_receipts(start_block);

        ingestor.push_block_and_receipts(block).await.expect("Failed to poll block");

        if let Some(BlockAndReceipts { block, .. }) = receiver.recv().await {
            assert_eq!(block.number, start_block);
        } else {
            panic!("No block received");
        }
        Ok(())
    }
    #[tokio::test]
    async fn test_start_polling_simple() -> Result<(), Error> {
        let start_block = 1;
        let polling_interval = Duration::from_millis(10);

        // Create a simple channel for testing.
        let (sender, _) = channel(10);
        let client = EthClient::new(test_config().sequencing.sequencing_rpc_url.as_str()).await?;

        let mut ingestor =
            Ingestor { client, current_block_number: start_block, sender, polling_interval };

        // Spawn the polling task.
        let polling_handle = tokio::spawn(async move {
            let result = ingestor.start_polling().await;
            assert!(result.is_ok(), "Polling task failed: {:?}", result);
        });

        tokio::time::sleep(Duration::from_millis(50)).await;

        polling_handle.abort();

        match polling_handle.await {
            Err(err) => {
                panic!("Unexpected error when waiting on the polling task: {:?}", err);
            }
            Ok(_) => {
                debug!("Polling task was successfully aborted.");
            }
        }

        Ok(())
    }
}
