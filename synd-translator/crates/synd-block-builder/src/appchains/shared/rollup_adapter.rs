//! Shared traits and types for rollup-specific block builders.
//!
//! This module provides the core [`RollupAdapter`] trait that defines how
//! different rollup implementations can construct and process their blocks.

use crate::appchains::shared::SequencingTransactionParser;
use alloy::primitives::{Bytes, TxHash};
use async_trait::async_trait;
use shared::types::PartialBlock;
use std::marker::{Send, Sync};
use tracing::warn;

/// Trait for rollup-specific block builders that construct batches from sequencer transactions
/// and delayed messages from settlement ones.
#[async_trait]
pub trait RollupAdapter: Send + Sync {
    /// Parses a sequencing chain block into a batch.
    ///
    /// Uses the associated transaction parser to extract transactions
    /// from the logs within block receipts.
    ///
    /// # Arguments
    /// * `input` - A block along with its associated receipts.
    ///
    /// # Returns
    /// A vector of extracted transaction data containing the raw `Bytes` and tx hash
    fn parse_block_to_mbtxs(&self, input: &PartialBlock) -> Vec<(Bytes, TxHash)> {
        input
            .logs
            .iter()
            .filter_map(|log| match self.transaction_parser().get_event_transactions(log) {
                Ok(txs) => Some(txs),
                Err(e) => {
                    warn!("Failed to get event transactions from log: {:?}, error: {:?}", log, e);
                    None
                }
            })
            .flatten()
            .collect()
    }

    /// Provides access to the transaction parser used by the block builder.
    fn transaction_parser(&self) -> &SequencingTransactionParser;
}
