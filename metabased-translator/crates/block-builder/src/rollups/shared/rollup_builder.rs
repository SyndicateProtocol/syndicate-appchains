//! Shared traits and types for rollup-specific block builders.
//!
//! This module provides the core [`RollupBlockBuilder`] trait that defines how
//! different rollup implementations can construct and process their blocks.

use alloy::{
    primitives::{Address, Bytes},
    rpc::types::TransactionRequest,
};
use async_trait::async_trait;
use common::types::BlockAndReceipts;
use eyre::{Error, Result};
use std::{
    fmt::Debug,
    marker::{Send, Sync},
};

use crate::rollups::shared::SequencingTransactionParser;

/// Trait for rollup-specific block builders that construct batches from transactions
#[async_trait]
pub trait RollupBlockBuilder: Debug + Send + Sync + Unpin + 'static {
    /// Creates a new block builder instance with a sequencing contract address.
    ///
    /// # Arguments
    /// - `sequencing_contract_address`: The address of the sequencing contract to monitor.
    ///
    /// # Returns
    /// - A new instance of the implementing type.
    fn new(sequencing_contract_address: Address) -> Self
    where
        Self: Sized;

    /// Builds a batch of transactions into a rollup-specific batch transaction
    async fn build_batch_txn(&self, txs: Vec<Bytes>) -> Result<TransactionRequest, Error>;

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
        let mut parsed_transactions = Vec::new();
        for block in input {
            for receipt in block.receipts {
                for log in receipt.logs {
                    if let Ok(transactions) = self.transaction_parser().get_event_transactions(&log)
                    {
                        parsed_transactions.extend(transactions);
                    }
                }
            }
        }
        parsed_transactions
    }

    /// Provides access to the transaction parser used by the block builder.
    fn transaction_parser(&self) -> &SequencingTransactionParser;
}
