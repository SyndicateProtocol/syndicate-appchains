//! Optimism block builder implementation
//!
//! This module provides functionality for building an Optimism batcher transaction from a list of
//! transactions. It implements the [`RollupBlockBuilder`] trait to standardize block construction
//! across different rollup implementations

use crate::{
    config::BlockBuilderConfig,
    rollups::{
        optimism::{
            batch::{new_batcher_tx, Batch},
            frame::to_data,
        },
        shared::{RollupAdapter, SequencingTransactionParser},
    },
};
use alloy::{
    primitives::{Address, Bytes, B256},
    rpc::types::TransactionRequest,
};
use async_trait::async_trait;
use eyre::Result;
use mchain::db::DelayedMessage;
use shared::types::PartialBlock;
use std::str::FromStr;

#[derive(Debug, Clone)]
/// Builder for constructing Optimism blocks from transactions
pub struct OptimismAdapter {}

#[async_trait]
impl RollupAdapter for OptimismAdapter {
    fn build_batch(&self, _: &PartialBlock) -> Result<(u64, Bytes)> {
        panic!("Not implemented")
    }

    fn process_delayed_messages(&self, _: &PartialBlock) -> Result<Vec<DelayedMessage>> {
        panic!("Not implemented")
    }

    fn transaction_parser(&self) -> &SequencingTransactionParser {
        panic!("Not implemented")
    }
}

#[allow(dead_code)]
impl OptimismAdapter {
    /// Creates a new Optimism block builder
    pub const fn new(_config: &BlockBuilderConfig) -> Self {
        Self {}
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
    use alloy::primitives::{Address, TxKind};
    use std::str::FromStr;

    #[tokio::test]
    async fn test_build_batch_txn() {
        let sequencing_contract_address =
            Address::from_str("0x1234000000000000000000000000000000000000")
                .expect("Invalid address format");
        let builder = OptimismAdapter::new(&BlockBuilderConfig {
            sequencing_contract_address: Some(sequencing_contract_address),
            ..Default::default()
        });
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
