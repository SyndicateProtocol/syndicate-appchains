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
use tracing::{debug, error, info};

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

        let builder: Box<dyn RollupBlockBuilder> = match config.target_rollup_type {
            TargetRollupType::ARBITRUM => Box::new(ArbitrumBlockBuilder::new(config)),
            TargetRollupType::OPTIMISM => {
                Box::new(OptimismBlockBuilder::new(config.sequencing_contract_address))
            }
        };

        Ok(Self { slotter_rx, mchain, builder })
    }

    /// Start the block builder
    pub async fn start(
        mut self,
        known_safe_block_number: Option<u64>,
        mut shutdown_rx: oneshot::Receiver<()>,
    ) {
        // Add reorg handling at start
        if let Some(block_number) = known_safe_block_number {
            if let Err(e) = self.mchain.rollback_to_block(block_number).await {
                panic!("Error during startup, unable to reorg to block: {}", e);
            }
        }

        loop {
            tokio::select! {
                Some(slot) = self.slotter_rx.recv() => {
                    debug!("Received slot: {:?}", slot);

                    // [OP / ARB] Build block of MChain transactions from slot
                    let transactions = match self.builder.build_block_from_slot(slot.clone()).await {
                        Ok(transactions) => transactions,
                        Err(e) => {
                            panic!("Error building batch transaction: {}", e);
                        }
                    };

                    debug!("Submitting {} transactions", transactions.len());

                    // Submit transactions to mchain
                    if let Err(e) = self.mchain.submit_txns(transactions).await {
                        panic!("Error submitting transaction: {}", e);
                    }

                    // Mine mchain block
                    if let Err(e) = self.mchain.mine_block(slot.timestamp).await {
                        panic!("Error mining block: {}", e);
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::providers::Provider;
    use eyre::Result;
    use tokio::sync::mpsc;

    /// Test the block builder startup and basic functionality
    /// This test requires Anvil (part of Foundry toolchain) to simulate a local Ethereum node.
    /// The test spawns an Anvil instance with custom parameters for gas and mining settings.
    #[tokio::test]
    async fn test_block_builder_start() -> Result<()> {
        let (tx, rx) = mpsc::channel(32);
        let config = BlockBuilderConfig::default();
        let genesis_ts = config.genesis_timestamp;
        let builder = BlockBuilder::new(rx, config).await?;

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let handle = tokio::spawn(async move { builder.start(None, shutdown_rx).await });

        // Send a test block
        let test_slot = Slot::new(1, genesis_ts + 1);
        tx.send(test_slot).await?;

        // Give some time for processing
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Clean shutdown
        let _ = shutdown_tx.send(());
        handle.await?;
        Ok(())
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    #[ignore] // TODO SEQ-528 unskip/re-write
    async fn test_block_builder_resume_from_known_safe_slot() -> Result<()> {
        let (tx, rx) = mpsc::channel(1);
        let config = BlockBuilderConfig::default();
        let builder = BlockBuilder::new(rx, config.clone()).await?;

        let provider = builder.mchain.provider.clone();

        // First run: send a few slots
        let test_slot1 = Slot::new(1, 1000);
        let test_slot2 = Slot::new(2, 2000);
        let test_slot3 = Slot::new(3, 3000);

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let handle = tokio::spawn(async move { builder.start(None, shutdown_rx).await });

        tx.send(test_slot1).await?;
        tx.send(test_slot2.clone()).await?;
        tx.send(test_slot3).await?;

        // Give time for processing and state to be persisted
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        let latest_block = provider.get_block_number().await?;
        assert_eq!(latest_block, 3, "Chain should be at block 3");

        let _ = shutdown_tx.send(());
        handle.await?;

        // Second run: resume builder
        let (_resumed_tx, resumed_rx) = mpsc::channel(1);
        let resumed_builder = BlockBuilder::new(resumed_rx, config).await?;

        let resumed_provider = resumed_builder.mchain.provider.clone();

        // resumed builder with the "last known safe slot" as slot2
        let (_shutdown_tx, shutdown_rx) = oneshot::channel();
        tokio::spawn(
            async move { resumed_builder.start(Some(test_slot2.number), shutdown_rx).await },
        );

        // Give time for rollback to slot0
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        let latest_block = resumed_provider.get_block_number().await?;
        assert_eq!(latest_block, 2, "Chain should be at block 2 after reorg");

        Ok(())
    }

    // TODO SEQ-529 - write a test that asserts for determinism (same slots should yield the same
    // block chain on separate block builders)
}
