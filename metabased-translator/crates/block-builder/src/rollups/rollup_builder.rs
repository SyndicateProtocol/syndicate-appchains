use alloy_rpc_types::TransactionRequest;
use async_trait::async_trait;
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
    async fn build_batch_txn(&self, txs: Vec<Vec<u8>>) -> Result<TransactionRequest, Error>;

    /// Parses sequencing chain blocks into meta based transactions
    fn parse_blocks_to_mbtxs(&self, _blocks: Vec<String>) -> Vec<Vec<u8>> {
        // TODO: Implement
        vec![]
    }
}
