//! The `ingestor` module  handles block polling from a remote Ethereum chain and forwards them to a consumer using a channel

use alloy::{
    providers::{Provider, ProviderBuilder, RootProvider},
    transports::BoxTransport,
};
use eyre::eyre;
use std::{error::Error, time::Duration};
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::types::{Block, BlockAndReceipts, Receipt};

/// Polls and ingests blocks from an Ethereum chain
#[derive(Debug)]
pub struct Ingestor {
    chain: RootProvider<BoxTransport>,
    current_block_number: String,
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
        rpc_url: &str,
        start_block: String,
        buffer_size: usize,
        polling_interval: Duration,
    ) -> Result<(Self, Receiver<BlockAndReceipts>), Box<dyn Error>> {
        let chain = ProviderBuilder::new().on_builtin(rpc_url).await?;
        let (sender, receiver) = channel(buffer_size);
        Ok((
            Self {
                chain,
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
    /// The block retrieved from the chain or an error if the block is not found.
    async fn get_block(&self, block_number: String) -> Result<BlockAndReceipts, Box<dyn Error>> {
        let block: Block = self
            .chain
            .client()
            .request::<_, Option<Block>>("eth_getBlockByNumber", (block_number.clone(), true))
            .await?
            // .map(|mut block| block)
            .ok_or_else(|| eyre!("Block not found"))?;

        let receipts = self
            .chain
            .client()
            .request::<_, Option<Vec<Receipt>>>("eth_getBlockReceipts", (block_number.clone(),))
            .await?
            .ok_or_else(|| eyre!("Block receipts not found"))?;

        // let test = self.chain.get_block_receipts(block_id).await?;

        let block_info = BlockAndReceipts {
            block: block,
            receipts: receipts,
        };

        log::info!("[Ingestor] Got block: {:?}", block_info.block.number);
        Ok(block_info)
    }

    /// Sends the retrieved block to the consumer and updates the current block number.
    ///
    /// # Arguments
    /// - `block`: The block to be sent to the consumer.
    ///
    /// # Errors
    /// Returns an error if the block number does not match the expected current block number.
    async fn push_block(&mut self, block_info: BlockAndReceipts) -> Result<(), Box<dyn Error>> {
        if block_info.block.number != self.current_block_number {
            return Err(eyre!("Block number mismatch").into());
        }
        self.sender.send(block_info.clone()).await?;
        let num = hex_to_u128(&self.current_block_number.clone()).expect("Invalid hex value");
        self.current_block_number = format!("0x{:x}", num + 1);
        Ok(())
    }

    /// Starts the polling process.
    ///
    /// Polls for new blocks at the specified interval and sends them to the consumer.
    pub async fn start_polling(&mut self) -> Result<(), Box<dyn Error>> {
        log::info!("[Ingestor] Starting polling");

        let mut interval = tokio::time::interval(self.polling_interval);
        loop {
            let block_info = self.get_block(self.current_block_number.clone()).await?;
            log::info!("[Ingestor] Pushing block: {:?}", block_info.block.number);
            self.push_block(block_info).await?;
            interval.tick().await;
        }
    }
}

fn hex_to_u128(hex: &str) -> Result<u128, std::num::ParseIntError> {
    u128::from_str_radix(hex.trim_start_matches("0x"), 16)
}

fn u128_to_hex(num: u128) -> String {
    format!("0x{:x}", num)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Block, BlockAndReceipts};

    use eyre::Result;

    const RPC_URL: &str = "https://syndicate.io";

    fn get_dummy_block_info(block_number: String) -> BlockAndReceipts {
        let block: Block = Block {
            hash: "0xHash".to_string(),
            nonce: "0xNon".to_string(),
            number: block_number,
            parent_hash: "0xPar".to_string(),
            logs_bloom: "0xLog".to_string(),
            transactions_root: "0xTra".to_string(),
            state_root: "0xSta".to_string(),
            receipts_root: "0xRec".to_string(),
            timestamp: "0xTim".to_string(),
            transactions: vec![],
        };
        let block_info = BlockAndReceipts {
            block: block,
            receipts: vec![],
        };
        block_info
    }

    #[tokio::test]
    async fn test_ingestor_new() -> Result<(), Box<dyn Error>> {
        let start_block = "0x12958cb".to_string();
        let buffer_size = 10;
        let polling_interval = Duration::from_secs(1);

        let (ingestor, receiver) =
            Ingestor::new(RPC_URL, start_block.clone(), buffer_size, polling_interval).await?;

        assert_eq!(ingestor.current_block_number, start_block.clone());
        assert_eq!(receiver.capacity(), buffer_size);
        Ok(())
    }

    #[tokio::test]
    async fn test_poll_block() -> Result<(), Box<dyn Error>> {
        let start_block = "0x12958cb".to_string();
        let polling_interval = Duration::from_secs(1);

        let (sender, mut receiver) = channel(10);
        let mut ingestor = Ingestor {
            chain: ProviderBuilder::new().on_builtin(RPC_URL).await?,
            current_block_number: start_block.clone(),
            sender,
            polling_interval,
        };

        let block = get_dummy_block_info(start_block.clone());

        ingestor
            .push_block(block)
            .await
            .expect("Failed to poll block");

        if let Some(BlockAndReceipts { block, .. }) = receiver.recv().await {
            assert_eq!(block.number, start_block.to_string());
        } else {
            panic!("No block received");
        }

        let next_block =
            u128_to_hex(hex_to_u128(start_block.as_str()).expect("Invalid hex value") + 1);

        assert_eq!(ingestor.current_block_number, next_block);
        Ok(())
    }

    #[tokio::test]
    async fn test_poll_block_mismatch_error() -> Result<(), Box<dyn Error>> {
        let start_block = "0x12958cb".to_string();
        let polling_interval = Duration::from_secs(1);

        let (sender, _) = channel(10);
        let mut ingestor = Ingestor {
            chain: ProviderBuilder::new().on_builtin(RPC_URL).await?,
            current_block_number: start_block.clone(),
            sender,
            polling_interval,
        };

        let mismatched_block = get_dummy_block_info(u128_to_hex(
            hex_to_u128(start_block.as_str()).expect("Invalid hex value") + 10,
        ));
        let result = ingestor.push_block(mismatched_block).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Block number mismatch");
        Ok(())
    }
}
