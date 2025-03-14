//! Shared traits and types for rollup-specific block builders.
//!
//! This module provides the core [`RollupAdapter`] trait that defines how
//! different rollup implementations can construct and process their blocks.

use crate::rollups::shared::SequencingTransactionParser;
use alloy::{
    eips::BlockNumberOrTag, primitives::Bytes, providers::Provider, rpc::types::TransactionRequest,
};
use async_trait::async_trait;
use common::types::{BlockAndReceipts, KnownState, Slot};
use eyre::{Error, Result};
use std::{
    fmt::Debug,
    marker::{Send, Sync},
    sync::Arc,
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
    ) -> Result<Vec<TransactionRequest>, Error>;

    /// decodes an error from the rollup contract - useful for humanly readable logs
    fn decode_error(&self, output: &Bytes) -> String;

    /// Gets the source chain's processed blocks from the rollup
    async fn get_processed_blocks<T: Provider>(
        &self,
        provider: &T,
        block: BlockNumberOrTag,
    ) -> Result<Option<(KnownState, u64)>>;

    /// Gets the last sequencing block processed
    async fn get_last_sequencing_block_processed<T: Provider>(&self, provider: &T) -> Result<u64>;
}
