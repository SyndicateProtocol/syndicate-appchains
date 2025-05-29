//! The `synd-seqchain-verifier` crate verifies batches of blocks from the L1 chain,
//! producing `BlockVerifierInput` objects for further processing.

use crate::{
    config::SeqchainVerifierConfig,
    errors::VerifierError,
    types::{BlockVerifierInput, L1ChainInput},
};
use alloy::primitives::{Address, U256};
use eyre::Result;

/// The `Verifier` struct
#[derive(Default, Debug, Clone)]
pub struct Verifier {
    /// Arbitrum contract address
    arbitrum_bridge_address: Address,
}

impl Verifier {
    /// Constructs a new [`Verifier`] from the provided config.
    pub const fn new(config: &SeqchainVerifierConfig) -> Self {
        Self { arbitrum_bridge_address: config.arbitrum_bridge_address }
    }

    /// Verifies both the sequencing and settlement chains and creates an array of
    /// [`BlockVerifierInput`] objects.
    pub fn verify_and_create_output(
        &self,
        l1_chain_input: &L1ChainInput,
    ) -> Result<Vec<BlockVerifierInput>, VerifierError> {
        l1_chain_input.validate(self.arbitrum_bridge_address)?;
        self.generate_output(l1_chain_input)
    }

    fn generate_output(
        &self,
        l1_chain_input: &L1ChainInput,
    ) -> Result<Vec<BlockVerifierInput>, VerifierError> {
        let batches = l1_chain_input.batches.clone();
        let delayed_messages = l1_chain_input.delayed_messages.clone();

        let mut block_verifier_inputs = vec![];
        for batch in &batches[1..] {
            let mut messages = vec![];
            for delayed_message in &delayed_messages {
                let message_num: U256 = delayed_message.header.request_id.into();
                if message_num <= batch.after_delayed_messages_read {
                    messages.push(delayed_message.clone());
                }
            }

            let block_verifier_input = BlockVerifierInput {
                min_block_number: batch.time_bounds.min_block_number,
                max_block_number: batch.time_bounds.max_block_number,
                min_timestamp: batch.time_bounds.min_timestamp,
                max_timestamp: batch.time_bounds.max_timestamp,
                messages,
                batch: batch.data.clone(),
            };

            block_verifier_inputs.push(block_verifier_input);
        }
        Ok(block_verifier_inputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifier_creation() {
        let config = SeqchainVerifierConfig { arbitrum_bridge_address: Address::ZERO };
        let _verifier = Verifier::new(&config);
    }
}
