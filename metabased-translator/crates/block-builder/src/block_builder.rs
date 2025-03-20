//! Block builder service for processing and building L3 blocks.

use crate::{connectors::mchain::MetaChainProvider, rollups::shared::RollupAdapter};
use alloy::{
    eips::BlockNumberOrTag,
    providers::ext::TraceApi,
    transports::{RpcError, TransportErrorKind},
};
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
        let transactions = match self
            .rollup_adapter
            .build_block_from_slot(slot, self.get_current_block_number().await + 1)
            .await
        {
            Ok(transactions) => transactions,
            Err(e) => {
                panic!("Error building batch transaction: {}", e);
            }
        };

        let transactions_len = transactions.len();
        if transactions_len == 0 && !self.mine_empty_blocks {
            trace!("Skipping empty block");
            return Ok(());
        }

        trace!("Submitting {} transactions", transactions_len);
        self.metrics.record_transactions_per_slot(transactions_len);

        // Submit transactions to mchain
        if let Err(e) = self.submit_txns(transactions).await {
            panic!("Error submitting transaction: {}", e);
        }

        // Mine the actual block with slot timestamp
        if let Err(e) = self.mine_block(slot.timestamp()).await {
            panic!("Error mining block: {}", e);
        }

        // TODO(SEQ-623): add a flag to enable/disable this check
        self.verify_block(transactions_len, slot.sequencing.block.number).await;
        Ok(())
    }
}

impl<R: RollupAdapter> MetaChainProvider<R> {
    async fn verify_block(&self, transactions_len: usize, slot_seq_number: u64) {
        let current_block = self.get_current_block_number().await;
        trace!("Mined block: {:?} from slot: {:?}", current_block, slot_seq_number);

        // Verify transactions are all included and succeeded
        // TODO(SEQ-623): check to make sure the tx hashes match as well
        let receipts =
            self.get_block_receipts(BlockNumberOrTag::Number(current_block)).await.unwrap_or_else(
                |e| panic!("Failed to fetch receipts for block {:#?}: {:#?}", current_block, e),
            );

        assert!(
            receipts.len() == transactions_len,
            "expected {:#?} receipts, got {:#?}",
            transactions_len,
            receipts
        );

        for r in receipts {
            if r.status != 1 {
                let trace =
                    self.provider.trace_transaction(r.transaction_hash).await.unwrap_or_default();
                let error_msg = trace
                    .first()
                    .and_then(|t| t.trace.result.as_ref())
                    .map_or_else(String::new, |output| {
                        self.rollup_adapter.decode_error(output.output())
                    });

                panic!(
                    "tx failed: receipt: {:#?} trace: {:#?}, humanly_readable_error: {}",
                    r, trace, error_msg
                );
            }
        }
    }

    async fn get_current_block_number(&self) -> u64 {
        self.get_block_number().await.unwrap_or_else(|e| {
            panic!("Error getting current block number: {}", e);
        })
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
