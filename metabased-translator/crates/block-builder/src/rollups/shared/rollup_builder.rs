//! Shared traits and types for rollup-specific block builders.
//!
//! This module provides the core [`RollupBlockBuilder`] trait that defines how
//! different rollup implementations can construct and process their blocks.

use crate::rollups::shared::SequencingTransactionParser;
use alloy::{primitives::Bytes, rpc::types::TransactionRequest};
use async_trait::async_trait;
use common::types::{BlockAndReceipts, Slot};
use eyre::{Error, Result};
use std::{
    fmt::Debug,
    marker::{Send, Sync},
};

/// Trait for rollup-specific block builders that construct batches from transactions
#[async_trait]
pub trait RollupBlockBuilder: Debug + Send + Sync + Unpin + 'static {
    /// Parses sequencing chain blocks into metabased transactions.
    ///
    /// By default, this method uses the associated transaction parser to extract
    /// transactions from the logs within block receipts.
    ///
    /// # Arguments
    /// * `input` - A vector of blocks along with their associated receipts.
    ///
    /// # Returns
    /// A vector of extracted transactions in raw `Bytes` format.
    fn parse_blocks_to_mbtxs(&self, input: Vec<BlockAndReceipts>) -> Vec<Bytes> {
        input
            .into_iter()
            .flat_map(|block| block.receipts)
            .flat_map(|receipt| receipt.logs)
            .filter_map(|log| self.transaction_parser().get_event_transactions(&log).ok())
            .flatten()
            .collect()
    }

    /// Provides access to the transaction parser used by the block builder.
    fn transaction_parser(&self) -> &SequencingTransactionParser;

    /// Builds a block from a slot
    async fn build_block_from_slot(&mut self, slot: Slot)
        -> Result<Vec<TransactionRequest>, Error>;
}
