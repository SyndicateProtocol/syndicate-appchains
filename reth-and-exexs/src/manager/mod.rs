use reth_primitives::Address;
use reth_provider::Chain;
use reth_tracing::tracing::{debug, info, warn};
use serde_json::json;

use crate::l3_block::process_chain_into_sequencer_l3blocks;

pub struct Manager {
    sequencer_address: Address,
    latest_sequencing_block: u64,
}

impl Manager {
    pub fn new(sequencer_address: Address) -> Self {
        Manager {
            sequencer_address,
            latest_sequencing_block: 0,
        }
    }

    /// Process a new chain commit.
    ///
    /// This function decodes all transactions to the sequencer contract into events, executes the
    /// corresponding actions and inserts the results into the database.
    pub async fn commit(&mut self, chain: &Chain) -> eyre::Result<()> {
        let sequencer_blocks = process_chain_into_sequencer_l3blocks(self.sequencer_address, chain);
        let tx_found = !sequencer_blocks.is_empty();
        for (block, _tx, seq_block) in sequencer_blocks {
            info!("Found L3 block in transaction input: {:?}", seq_block);

            info!(
                target: "based_sequencer_chain",
                event = "tx_found",
                tx_hash = seq_block.parent_hash.to_string(),
                block_number = block.number,
                sequencer_address = ?self.sequencer_address,
                "Block contains an event from processing a new batch on the BasedSequencerChain"
            );

            // TODO [SEQ-29]: L3 Block -> ExecutionPayload

            // TODO: Call engine_NewPayload

            // TODO: Call engine_ForkchoiceUpdated
        }

        // Log stats in a structured format
        debug!(
            target: "based_sequencer_chain_stats",
            json = json!({
                "latest_block_number": chain.tip().number,
                "blocks_in_commit": chain.len(),
                "contains_tx": tx_found,
                "sequencer_address": self.sequencer_address,
                "latest_sequencing_block": self.latest_sequencing_block,
            }).to_string()
        );

        Ok(())
    }

    /// Process a chain revert.
    ///
    /// This function decodes all transactions to the sequencer contract into events, reverts the
    /// corresponding actions and updates the database.
    pub fn revert(&mut self, chain: &Chain) -> eyre::Result<()> {
        let mut sequencer_blocks =
            process_chain_into_sequencer_l3blocks(self.sequencer_address, chain);
        // Reverse the order of events to start reverting from the tip
        sequencer_blocks.reverse();

        for (block, tx, _seq_block) in sequencer_blocks {
            self.latest_sequencing_block = block.number;
            warn!(
                target: "based_sequencer_chain",
                event = "chain_revert",
                reverted_chain = ?chain.range(),
                tx_hash = %tx.hash,
                "Chain reverted",
            );
            // TODO revert things
        }

        warn!(
            target: "based_sequencer_chain",
            event = "chain_revert",
            reverted_chain = ?chain.range(),
            "Chain reverted"
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use reth_primitives::{Address, Block, Header, Transaction, TransactionSigned, TxEip4844};
    use reth_provider::{Chain, ExecutionOutcome};
    use reth_testing_utils::generators::sign_tx_with_random_key_pair;

    use super::*;

    fn sequencer_address() -> Address {
        "0x0000000000000000000000000000000000000000"
            .to_string()
            .parse::<Address>()
            .unwrap()
    }

    async fn get_chain(head_num: Option<u64>, tx: Option<Transaction>) -> eyre::Result<Chain> {
        let num = head_num.unwrap_or(0);
        let transaction = tx.unwrap_or_else(|| {
            Transaction::Eip4844(TxEip4844 {
                to: sequencer_address(),
                ..Default::default()
            })
        });

        // Sign the transaction
        let transaction_signed = sign_tx_with_random_key_pair(&mut rand::thread_rng(), transaction);

        let block = Block {
            header: Header {
                number: num + 1,
                ..Default::default()
            },
            body: vec![transaction_signed],
            ..Default::default()
        }
        .seal_slow()
        .seal_with_senders()
        .ok_or_else(|| eyre::eyre!("failed to recover senders"))?;

        Ok(Chain::from_block(block, ExecutionOutcome::default(), None))
    }

    fn create_block(number: u64, transactions: Vec<TransactionSigned>) -> Block {
        Block {
            header: Header {
                number,
                ..Default::default()
            },
            body: transactions,
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_manager_revert() -> eyre::Result<()> {
        let chain = get_chain(Some(0), None).await?;

        let mut manager = Manager::new(sequencer_address());
        manager.revert(&chain)?;

        Ok(())
    }

    #[tokio::test]
    async fn test_manager_commit_empty_chain() -> eyre::Result<()> {
        let chain = get_chain(None, None).await?;
        let mut manager = Manager::new(sequencer_address());
        manager.commit(&chain).await?;
        assert_eq!(manager.latest_sequencing_block, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_commit_single_block_no_relevant_tx() -> eyre::Result<()> {
        let block = create_block(1, vec![]);
        let chain = get_chain(Some(block.header.number), None).await?;
        let mut manager = Manager::new(sequencer_address());
        manager.commit(&chain).await?;
        assert_eq!(manager.latest_sequencing_block, 0);
        Ok(())
    }
}
