//! Block builder service for processing and building L3 blocks.

use crate::{
    config::{BlockBuilderConfig, TargetRollupType},
    connectors::anvil::MetaChainProvider,
    rollups::{
        arbitrum::arbitrum_builder::ArbitrumBlockBuilder,
        optimism::optimism_builder::OptimismBlockBuilder, shared::RollupBlockBuilder,
    },
};
use alloy::transports::{RpcError, TransportErrorKind};
use common::types::Slot;
use eyre::{Error, Result};
use thiserror::Error;
use tokio::sync::{mpsc::Receiver, oneshot};
use tracing::{error, info};

/// Block builder service for processing and building L3 blocks.
#[derive(Debug)]
pub struct BlockBuilder {
    slotter_rx: Receiver<Slot>,
    mchain: MetaChainProvider,
    builder: Box<dyn RollupBlockBuilder>,
}

// TODO reorg to latest known safe slot

impl BlockBuilder {
    /// Create a new block builder
    pub async fn new(
        slotter_rx: Receiver<Slot>,
        config: BlockBuilderConfig,
    ) -> Result<Self, Error> {
        let mchain = MetaChainProvider::start(config.clone()).await?;

        let builder: Box<dyn RollupBlockBuilder> = match config.target_rollup_type {
            TargetRollupType::ARBITRUM => {
                Box::new(ArbitrumBlockBuilder::new(config.sequencing_contract_address))
            }
            TargetRollupType::OPTIMISM => {
                Box::new(OptimismBlockBuilder::new(config.sequencing_contract_address))
            }
        };

        Ok(Self { slotter_rx, mchain, builder })
    }

    /// Start the block builder
    pub async fn start(mut self, mut shutdown_rx: oneshot::Receiver<()>) {
        loop {
            tokio::select! {
                Some(slot) = self.slotter_rx.recv() => {
                    info!("Received slot: {:?}", slot);

                    // Process sequencing chain blocks into mB transactions
                    let mbtxs = self.builder.parse_blocks_to_mbtxs(slot.sequencing_chain_blocks);

                    // TODO (SEQ-416): [OP / ARB] Process deposit transactions

                    // [OP / ARB] Build and submit batch
                    let batch_txn = match self.builder.build_batch_txn(mbtxs).await {
                        Ok(txn) => txn,
                        Err(e) => {
                            error!("Error building batch transaction: {}", e);
                            continue;
                        }
                    };

                    // Submit batch transaction to mchain
                    if let Err(e) = self.mchain.submit_txn(batch_txn).await {
                        error!("Error submitting transaction: {}", e);
                        continue;
                    }

                    // Mine mchain block
                    if let Err(e) = self.mchain.mine_block(slot.timestamp).await {
                        error!("Error mining block: {}", e);
                    }
                }
                _ = &mut shutdown_rx => {
                    info!("Block builder shutting down");
                    break;
                }
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
        let mut builder = BlockBuilder::new(rx, config).await?;

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let handle = tokio::spawn(async move { builder.start(shutdown_rx).await });

        // Send a test block
        let test_slot = Slot::new(1, 1);
        tx.send(test_slot).await?;

        // Give some time for processing
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Clean shutdown
        let _ = shutdown_tx.send(());
        handle.await?;
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

    #[error("No contract addr found")]
    NoContractAddress(),

    #[error("No block number found")]
    NoBlockNumber(),

    #[error("Overflow error")]
    Overflow(),
}
