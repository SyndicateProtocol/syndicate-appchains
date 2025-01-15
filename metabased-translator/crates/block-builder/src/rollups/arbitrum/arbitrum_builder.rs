use crate::rollups::rollup_builder::RollupBlockBuilder;
use alloy_rpc_types::TransactionRequest;
use async_trait::async_trait;
use eyre::{Error, Result};

#[derive(Debug)]
/// Builder for constructing Arbitrum blocks from transactions
pub struct ArbitrumBlockBuilder;

#[async_trait]
impl RollupBlockBuilder for ArbitrumBlockBuilder {
    /// Creates a new Arbitrum block builder
    fn new() -> Self {
        Self
    }

    /// Builds a batch of transactions into an Arbitrum batch
    async fn build_batch_txn(&self, _txs: Vec<Vec<u8>>) -> Result<TransactionRequest, Error> {
        // TODO: Implement
        Ok(TransactionRequest::default())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_builder() {
        let builder = ArbitrumBlockBuilder::new();
        assert!(matches!(builder, ArbitrumBlockBuilder));
    }

    #[tokio::test]
    async fn test_build_batch_with_txs() {
        let builder = ArbitrumBlockBuilder::new();
        let txs = vec![];
        let batch = builder.build_batch_txn(txs).await.unwrap();
        // Currently just verifies it returns default request
        // TODO: Update test when build_batch_txn is implemented
        assert_eq!(batch, TransactionRequest::default());
    }
}
