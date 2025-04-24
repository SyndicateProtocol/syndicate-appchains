//! Shared traits and types for rollup-specific block builders.
//!
//! This module provides the core [`RollupAdapter`] trait that defines how
//! different rollup implementations can construct and process their blocks.

use crate::{connectors::mchain::MetaChainProvider, rollups::shared::SequencingTransactionParser};
use alloy::{eips::BlockNumberOrTag, primitives::Bytes};
use async_trait::async_trait;
use common::types::{KnownState, PartialBlock, SeqBlock, SetBlock};
use eyre::Result;
use mchain::db::DelayedMessage;
use std::{
    fmt::Debug,
    marker::{Send, Sync},
};

/// Trait for rollup-specific block builders that construct batches from sequencer transactions
/// and delayed messages from settlement ones.
#[async_trait]
pub trait RollupAdapter: Debug + Send + Sync + Unpin + Clone + 'static {
    /// Parses a sequencing chain block into a batch.
    ///
    /// Uses the associated transaction parser to extract transactions
    /// from the logs within block receipts.
    ///
    /// # Arguments
    /// * `input` - A block along with its associated receipts.
    ///
    /// # Returns
    /// A vector of extracted transactions in raw `Bytes` format.
    fn parse_block_to_mbtxs(&self, input: &PartialBlock) -> Vec<Bytes> {
        input
            .logs
            .iter()
            .filter_map(|log| self.transaction_parser().get_event_transactions(log).ok())
            .flatten()
            .collect()
    }

    /// Provides access to the transaction parser used by the block builder.
    fn transaction_parser(&self) -> &SequencingTransactionParser;

    /// Builds a batch from a sequencing block
    /// Returns (number of transactions, batch data)
    fn build_batch(&self, block: &PartialBlock) -> Result<(u64, Bytes)>;

    /// Extract delayed messages from a settlement block
    fn process_delayed_messages(&self, block: &PartialBlock) -> Result<Vec<DelayedMessage>>;

    /// Gets the source chain's processed blocks from the rollup
    async fn get_processed_blocks(
        &self,
        provider: &MetaChainProvider<Self>,
        block: BlockNumberOrTag,
    ) -> Result<Option<KnownState>>;
}

/// A trait for building blocks from the sequencing and settlement chains.
#[async_trait]
pub trait BlockBuilder<T>: Send {
    /// Process a single slot
    fn build_block(&self, block: &PartialBlock) -> Result<T>;
}

impl<T: RollupAdapter> BlockBuilder<SeqBlock> for T {
    fn build_block(&self, block: &PartialBlock) -> Result<SeqBlock> {
        let (tx_count, batch) = self.build_batch(block)?;
        Ok(SeqBlock {
            block_ref: block.block_ref.clone(),
            parent_hash: block.parent_hash,
            tx_count,
            batch,
        })
    }
}

impl<T: RollupAdapter> BlockBuilder<SetBlock> for T {
    fn build_block(&self, block: &PartialBlock) -> Result<SetBlock> {
        Ok(SetBlock {
            block_ref: block.block_ref.clone(),
            parent_hash: block.parent_hash,
            messages: self.process_delayed_messages(block)?,
        })
    }
}
