//! Arbitrum block builder implementation
//!
//! This module provides functionality for building an Arbitrum batch submitter transaction from a
//! list of transactions. It implements the [`RollupBlockBuilder`] trait to standardize block
//! construction across different rollup implementations

use crate::rollups::shared::{RollupBlockBuilder, SequencingTransactionParser};
use alloy::{
    primitives::{Address, Bytes},
    rpc::types::TransactionRequest,
};
use async_trait::async_trait;
use eyre::{Error, Result};

#[derive(Debug)]
/// Builder for constructing Arbitrum blocks from transactions
pub struct ArbitrumBlockBuilder {
    transaction_parser: SequencingTransactionParser,
}

#[async_trait]
impl RollupBlockBuilder for ArbitrumBlockBuilder {
    /// Creates a new Arbitrum block builder.
    ///
    /// # Arguments
    /// - `sequencing_contract_address`: The address of the sequencing contract to monitor.
    fn new(sequencing_contract_address: Address) -> Self {
        let transaction_parser = SequencingTransactionParser::new(sequencing_contract_address);
        Self { transaction_parser }
    }

    fn transaction_parser(&self) -> &SequencingTransactionParser {
        &self.transaction_parser
    }

    /// Builds a batch of transactions into an Arbitrum batch
    async fn build_batch_txn(&self, _txs: Vec<Bytes>) -> Result<TransactionRequest, Error> {
        // TODO: Implement
        Ok(TransactionRequest::default())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_new_builder() {
        let sequencing_contract_address =
            Address::from_str("0x1234000000000000000000000000000000000000")
                .expect("Invalid address format");
        let builder = ArbitrumBlockBuilder::new(sequencing_contract_address);
        let parser = builder.transaction_parser();
        assert!(!std::ptr::eq(parser, std::ptr::null()), "Transaction parser should not be null");
    }

    #[tokio::test]
    async fn test_build_batch_with_txs() {
        let sequencing_contract_address =
            Address::from_str("0x1234000000000000000000000000000000000000")
                .expect("Invalid address format");
        let builder = ArbitrumBlockBuilder::new(sequencing_contract_address);
        let txs = vec![];
        let batch = builder.build_batch_txn(txs).await.unwrap();
        // Currently just verifies it returns default request
        // TODO: Update test when build_batch_txn is implemented
        assert_eq!(batch, TransactionRequest::default());
    }
}
