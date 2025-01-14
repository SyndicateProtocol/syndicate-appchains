use crate::rollups::utils::BlockBuilder;
use alloy_rpc_types::TransactionRequest;

#[derive(Debug)]
/// Builder for constructing Arbitrum blocks from transactions
pub struct ArbitrumBlockBuilder;

impl BlockBuilder for ArbitrumBlockBuilder {
    /// Creates a new Arbitrum block builder
    fn new() -> Self {
        Self
    }

    /// Builds a batch of transactions into an Arbitrum batch
    fn build_batch_txn(&self, _txs: Vec<Vec<u8>>) -> TransactionRequest {
        // TODO: Implement
        TransactionRequest::default()
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

    #[test]
    fn test_build_batch_with_txs() {
        let builder = ArbitrumBlockBuilder::new();
        let txs = vec![];
        let batch = builder.build_batch_txn(txs);
        // Currently just verifies it returns default request
        // TODO: Update test when build_batch_txn is implemented
        assert_eq!(batch, TransactionRequest::default());
    }
}
