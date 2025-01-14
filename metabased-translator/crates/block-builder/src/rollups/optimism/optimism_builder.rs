use crate::rollups::optimism::batch::{new_batcher_tx, Batch};
use crate::rollups::optimism::frame::to_data;
use crate::rollups::utils::BlockBuilder;
use alloy_primitives::{Address, B256};
use alloy_rpc_types::TransactionRequest;
use std::str::FromStr;

#[derive(Debug)]
/// Builder for constructing Optimism blocks from transactions
pub struct OptimismBlockBuilder;

impl BlockBuilder for OptimismBlockBuilder {
    /// Creates a new Optimism block builder
    fn new() -> Self {
        Self
    }

    /// Builds a batch of transactions into an Optimism batch
    fn build_batch_txn(&self, txs: Vec<Vec<u8>>) -> TransactionRequest {
        // TODO: Implement
        let batcher = Address::from_str("0x063D87A885a9323831A688645647eD7d0e859C5d")
            .expect("Failed to parse batcher address");
        let batch_inbox = Address::from_str("0x97395dd253e2d096a0caa62a574895c3c2f2b2e0")
            .expect("Failed to parse Batch Inbox address");
        let hash =
            B256::from_str("0xe009262cd1adf34cfaf845fd1c17a6ddb7f97c67b2992cd9f286ff4e1c6ad233")
                .unwrap();

        let single_batch = Batch {
            parent_hash: hash,
            epoch_num: 0,
            epoch_hash: hash,
            timestamp: 1712500002,
            transactions: txs,
        };
        let frames = single_batch.get_frames(1000000).unwrap();
        let data = to_data(&frames).unwrap();

        let tx = new_batcher_tx(batcher, batch_inbox, data.into());
        tx
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

    #[test]
    fn test_build_batch_txn() {
        let builder = OptimismBlockBuilder::new();
        let txs = vec![];

        let tx = builder.build_batch_txn(txs);

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
