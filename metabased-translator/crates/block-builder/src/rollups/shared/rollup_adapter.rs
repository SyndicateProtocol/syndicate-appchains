//! Shared traits and types for rollup-specific block builders.
//!
//! This module provides the core [`RollupAdapter`] trait that defines how
//! different rollup implementations can construct and process their blocks.

use crate::{connectors::mchain::MetaChainProvider, rollups::shared::SequencingTransactionParser};
use alloy::{
    eips::BlockNumberOrTag,
    primitives::{Address, Bytes},
};
use async_trait::async_trait;
use common::types::{KnownState, PartialBlock, Slot};
use eyre::{Error, Result};
use mchain::db::MBlock;
use std::{
    fmt::Debug,
    marker::{Send, Sync},
};

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
    fn parse_block_to_mbtxs(&self, input: PartialBlock) -> Vec<Bytes> {
        input
            .logs
            .iter()
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
    async fn get_processed_blocks(
        &self,
        provider: &MetaChainProvider<Self>,
        block: BlockNumberOrTag,
    ) -> Result<Option<(KnownState, u64)>>;

    /// Gets the last sequencing block processed
    async fn get_last_sequencing_block_processed(&self, provider: &MetaChainProvider<Self>) -> u64;

    /// Returns a list of addresses that are interesting to monitor on the sequencing chain
    fn sequencing_addresses_to_monitor(&self) -> Vec<Address>;

    /// Returns a list of addresses that are interesting to monitor on the settlement chain
    fn settlement_addresses_to_monitor(&self) -> Vec<Address>;
}
