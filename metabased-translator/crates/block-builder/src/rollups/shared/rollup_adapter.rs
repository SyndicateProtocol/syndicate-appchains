//! Shared traits and types for rollup-specific block builders.
//!
//! This module provides the core [`RollupAdapter`] trait that defines how
//! different rollup implementations can construct and process their blocks.

use crate::rollups::shared::SequencingTransactionParser;
use alloy::{
    eips::BlockNumberOrTag,
    primitives::{Address, Bytes, FixedBytes},
    providers::Provider,
};
use async_trait::async_trait;
use common::types::{BlockAndReceipts, KnownState, Slot};
use eyre::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    marker::{Send, Sync},
    sync::Arc,
};

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DelayedMessage {
    pub kind: u8,
    pub sender: Address,
    pub data: Bytes,
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MBlock {
    pub timestamp: u64,
    pub messages: Vec<DelayedMessage>,
    pub batch: Bytes,
    pub seq_block_number: u64,
    pub seq_block_hash: FixedBytes<32>,
    pub set_block_number: u64,
    pub set_block_hash: FixedBytes<32>,
}

/// Trait for rollup-specific block builders that construct batches from transactions
#[async_trait]
pub trait RollupAdapter: Debug + Send + Sync + Unpin + Clone + 'static {
    /// Parses a sequencing chain block into metabased transactions.
    ///
    /// Uses the associated transaction parser to extract transactions
    /// from the logs within block receipts.
    ///
    /// # Arguments
    /// * `input` - A block along with its associated receipts.
    ///
    /// # Returns
    /// A vector of extracted transactions in raw `Bytes` format.
    fn parse_block_to_mbtxs(&self, input: Arc<BlockAndReceipts>) -> Vec<Bytes> {
        input
            .receipts
            .iter()
            .flat_map(|receipt| &receipt.logs)
            .filter_map(|log| self.transaction_parser().get_event_transactions(log).ok())
            .flatten()
            .collect()
    }

    /// Provides access to the transaction parser used by the block builder.
    fn transaction_parser(&self) -> &SequencingTransactionParser;

    /// Builds a block from a slot
    async fn build_block_from_slot(
        &self,
        slot: &Slot,
        mchain_block_number: u64,
    ) -> Result<Option<MBlock>, Error>;

    /// Gets the source chain's processed blocks from the rollup
    async fn get_processed_blocks<T: Provider>(
        &self,
        provider: &T,
        block: BlockNumberOrTag,
    ) -> Result<Option<(KnownState, u64)>>;

    /// Gets the last sequencing block processed
    async fn get_last_sequencing_block_processed<T: Provider>(&self, provider: &T) -> Result<u64>;
}
