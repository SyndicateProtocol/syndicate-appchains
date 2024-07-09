use reth_primitives::Address;
use reth_provider::Chain;
use reth_tracing::tracing::{debug, info, warn};
use serde_json::json;

use crate::l3_block;
use l3_block::L3Block;

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
    /// This function decodes all transactions to the rollup contract into events, executes the
    /// corresponding actions and inserts the results into the database.
    pub async fn commit(&mut self, chain: &Chain) -> eyre::Result<()> {
        for block in chain.blocks_iter() {
            let block_number = block.header.number;
            let mut tx_found = false;
            if let Some(tx) = block.transactions().find(|tx| {
                tx.to() == Some(self.sequencer_address)
                // TODO: Add check against function signature as well
            }) {
                self.latest_sequencing_block = block_number;
                tx_found = true;
                info!(
                    target: "based_sequencer_chain",
                    event = "tx_found",
                    tx_hash = ?tx.hash(),
                    block_number = ?block_number,
                    sequencer_address = ?self.sequencer_address,
                    "Block contains a tx to the BasedSequencerChain address"
                );

                if let Some(l3_block) = L3Block::new(tx.input()) {
                    info!("Found L3 block in transaction input: {:?}", l3_block);
                }

                // TODO [SEQ-29]: L3 Block -> ExecutionPayload

                // TODO: Call engine_NewPayload

                // TODO: Call engine_ForkchoiceUpdated
            } else {
                // We can avoid logging this in production since it will
                // be very high frequency. We can also get it implicitly
                // by looking at block numbers that do not have an
                // `info` log associated with them
                debug!(
                    target: "based_sequencer_chain",
                    event = "no_tx_found",
                    block_number = ?block_number,
                    sequencer_address = ?self.sequencer_address,
                    "Block does not contain a tx to the BasedSequencerChain address"
                );
            }

            // Log stats in a structured format
            debug!(
                target: "based_sequencer_chain_stats",
                json = json!({
                    "block_number": block_number,
                    "contains_tx": tx_found,
                    "sequencer_address": self.sequencer_address,
                    "latest_sequencing_block": self.latest_sequencing_block,
                }).to_string()
            );
        }

        Ok(())
    }

    /// Process a chain revert.
    ///
    /// This function decodes all transactions to the rollup contract into events, reverts the
    /// corresponding actions and updates the database.
    pub fn revert(&mut self, chain: &Chain) -> eyre::Result<()> {
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
    use super::*;
    use reth_primitives::{Block, Header, Transaction, TxEip4844};
    use reth_provider::{Chain, ExecutionOutcome};
    use reth_testing_utils::generators::sign_tx_with_random_key_pair;

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

    #[tokio::test]
    async fn test_manager_revert() -> eyre::Result<()> {
        let chain = get_chain(Some(0), None).await?;

        let mut manager = Manager::new(sequencer_address());
        manager.revert(&chain)?;

        Ok(())
    }
}
