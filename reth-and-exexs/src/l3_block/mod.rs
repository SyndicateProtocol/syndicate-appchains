use alloy::primitives::{Bytes, FixedBytes, U256, U64};
use alloy_sol_types::{sol, SolEventInterface};
use reth_primitives::{transaction::TransactionSigned, Address, SealedBlockWithSenders};
use reth_provider::Chain;

sol!(
    BasedSequencerChain,
    "../contracts/out/BasedSequencerChain.sol/BasedSequencerChain.json"
);
use BasedSequencerChain::{BasedSequencerChainEvents, LatestBatchProcessed};

#[derive(Debug, Clone)]
pub struct L3Block {
    pub parent_hash: FixedBytes<32>,
    pub epoch_number: U256,
    pub timestamp: U64,
    pub transaction_list: Vec<Bytes>,
}

impl From<LatestBatchProcessed> for L3Block {
    fn from(batch: LatestBatchProcessed) -> Self {
        L3Block {
            parent_hash: batch.parent_hash,
            epoch_number: batch.epoch_number,
            timestamp: U64::from(0),
            transaction_list: batch.transaction_list,
        }
    }
}

/// Process chain of blocks into a flattened list of receipt logs, filter only transactions to the
/// sequencer contract, extract the sequencer event and parse into an [L3Block].
pub fn process_chain_into_sequencer_l3blocks(
    filter_address: Address,
    chain: &Chain,
) -> Vec<(&SealedBlockWithSenders, &TransactionSigned, L3Block)> {
    chain
        // Get all blocks and receipts
        .blocks_and_receipts()
        // Get all receipts
        .flat_map(|(block, receipts)| {
            block
                .body
                .iter()
                .zip(receipts.iter().flatten())
                .map(move |(tx, receipt)| (block, tx, receipt))
        })
        // Get all logs from sequencer contract
        .flat_map(|(block, tx, receipt)| {
            receipt
                .logs
                .iter()
                .filter(|log| log.address == filter_address)
                .map(move |log| (block, tx, log))
        })
        // Decode and filter sequencer events
        .filter_map(|(block, tx, log)| {
            BasedSequencerChainEvents::decode_raw_log(log.topics(), &log.data.data, true)
                .ok()
                .map(|event| (block, tx, event))
        })
        // Filter to LatestBatchProcessed event and pull out batch
        .filter_map(|(block, tx, event)| match event {
            BasedSequencerChainEvents::LatestBatchProcessed(batch) => {
                Some((block, tx, L3Block::from(batch)))
            }
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::Uint;

    #[tokio::test]
    async fn test_parse_l3_block() {
        // Create a sample transaction data
        let parent_hash = FixedBytes::from([1u8; 32]);
        let epoch_hash = FixedBytes::from([1u8; 32]);
        let parent_epoch_number = Uint::from(1);
        let epoch_number = Uint::from(2);
        let transaction_list = vec![Bytes::from(vec![1, 2, 3])];
        let latest_batch = BasedSequencerChain::LatestBatchProcessed {
            parent_hash,
            parent_epoch_number,
            epoch_number,
            epoch_hash,
            transaction_list: transaction_list.clone(),
        };

        // Parse the L3 block
        let parsed_block = L3Block::from(latest_batch);

        // Check the parsed block fields
        assert_eq!(parsed_block.parent_hash, parent_hash);
        assert_eq!(parsed_block.epoch_number, epoch_number);
        assert_eq!(parsed_block.transaction_list, transaction_list);
        assert_eq!(parsed_block.timestamp.to::<u64>(), 0);
    }
}
