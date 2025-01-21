//! Shared traits and types for rollup-specific block builders.
//!
//! This module provides the core [`RollupBlockBuilder`] trait that defines how
//! different rollup implementations can construct and process their blocks.

use alloy::{primitives::Bytes, rpc::types::TransactionRequest};
use async_trait::async_trait;
use common::types::BlockAndReceipts;
use eyre::{Error, Result};
use std::{
    fmt::Debug,
    marker::{Send, Sync},
};

/// Trait for rollup-specific block builders that construct batches from transactions
#[async_trait]
pub trait RollupBlockBuilder: Debug + Send + Sync + Unpin + 'static {
    /// Creates a new block builder instance
    fn new() -> Self
    where
        Self: Sized;

    /// Builds a batch of transactions into a rollup-specific batch transaction
    async fn build_batch_txn(&self, txs: Vec<Bytes>) -> Result<TransactionRequest, Error>;

    /// Parses sequencing chain blocks into meta based transactions
    fn parse_blocks_to_mbtxs(&self, _blocks: Vec<BlockAndReceipts>) -> Vec<Bytes> {
        // TODO: Implement
        vec![]
    }
}
