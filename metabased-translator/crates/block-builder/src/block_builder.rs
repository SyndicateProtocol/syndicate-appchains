//! Block builder service for processing and building L3 blocks.

use crate::{
    config::BlockBuilderConfig,
    connectors::anvil::MetaChainProvider,
    rollups::{arbitrum::arbitrum_builder::ArbitrumBlockBuilder, shared::RollupBlockBuilder},
};
use alloy::transports::{RpcError, TransportErrorKind};
use common::types::Slot;
use eyre::{Error, Result};
use thiserror::Error;
use tokio::sync::mpsc::Receiver;
use tracing::{error as log_error, info};

/// Block builder service for processing and building L3 blocks.
#[derive(Debug)]
pub struct BlockBuilder {
    slotter_rx: Receiver<Slot>,

    mchain: MetaChainProvider,
    builder: Box<dyn RollupBlockBuilder>,
}

impl BlockBuilder {
    /// Create a new block builder
    pub async fn new(
        slotter_rx: Receiver<Slot>,
        config: BlockBuilderConfig,
    ) -> Result<Self, Error> {
        let mchain = MetaChainProvider::start(config.clone()).await?;

        // TODO (SEQ-515): Dynamically select builder based on config
        let builder = Box::new(ArbitrumBlockBuilder::new(config.sequencing_contract_address));

        Ok(Self { slotter_rx, mchain, builder })
    }

    /// Start the block builder
    pub async fn start(mut self) {
        while let Some(slot) = self.slotter_rx.recv().await {
            info!("Received slot: {:?}", slot);

            // Process sequencing chain blocks into mB transactions
            let mbtxs = self.builder.parse_blocks_to_mbtxs(slot.sequencing_chain_blocks);

            // TODO (SEQ-416): [OP / ARB] Process deposit transactions

            // [OP / ARB] Build and submit batch
            let batch_txn = match self.builder.build_batch_txn(mbtxs).await {
                Ok(txn) => txn,
                Err(e) => {
                    log_error!("Error building batch transaction: {}", e);
                    continue;
                }
            };

            // Submit batch transaction to mchain
            if let Err(e) = self.mchain.submit_txn(batch_txn).await {
                log_error!("Error submitting transaction: {}", e);
                continue;
            }

            // Mine mchain block
            if let Err(e) = self.mchain.mine_block(slot.timestamp).await {
                log_error!("Error mining block: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;
    use tokio::sync::mpsc;

    /// Test the block builder startup and basic functionality
    /// This test requires Anvil (part of Foundry toolchain) to simulate a local Ethereum node.
    /// The test spawns an Anvil instance with custom parameters for gas and mining settings.
    #[tokio::test]
    async fn test_block_builder_start() -> Result<()> {
        let (tx, rx) = mpsc::channel(32);
        let config = BlockBuilderConfig::default();

        let builder = BlockBuilder::new(rx, config).await?;

        // Start the block builder in a separate task
        let handle = tokio::spawn(async move { builder.start().await });

        // Send a test block
        let test_slot = Slot::new(1, 1);
        tx.send(test_slot).await?;

        // Give some time for processing
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Clean shutdown
        handle.abort();
        Ok(())
    }
}

#[allow(missing_docs)] // self-documenting
#[derive(Debug, Error)]
pub enum BlockBuilderError {
    #[error("Anvil failed to start: {0}")]
    AnvilStartError(&'static str),

    #[error("Failed to submit transaction to MetaChain: {0}")]
    SubmitTxnError(RpcError<TransportErrorKind>),

    #[error("Cannot serialize empty l2 msg")]
    EmptyL2Message(),
}
