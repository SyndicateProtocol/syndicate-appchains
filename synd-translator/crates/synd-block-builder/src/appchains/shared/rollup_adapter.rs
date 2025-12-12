//! Shared traits and types for rollup-specific block builders.
//!
//! This module provides the core [`RollupAdapter`] trait that defines how
//! different rollup implementations can construct and process their blocks.

use alloy::primitives::{Bytes, Log, TxHash};
use async_trait::async_trait;
use std::marker::{Send, Sync};

/// Trait for rollup-specific block builders that construct batches from sequencer transactions
/// and delayed messages from settlement ones.
#[async_trait]
pub trait RollupAdapter: Send + Sync {
    /// Decodes the event data into a vector of transactions
    fn get_event_transactions(&self, eth_log: &Log) -> eyre::Result<Vec<(Bytes, TxHash)>>;

    /// constructs a batch of sequenced transaction to be added to mchain in a format that's
    /// compatible with the rollup node
    /// NOTE: this must mirror the logic of the TEE enclave
    fn build_batch_bytes(
        &self,
        txs: Vec<Bytes>,
        l1_block_number: u64,
        mchain_timestamp: u64,
    ) -> eyre::Result<Bytes>; // TODO txhash is not used and should be rm'd
}
