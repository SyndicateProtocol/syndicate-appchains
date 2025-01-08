//! The `ingestor` module  handles block polling from a remote Ethereum chain and forwards them to a consumer using a channel

use alloy::{
    eips::eip1898::BlockNumberOrTag,
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::{Block, BlockTransactionsKind},
    transports::BoxTransport,
};
use eyre::{eyre, Error};
use std::time::Duration;
use tokio::sync::mpsc;

/// Polls and ingests blocks from an Ethereum chain
#[derive(Debug)]
pub struct Ingestor {
    chain: RootProvider<BoxTransport>,
    current_block_number: u64,
    sender: mpsc::Sender<Block>,
    polling_interval: Duration,
}

impl Ingestor {
    /// Creates a new ingestor
    pub async fn new(
        rpc_url: &str,
        start_block: u64,
        buffer_size: usize,
        polling_interval: Duration,
    ) -> Result<(Self, mpsc::Receiver<Block>), Error> {
        let chain = ProviderBuilder::new().on_builtin(rpc_url).await?;
        let (sender, receiver) = mpsc::channel(buffer_size);
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

    async fn get_block(&self, block_number: u64) -> Result<Block, Error> {
        let block = self
            .chain
            .get_block_by_number(
                BlockNumberOrTag::from(block_number),
                BlockTransactionsKind::Full,
            )
            .await?
            .ok_or_else(|| eyre!("Block not found"))?;
        Ok(block)
    }

    async fn push_block(&mut self, block: Block) -> Result<(), Error> {
        if block.header.inner.number != self.current_block_number {
            return Err(eyre!("Block number mismatch"));
        }
        self.sender.send(block.clone()).await?;
        self.current_block_number += 1;
        Ok(())
    }

    /// Starts polling for blocks
    pub async fn start_polling(&mut self) -> Result<(), Error> {
        let mut interval = tokio::time::interval(self.polling_interval);
        loop {
            let block = self.get_block(self.current_block_number).await?;
            self.push_block(block).await?;
            interval.tick().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::{B256, U256},
        rpc::types::{Block, BlockTransactions, Header},
    };
    use eyre::Result;
    use tokio::sync::mpsc;

    const RPC_URL: &str = "https://syndicate.io";

    fn get_dummy_block(block_number: u64) -> Block {
        let mut block: Block = Block {
            header: Header {
                hash: B256::with_last_byte(block_number as u8),
                inner: Default::default(),
                total_difficulty: Some(U256::from(block_number * 1000)),
                size: Some(U256::from(1000)),
            },
            uncles: vec![B256::with_last_byte((block_number - 1) as u8)],
            transactions: BlockTransactions::Full(vec![]),
            withdrawals: None,
        };
        block.header.inner.number = block_number;
        block
    }

    #[tokio::test]
    async fn test_ingestor_new() -> Result<()> {
        let start_block = 100;
        let buffer_size = 10;
        let polling_interval = Duration::from_secs(1);

        let (ingestor, receiver) =
            Ingestor::new(RPC_URL, start_block, buffer_size, polling_interval).await?;

        assert_eq!(ingestor.current_block_number, start_block);
        assert_eq!(receiver.capacity(), buffer_size);

        Ok(())
    }

    #[tokio::test]
    async fn test_poll_block() -> Result<()> {
        let start_block = 100;
        let polling_interval = Duration::from_secs(1);

        let (sender, mut receiver) = mpsc::channel(10);
        let mut ingestor = Ingestor {
            chain: ProviderBuilder::new().on_builtin(RPC_URL).await?,
            current_block_number: start_block,
            sender,
            polling_interval,
        };

        let block = get_dummy_block(start_block);

        ingestor
            .push_block(block)
            .await
            .expect("Failed to poll block");

        if let Some(block) = receiver.recv().await {
            println!("Received block number: {:?}", block.header.inner.number);
            assert_eq!(block.header.inner.number, start_block);
        } else {
            panic!("No block received");
        }

        assert_eq!(ingestor.current_block_number, start_block + 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_poll_block_mismatch_error() -> Result<()> {
        let start_block = 100;
        let polling_interval = Duration::from_secs(1);

        let (sender, _) = mpsc::channel(10);
        let mut ingestor = Ingestor {
            chain: ProviderBuilder::new().on_builtin(RPC_URL).await?,
            current_block_number: start_block,
            sender,
            polling_interval,
        };

        let mismatched_block = get_dummy_block(start_block + 1);
        let result = ingestor.push_block(mismatched_block).await;

        assert!(result.is_err());
        assert_eq!(result.err().unwrap().to_string(), "Block number mismatch");

        Ok(())
    }
}
