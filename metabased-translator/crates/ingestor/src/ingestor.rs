//! The `ingestor` module  handles block polling from a remote Ethereum chain and forwards them to a consumer using a channel

use eyre::{eyre, Error};
use log::info;
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::eth_client::EthClient;
use common::types::BlockAndReceipts;

/// Polls and ingests blocks from an Ethereum chain
#[derive(Debug)]
#[allow(unreachable_pub)] // TODO: remove when used
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
    #[allow(unreachable_pub)] // TODO: remove when used
    pub async fn new(
        rpc_url: &str,
        start_block: u64,
        buffer_size: usize,
        polling_interval: Duration,
    ) -> Result<(Self, Receiver<BlockAndReceipts>), Error> {
        let client = EthClient::new(rpc_url).await?;
        let (sender, receiver) = channel(buffer_size);
        Ok((
            Self {
                client,
                current_block_number: start_block,
                sender,
                polling_interval,
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
        info!("Got block: {:?}", block.number);

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
    #[allow(unreachable_pub)] // TODO: remove when used
    pub async fn start_polling(&mut self) -> Result<(), Error> {
        info!("Starting polling");

        let mut interval = tokio::time::interval(self.polling_interval);
        loop {
            let block_and_receipts = self
                .get_block_and_receipts(self.current_block_number)
                .await?;
            info!("Pushing block: {:?}", block_and_receipts.block.number);
            self.push_block_and_receipts(block_and_receipts).await?;
            interval.tick().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::types::{Block, BlockAndReceipts};

    use eyre::Result;

    const RPC_URL: &str = "https://syndicate.io";

    fn get_dummy_block_and_receipts(block_number: u64) -> BlockAndReceipts {
        let block: Block = Block {
            hash: "0xHash".to_string(),
            number: block_number,
            parent_hash: "0xPar".to_string(),
            logs_bloom: "0xLog".to_string(),
            transactions_root: "0xTra".to_string(),
            state_root: "0xSta".to_string(),
            receipts_root: "0xRec".to_string(),
            timestamp: 1000000000,
            transactions: vec![],
        };
        BlockAndReceipts {
            block,
            receipts: vec![],
        }
    }

    #[tokio::test]
    async fn test_ingestor_new() -> Result<(), Error> {
        let start_block = 19486923;
        let buffer_size = 10;
        let polling_interval = Duration::from_secs(1);

        let (ingestor, receiver) =
            Ingestor::new(RPC_URL, start_block, buffer_size, polling_interval).await?;

        assert_eq!(ingestor.current_block_number, start_block);
        assert_eq!(receiver.capacity(), buffer_size);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_block_and_receipts() -> Result<(), Error> {
        let start_block = 19486923;
        let polling_interval = Duration::from_secs(1);

        let (sender, mut receiver) = channel(10);
        let client = EthClient::new(RPC_URL).await?;
        let mut ingestor = Ingestor {
            client,
            current_block_number: start_block,
            sender,
            polling_interval,
        };

        let block = get_dummy_block_and_receipts(start_block);

        ingestor
            .push_block_and_receipts(block)
            .await
            .expect("Failed to poll block");

        if let Some(BlockAndReceipts { block, .. }) = receiver.recv().await {
            assert_eq!(block.number, start_block);
        } else {
            panic!("No block received");
        }
        Ok(())
    }
}
