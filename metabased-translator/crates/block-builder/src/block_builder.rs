//! Block builder service for processing and building L3 blocks.

use crate::{
    config::{BlockBuilderConfig, TargetRollupType},
    connectors::mchain::MetaChainProvider,
    metrics::BlockBuilderMetrics,
    rollups::{
        arbitrum::arbitrum_builder::ArbitrumBlockBuilder,
        optimism::optimism_builder::OptimismBlockBuilder, shared::RollupBlockBuilder,
    },
};
use alloy::{
    eips::BlockNumberOrTag,
    transports::{RpcError, TransportErrorKind},
};
use common::{db::TranslatorStore, types::Slot};
use derivative::Derivative;
use eyre::{Error, Result};
use std::sync::Arc;
use tokio::sync::{mpsc::Receiver, oneshot};
use tracing::{error, info, trace};
use url::Url;

/// Block builder service for processing and building L3 blocks.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct BlockBuilder {
    slotter_rx: Receiver<Slot>,
    #[allow(missing_docs)]
    pub mchain: MetaChainProvider,
    builder: Box<dyn RollupBlockBuilder>,
    slot_duration_sec: u64,
    metrics: BlockBuilderMetrics,

    #[derivative(Debug = "ignore")]
    store: Arc<dyn TranslatorStore + Send + Sync>,
}

impl BlockBuilder {
    /// Create a new block builder
    pub async fn new(
        slotter_rx: Receiver<Slot>,
        config: &BlockBuilderConfig,
        slot_duration_sec: u64,
        store: Arc<dyn TranslatorStore + Send + Sync>,
        metrics: BlockBuilderMetrics,
    ) -> Result<Self, Error> {
        let mchain = MetaChainProvider::start(config, &metrics.mchain_metrics).await?;

        let builder: Box<dyn RollupBlockBuilder> = match config.target_rollup_type {
            TargetRollupType::ARBITRUM => Box::new(ArbitrumBlockBuilder::new(config)),
            TargetRollupType::OPTIMISM => {
                Box::new(OptimismBlockBuilder::new(config.sequencing_contract_address))
            }
        };

        Ok(Self { slotter_rx, mchain, builder, metrics, slot_duration_sec, store })
    }

    /// Validates and rolls back to a known block number if necessary
    async fn resume_from_block(
        &self,
        known_block_number: Option<u64>,
    ) -> Result<(), BlockBuilderError> {
        let Some(known_block_number) = known_block_number else {
            info!("No known block number to resume from, starting from genesis");
            return Ok(());
        };

        let current_block_number = self.mchain.get_block_number().await.map_err(|e| {
            BlockBuilderError::ResumeFromBlock(format!("Error getting current block number: {}", e))
        })?;

        if known_block_number > current_block_number {
            return Err(BlockBuilderError::ResumeFromBlock(format!(
                "Known block(slot) number {} is greater than the current mchain block number {}",
                known_block_number, current_block_number
            )));
        }

        // rollback to block if necessary
        if known_block_number < current_block_number {
            self.mchain.rollback_to_block(known_block_number).await.map_err(|e| {
                BlockBuilderError::ResumeFromBlock(format!("Unable to reorg to block: {}", e))
            })?;
        }
        info!("Resumed from block: {}", known_block_number);
        Ok(())
    }

    async fn verify_block(&self, transactions_len: usize, slot_number: u64) {
        let current_block = self.get_current_block_number().await;
        assert_eq!(
            current_block, slot_number,
            "Mined block number {} does not match slot number {}",
            current_block, slot_number
        );
        trace!("Mined block: {:?} from slot: {:?}", current_block, slot_number);

        // Verify transactions are all included and succeeded
        // TODO(SEQ-623): check to make sure the tx hashes match as well
        let receipts = self
            .mchain
            .get_block_receipts(BlockNumberOrTag::Number(current_block))
            .await
            .unwrap_or_else(|e| {
                panic!("Failed to fetch receipts for block {:#?}: {:#?}", current_block, e)
            });
        assert!(
            receipts.len() == transactions_len,
            "expected {:#?} receipts, got {:#?}",
            transactions_len,
            receipts
        );

        for r in receipts {
            assert!(r.status == 1, "tx failed: {:#?}", r);
        }
    }

    /// Start the block builder
    pub async fn start(
        mut self,
        known_block_number: Option<u64>,
        mut shutdown_rx: oneshot::Receiver<()>,
    ) {
        // resume from known state
        if let Err(e) = self.resume_from_block(known_block_number).await {
            panic!("Failed to validate and rollback: {}", e);
        }

        loop {
            tokio::select! {
                biased; // biased allows us to process everything that's in the channel before shutting down
                Some(slot) = self.slotter_rx.recv() => {
                    trace!("Received slot: {:?}", slot);
                    self.metrics.record_last_slot(slot.number);

                    // [OP / ARB] Build block of MChain transactions from slot
                    let transactions = match self.builder.build_block_from_slot(slot.clone()).await {
                        Ok(transactions) => transactions,
                        Err(e) => {
                            panic!("Error building batch transaction: {}", e);
                        }
                    };

                    let transactions_len = transactions.len();
                    trace!("Submitting {} transactions", transactions_len);
                    self.metrics.record_transactions_per_slot(transactions_len);

                    // Fill gap with empty blocks if needed
                    let mut block_number = self.get_current_block_number().await;
                    while block_number < slot.number-1 {
                        let empty_block_timestamp = slot.timestamp - ((slot.number - block_number) * self.slot_duration_sec);
                        trace!("Mining empty block {} with timestamp {}", block_number + 1, empty_block_timestamp);

                        if let Err(e) = self.mchain.mine_block(empty_block_timestamp).await {
                            panic!("Error mining block: {}", e);
                        }
                        block_number += 1;
                    }

                    // Submit transactions to mchain
                    if let Err(e) = self.mchain.submit_txns(transactions).await {
                        panic!("Error submitting transaction: {}", e);
                    }

                    // Mine the actual block with slot timestamp
                    if let Err(e) = self.mchain.mine_block(slot.timestamp).await {
                        panic!("Error mining block: {}", e);
                    }

                    // TODO(SEQ-623): add a flag to enable/disable this check
                    self.verify_block(transactions_len, slot.number).await;

                    if let Err(e) = self.store.save_unsafe_slot(&slot).await {
                        panic!("Error saving slot: {}", e);
                    }
                }
                _ = &mut shutdown_rx => {
                    drop(self.mchain);
                    info!("Block builder stopped");
                    return;
                }
            }
        }
    }

    async fn get_current_block_number(&self) -> u64 {
        self.mchain.get_block_number().await.unwrap_or_else(|e| {
            panic!("Error getting current block number: {}", e);
        })
    }
}

#[allow(missing_docs)] // self-documenting
#[derive(Debug, thiserror::Error)]
pub enum BlockBuilderError {
    #[error("Failed to submit transaction to MetaChain: {0}")]
    SubmitTxnError(RpcError<TransportErrorKind>),

    #[error("Cannot serialize empty l2 msg")]
    EmptyL2Message(),

    #[error("Error resuming from block: {0}")]
    ResumeFromBlock(String),

    #[error("Error mining block: {0}")]
    MineBlock(String),
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

/*
#[cfg(test)]
mod tests {
    use prometheus_client::registry::Registry;

    use super::*;
    use alloy::providers::Provider;
    use common::db::RocksDbStore;
    use eyre::Result;
    use test_utils::test_path;
    use tokio::sync::mpsc;
    use tracing_test::traced_test;

    struct MetricsState {
        /// Prometheus registry
        pub registry: Registry,
    }

    #[tokio::test]
    #[traced_test]
    #[ignore] // TODO SEQ-528 unskip/re-write
    async fn test_block_builder_resume_from_known_safe_slot() -> Result<()> {
        let (tx, rx) = mpsc::channel(1);
        let config = BlockBuilderConfig::default();
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = BlockBuilderMetrics::new(&mut metrics_state.registry);
        let db_path = test_path("db");
        let store = Arc::new(RocksDbStore::new(db_path.as_str()).unwrap());
        let builder = BlockBuilder::new(rx, &config, 2, store, metrics).await?;

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
        let store = Arc::new(RocksDbStore::new(db_path.as_str()).unwrap());
        let resumed_builder = BlockBuilder::new(resumed_rx, &config, 2, store, metrics).await?;

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
*/
