//! The `synd-appchain-verifier` crate is responsible for verifying a batch of blocks and creating a
//! new `mchain` block

use crate::{
    config::AppchainVerifierConfig,
    errors::VerifierError,
    types::{
        get_current_messages, get_input_batches_with_timestamps, BlockVerifierInput,
        SequencingChainInput, SettlementChainInput,
    },
};
use eyre::Result;

/// The `Verifier` struct is responsible for verifying a batch of blocks and creating a new mchain
/// block.
#[derive(Default, Debug, Clone)]
pub struct Verifier {
    /// Settlement delay
    _settlement_delay: u64,
}

impl Verifier {
    /// Create a new `Verifier`
    pub const fn new(config: &AppchainVerifierConfig) -> Self {
        Self { _settlement_delay: config.settlement_delay }
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
        let batches = get_input_batches_with_timestamps(sequencing_chain_input)?;
        let delayed_messages = &settlement_chain_input.delayed_messages;
        let mut block_verifier_inputs = vec![];
        for batch in batches {
            let messages = get_current_messages(delayed_messages.clone(), batch.timestamp);
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
}
