//! Block builder service for processing and building L3 blocks.

use crate::{
    config::{get_slot_tracker_contract_address, BlockBuilderConfig, TargetRollupType},
    connectors::anvil::MetaChainProvider,
    metrics::BlockBuilderMetrics,
    rollups::{
        arbitrum::arbitrum_builder::ArbitrumBlockBuilder,
        optimism::optimism_builder::OptimismBlockBuilder, shared::RollupBlockBuilder,
    },
};
use alloy::{
    primitives::U256,
    providers::Provider,
    rpc::types::{BlockNumberOrTag, BlockTransactionsKind, TransactionRequest},
    sol_types::SolCall,
    transports::{RpcError, TransportErrorKind},
};
use common::types::Slot;
use contract_bindings::arbitrum::slottracker::SlotTracker;
use eyre::{eyre, Error, Result};
use tokio::sync::{mpsc::Receiver, oneshot};
use tracing::{debug, error, info};
use url::Url;

struct SlotInfo {
    number: u64,
    timestamp: u64,
    latest_sequencing_block_number: u64,
    latest_settlement_block_number: u64,
}

/// Block builder service for processing and building L3 blocks.
#[derive(Debug)]
pub struct BlockBuilder {
    slotter_rx: Receiver<Slot>,
    #[allow(missing_docs)]
    pub mchain: MetaChainProvider,
    builder: Box<dyn RollupBlockBuilder>,
    metrics: BlockBuilderMetrics,
}

impl BlockBuilder {
    /// Create a new block builder
    pub async fn new(
        slotter_rx: Receiver<Slot>,
        config: &BlockBuilderConfig,
        metrics: BlockBuilderMetrics,
    ) -> Result<Self, Error> {
        let mchain = MetaChainProvider::start(config, &metrics.mchain_metrics).await?;

        let builder: Box<dyn RollupBlockBuilder> = match config.target_rollup_type {
            TargetRollupType::ARBITRUM => Box::new(ArbitrumBlockBuilder::new(config)),
            TargetRollupType::OPTIMISM => {
                Box::new(OptimismBlockBuilder::new(config.sequencing_contract_address))
            }
        };

        Ok(Self { slotter_rx, mchain, builder, metrics })
    }

    pub async fn get_latest_slot(&self) -> Result<SlotInfo> {
        // Create the call data for getSlotInfo()
        let tx = TransactionRequest::default()
            .to(get_slot_tracker_contract_address())
            .input(SlotTracker::getSlotInfoCall::new(()).abi_encode().into());

        // Use the provider to make the call
        let info = self.mchain.provider.call(&tx).await?;

        let latest_block = self
            .mchain
            .provider
            .get_block_by_number(BlockNumberOrTag::Latest, BlockTransactionsKind::Hashes)
            .await?
            .ok_or(eyre!("No block found"))?;

        // Decode the response
        let decoded = SlotTracker::getSlotInfoCall::abi_decode_returns(&info, true)?;

        Ok(SlotInfo {
            number: decoded._0.to::<u64>(),
            timestamp: latest_block.header.timestamp,
            latest_sequencing_block_number: decoded
                ._1
                .last()
                .ok_or(eyre!("No sequencing blocks"))?
                .to::<u64>(),
            latest_settlement_block_number: decoded
                ._2
                .last()
                .ok_or(eyre!("No settlement blocks"))?
                .to::<u64>(),
        })
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
                    self.metrics.record_last_slot(slot.number);

                    // Get block numbers from BlockAndReceipts
                    let sequencing_block_numbers: Vec<U256> = slot.sequencing_chain_blocks
                        .iter()
                        .map(|block_and_receipts| U256::from(block_and_receipts.block.number))
                        .collect();

                    let settlement_block_numbers: Vec<U256> = slot.settlement_chain_blocks
                        .iter()
                        .map(|block_and_receipts| U256::from(block_and_receipts.block.number))
                        .collect();

                    // Slot Info Transaction
                    let slot_tx = TransactionRequest::default().to(get_slot_tracker_contract_address()).input(
                        SlotTracker::setSlotInfoCall::new((U256::from(slot.number), sequencing_block_numbers, settlement_block_numbers))
                        .abi_encode()
                        .into(),
                    );

                    // [OP / ARB] Build block of MChain transactions from slot
                    let transactions = match self.builder.build_block_from_slot(slot.clone()).await {
                        Ok(transactions) => transactions,
                        Err(e) => {
                            panic!("Error building batch transaction: {}", e);
                        }
                    };

                    let transactions_len = transactions.len();
                    self.metrics.record_transactions_per_slot(transactions_len);
                    debug!("Submitting {} transactions", transactions_len);

                    // Submit transactions to mchain
                    let mut all_transactions = vec![slot_tx];
                    all_transactions.extend(transactions);
                    if let Err(e) = self.mchain.submit_txns(all_transactions).await {
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
#[derive(Debug, thiserror::Error)]
pub enum BlockBuilderError {
    #[error("Error starting Anvil: {0}")]
    AnvilStart(AnvilStartError),

    #[error("Failed to submit transaction to MetaChain: {0}")]
    SubmitTxnError(RpcError<TransportErrorKind>),

    #[error("Cannot serialize empty l2 msg")]
    EmptyL2Message(),
}

#[allow(missing_docs)] // self-documenting
#[derive(Debug, thiserror::Error)]
pub enum AnvilStartError {
    #[error("Invalid host in mchain_url")]
    InvalidHost,
    #[error("No port found in mchain_url")]
    NoPort,
    #[error("Requested port in mchain_url {mchain_url:} is unavailable: {port}")]
    PortUnavailable { mchain_url: Url, port: u16 },
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::providers::Provider;
    use eyre::Result;
    use prometheus_client::registry::Registry;
    use tokio::sync::mpsc;
    use tracing_test::traced_test;

    struct MetricsState {
        /// Prometheus registry
        pub registry: Registry,
    }

    /// Test the block builder startup and basic functionality
    /// This test requires Anvil (part of Foundry toolchain) to simulate a local Ethereum node.
    /// The test spawns an Anvil instance with custom parameters for gas and mining settings.
    #[tokio::test]
    async fn test_block_builder_start() -> Result<()> {
        let (tx, rx) = mpsc::channel(32);
        let config = BlockBuilderConfig::default();
        let genesis_ts = config.genesis_timestamp;
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = BlockBuilderMetrics::new(&mut metrics_state.registry);
        let builder = BlockBuilder::new(rx, &config, metrics).await?;

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
    #[traced_test]
    #[ignore] // TODO SEQ-528 unskip/re-write
    async fn test_block_builder_resume_from_known_safe_slot() -> Result<()> {
        let (tx, rx) = mpsc::channel(1);
        let config = BlockBuilderConfig::default();
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = BlockBuilderMetrics::new(&mut metrics_state.registry);
        let builder = BlockBuilder::new(rx, &config, metrics).await?;

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
        let metrics = BlockBuilderMetrics::new(&mut metrics_state.registry);
        let resumed_builder = BlockBuilder::new(resumed_rx, &config, metrics).await?;

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
