//! The `synd-appchain-verifier` crate is responsible for verifying a batch of blocks and creating a
//! new `mchain` block

use crate::{
    config::AppchainVerifierConfig,
    errors::VerifierError,
    types::{
        BlockVerifierInput, L1IncomingMessage as DelayedMessage, SequencingChainInput,
        SettlementChainInput, SyndicateTransaction,
    },
};
use alloy::primitives::Bytes;
use eyre::Result;
use synd_block_builder::appchains::arbitrum::batch::{
    Batch, BatchMessage, L1IncomingMessage, L1IncomingMessageHeader,
};

/// The `Verifier` struct is responsible for verifying a batch of blocks and creating a new mchain
/// block.
#[derive(Default, Debug, Clone)]
pub struct Verifier {
    /// Settlement delay
    _settlement_delay: u64,
}

/// A batch with a timestamp
#[derive(Default, Debug, Clone)]
pub struct BatchWithTimestamp {
    /// Timestamp
    pub timestamp: u64,
    /// Batch
    pub batch: Bytes,
}

impl Verifier {
    /// Create a new `Verifier`
    pub const fn new(config: &AppchainVerifierConfig) -> Self {
        Self { _settlement_delay: config.settlement_delay }
    }

    const fn build_delayed_messages(
        &self,
        delayed_messages: Vec<DelayedMessage>,
    ) -> Result<Vec<DelayedMessage>, VerifierError> {
        Ok(delayed_messages)
    }

    /// Verifies blocks and receipts and creates a new mchain block
    pub fn verify_and_create_output(
        &self,
        sequencing_chain_input: &SequencingChainInput,
        settlement_chain_input: &SettlementChainInput,
    ) -> Result<Vec<BlockVerifierInput>, VerifierError> {
        // Validate Settlement Chain Input
        settlement_chain_input.validate()?;

        // Validate Sequencing Chain Input
        sequencing_chain_input.validate()?;

        // Generate output
        self.generate_output(sequencing_chain_input, settlement_chain_input)
    }

    // --------------------------------------------
    // Output Generation
    // --------------------------------------------
    #[allow(clippy::unwrap_used)]
    fn generate_output(
        &self,
        sequencing_chain_input: &SequencingChainInput,
        settlement_chain_input: &SettlementChainInput,
    ) -> Result<Vec<BlockVerifierInput>, VerifierError> {
        let batches = self.build_batch_txns(&sequencing_chain_input.syndicate_transactions)?;
        let delayed_messages =
            self.build_delayed_messages(settlement_chain_input.delayed_messages.clone())?;

        let mut block_verifier_inputs = vec![];
        for batch in batches {
            let messages = Self::get_current_messages(delayed_messages.clone(), batch.timestamp);
            let block_verifier_input = BlockVerifierInput {
                min_block_number: 0,
                max_block_number: u64::MAX,
                min_timestamp: 0,
                max_timestamp: u64::MAX,
                messages,
                batch: batch.batch,
            };
            block_verifier_inputs.push(block_verifier_input);
        }
        Ok(block_verifier_inputs)
    }

    fn get_current_messages(
        _delayed_messages: Vec<DelayedMessage>,
        _timestamp: u64,
    ) -> Vec<DelayedMessage> {
        // TODO: Implement this
        // delayed_messages
        //     .iter()
        //     .filter(|message| message.header.timestamp == timestamp)
        //     .map(|message| message.clone())
        //     .collect()

        vec![]
    }

    // --------------------------------------------
    // Batch Generation
    // --------------------------------------------
    /// Builds a batch of transactions into an Arbitrum batch
    fn build_batch(txs: Vec<Bytes>, block_number: u64, timestamp: u64) -> Result<Bytes> {
        let mut messages = vec![];
        if !txs.is_empty() {
            messages.push(BatchMessage::L2(L1IncomingMessage {
                header: L1IncomingMessageHeader { block_number, timestamp },
                l2_msg: txs,
            }));
        };

        let batch = Batch(messages);
        let encoded_batch = batch.encode()?;
        Ok(encoded_batch)
    }

    fn build_batch_txns(
        &self,
        txs_payloads: &Vec<SyndicateTransaction>,
    ) -> Result<Vec<BatchWithTimestamp>, VerifierError> {
        let mut batches = vec![];
        let mut slot = vec![];
        let mut block_number = txs_payloads[0].block_number;
        let mut timestamp = txs_payloads[0].timestamp;
        for tx in txs_payloads {
            if tx.block_number == block_number {
                slot.push(tx.payload.clone());
            } else {
                let batch = BatchWithTimestamp {
                    timestamp,
                    batch: Self::build_batch(slot, block_number, timestamp)?,
                };
                batches.push(batch);
                slot = vec![tx.payload.clone()];
                block_number = tx.block_number;
                timestamp = tx.timestamp;
            }
        }

        Ok(batches)
    }
}
