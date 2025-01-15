//! Block builder service for processing and building L3 blocks.

use crate::config::BlockBuilderConfig;
use crate::connectors::anvil::MetaChainProvider;
use crate::rollups::{
    arbitrum::arbitrum_builder::ArbitrumBlockBuilder, rollup_builder::RollupBlockBuilder,
    utils::sequencing_chain_blocks_to_mbtxs,
};
use eyre::{Error, Result};
use tokio::sync::mpsc::Receiver as TokioReceiver;

/// Block builder service for processing and building L3 blocks.
#[derive(Debug)]
pub struct BlockBuilder {
    slotter_receiver: TokioReceiver<String>,

    mchain: MetaChainProvider,
    builder: Box<dyn RollupBlockBuilder>,
}

impl BlockBuilder {
    /// Create a new block builder
    pub async fn new(
        slotter_receiver: TokioReceiver<String>,
        config: BlockBuilderConfig,
    ) -> Result<Self, Error> {
        let mchain = MetaChainProvider::start(config).await?;

        // TODO: Dynamically select builder based on config
        let builder = Box::new(ArbitrumBlockBuilder::new());

        Ok(Self {
            slotter_receiver,
            mchain,
            builder,
        })
    }

    /// Start the block builder
    pub async fn start(mut self) -> Result<()> {
        tokio::spawn(async move {
            while let Some(slot) = self.slotter_receiver.recv().await {
                println!("Received slot: {:?}", slot);

                // Process sequencing chain blocks into mB transactions
                let mbtxs = sequencing_chain_blocks_to_mbtxs(vec![slot]);

                // TODO: [OP / ARB] Process deposit transactions

                // [OP / ARB] Build batch
                let batch_txn = self.builder.build_batch_txn(mbtxs).await.unwrap();

                // Submit batch transaction to mchain
                let _ = self.mchain.submit_txn(batch_txn).await;

                // Mine mchain block
                let _ = self.mchain.mine_block().await;
            }
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_block_builder_new() -> Result<()> {
        let (_tx, rx) = mpsc::channel(32);
        let config = BlockBuilderConfig::default();
        let builder = BlockBuilder::new(rx, config).await?;

        assert!(builder.slotter_receiver.capacity() >= 32);
        Ok(())
    }

    #[tokio::test]
    async fn test_block_builder_start() -> Result<()> {
        let (tx, rx) = mpsc::channel(32);
        let config = BlockBuilderConfig::default();

        let builder = BlockBuilder::new(rx, config).await?;

        // Start the block builder in a separate task
        let handle = tokio::spawn(async move { builder.start().await });

        // Send a test block
        let test_slot = "test_slot".to_string();
        tx.send(test_slot).await?;

        // Give some time for processing
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Clean shutdown
        handle.abort();
        Ok(())
    }

    #[tokio::test]
    async fn test_block_builder_handles_channel_close() -> Result<()> {
        let (tx, rx) = mpsc::channel(32);
        let config = BlockBuilderConfig::default();
        let builder = BlockBuilder::new(rx, config).await?;

        let handle = tokio::spawn(async move { builder.start().await });

        // Drop sender to simulate channel close
        drop(tx);

        // Allow time for processing
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        handle.abort();
        Ok(())
    }
}
