//! Optimism block builder implementation
//!
//! This module provides functionality for building an Optimism batcher transaction from a list of
//! transactions. It implements the [`RollupBlockBuilder`] trait to standardize block construction
//! across different rollup implementations

use crate::rollups::{
    optimism::{
        batch::{new_batcher_tx, Batch},
        frame::to_data,
    },
    shared::{RollupBlockBuilder, SequencingTransactionParser},
};
use alloy::{
    primitives::{Address, Bytes, B256},
    rpc::types::TransactionRequest,
};
use async_trait::async_trait;
use common::types::{BlockAndReceiptsPointer, Slot};
use eyre::Result;
use std::str::FromStr;

#[derive(Debug)]
/// Builder for constructing Optimism blocks from transactions
pub struct OptimismBlockBuilder {
    transaction_parser: SequencingTransactionParser,
}

#[async_trait]
impl RollupBlockBuilder for OptimismBlockBuilder {
    async fn build_block_from_slot(&mut self, slot: &Slot) -> Result<Vec<TransactionRequest>> {
        let deposited_txns =
            self.process_deposited_txns(slot.settlement_chain_blocks.clone()).await?;

        let mbtxs = self.parse_blocks_to_mbtxs(slot.sequencing_chain_blocks.clone());

        let batch_txn = self.build_batch_txn(mbtxs).await?;

        let mut result: Vec<TransactionRequest> = vec![batch_txn];
        result.extend(deposited_txns);
        Ok(result)
    }

    fn transaction_parser(&self) -> &SequencingTransactionParser {
        &self.transaction_parser
    }
}

impl OptimismBlockBuilder {
    /// Creates a new Optimism block builder
    pub const fn new(sequencing_contract_address: Address) -> Self {
        let transaction_parser = SequencingTransactionParser::new(sequencing_contract_address);
        Self { transaction_parser }
    }

    async fn process_deposited_txns(
        &self,
        _txns: Vec<BlockAndReceiptsPointer>,
    ) -> Result<Vec<TransactionRequest>> {
        // TODO: Implement
        Ok(vec![])
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

    #[test]
    fn test_new_builder() {
        let sequencing_contract_address =
            Address::from_str("0x1234000000000000000000000000000000000000")
                .expect("Invalid address format");
        let builder = OptimismBlockBuilder::new(sequencing_contract_address);
        let parser = builder.transaction_parser();
        assert!(!std::ptr::eq(parser, std::ptr::null()), "Transaction parser should not be null");
    }

    #[tokio::test]
    async fn test_build_batch_txn() {
        let sequencing_contract_address =
            Address::from_str("0x1234000000000000000000000000000000000000")
                .expect("Invalid address format");
        let builder = OptimismBlockBuilder::new(sequencing_contract_address);
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
