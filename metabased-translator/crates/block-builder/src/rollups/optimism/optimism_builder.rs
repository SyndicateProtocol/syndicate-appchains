//! Optimism block builder implementation
//!
//! This module provides functionality for building an Optimism batcher transaction from a list of transactions.
//! It implements the [`RollupBlockBuilder`] trait to standardize block construction across different
//! rollup implementations

use crate::rollups::optimism::batch::{new_batcher_tx, Batch};
use crate::rollups::optimism::frame::to_data;
use crate::rollups::shared::RollupBlockBuilder;
use alloy_primitives::{Address, Bytes, B256};
use alloy_rpc_types::TransactionRequest;
use async_trait::async_trait;
use eyre::Result;
use std::str::FromStr;

#[derive(Debug)]
/// Builder for constructing Optimism blocks from transactions
pub struct OptimismBlockBuilder;

#[async_trait]
impl RollupBlockBuilder for OptimismBlockBuilder {
    /// Creates a new Optimism block builder
    fn new() -> Self {
        Self
    }

    /// Builds a batch of transactions into an Optimism batch
    async fn build_batch_txn(&self, txs: Vec<Bytes>) -> Result<TransactionRequest> {
        // TODO: Implement
        let batcher = Address::from_str("0x063D87A885a9323831A688645647eD7d0e859C5d")?;
        let batch_inbox = Address::from_str("0x97395dd253e2d096a0caa62a574895c3c2f2b2e0")?;
        let hash =
            B256::from_str("0xe009262cd1adf34cfaf845fd1c17a6ddb7f97c67b2992cd9f286ff4e1c6ad233")?;

        let single_batch = Batch {
            parent_hash: hash,
            epoch_num: 0,
            epoch_hash: hash,
            timestamp: 1712500002,
            transactions: txs,
        };
        let frames = single_batch.get_frames(1000000)?;
        let data = to_data(&frames)?;

        let tx = new_batcher_tx(batcher, batch_inbox, data.into());
        Ok(tx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, TxKind};
    use std::str::FromStr;

    #[test]
    fn test_new_builder() {
        let builder = OptimismBlockBuilder::new();
        assert!(matches!(builder, OptimismBlockBuilder));
    }

    #[tokio::test]
    async fn test_build_batch_txn() {
        let builder = OptimismBlockBuilder::new();
        let txs = vec![];

        let tx = builder.build_batch_txn(txs).await.unwrap();

        // Verify expected batcher address
        assert_eq!(
            tx.from,
            Some(Address::from_str("0x063D87A885a9323831A688645647eD7d0e859C5d").unwrap())
        );

        // Verify expected batch inbox address
        assert_eq!(
            tx.to,
            Some(TxKind::Call(
                Address::from_str("0x97395dd253e2d096a0caa62a574895c3c2f2b2e0").unwrap()
            ))
        );
    }
}
