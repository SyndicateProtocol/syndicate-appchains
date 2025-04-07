//! Block builder service for processing and building L3 blocks.

use crate::{connectors::mchain::MetaChainProvider, rollups::shared::RollupAdapter};
use alloy::transports::{RpcError, TransportErrorKind};
use async_trait::async_trait;
use common::types::{Slot, SlotProcessor};
use eyre::{Error, Result};
use tracing::trace;

// TODO - SEQ-693: re-structure the block-builder directory, and metrics.

#[async_trait]
impl<R: RollupAdapter> SlotProcessor for MetaChainProvider<R> {
    async fn process_slot(&self, slot: &Slot) -> Result<(), Error> {
        trace!("Received slot: {:?}", slot);
        self.metrics.record_last_slot(slot.sequencing.block.number);

        // [OP / ARB] Build block of MChain transactions from slot
        let batch = match self
            .rollup_adapter
            .build_block_from_slot(slot, self.provider.get_block_number().await + 1)
            .await
        {
            Ok(batch) => batch,
            Err(e) => {
                panic!("Error building batch transaction: {}", e);
            }
        };

        let batch = match batch {
            None => {
                trace!("Skipping empty block");
                return Ok(());
            }
            Some(x) => x,
        };

        trace!("Submitting {} transactions", batch.messages.len() + 1);
        self.metrics.record_transactions_per_slot(batch.messages.len() + 1);

        // Submit transactions to mchain
        self.provider.add_batch(batch).await?;

        Ok(())
    }
}

#[allow(missing_docs)] // self-documenting
#[derive(Debug, thiserror::Error)]
pub enum BlockBuilderError {
    #[error("Failed to submit transaction to MetaChain: {0}")]
    SubmitTxnError(RpcError<TransportErrorKind>),

    #[error("Error getting current block number: {0}")]
    GetCurrentBlockNumber(String),

    #[error("Known block(slot) number {0} is greater than the current mchain block number {1}")]
    KnownBlockNumberGreaterThanCurrentBlockNumber(u64, u64),

    #[error("Cannot serialize empty l2 msg")]
    EmptyL2Message(),

    #[error("Error resuming from block: {0}")]
    ResumeFromBlock(String),

    #[error("Error mining block: {0}")]
    MineBlock(String),

    #[error("Block builder was shut down")]
    Shutdown,
}
