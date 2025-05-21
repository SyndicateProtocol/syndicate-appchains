//! The `synd-appchain-verifier` crate is responsible for verifying a batch of blocks and creating a
//! new `mchain` block

use crate::{
    config::AppchainVerifierConfig,
    errors::VerifierError,
    types::{SequencingChainInput, SettlementChainInput, VerifierOutput},
};
use eyre::Result;
use std::collections::HashSet;
use synd_block_builder::appchains::{
    arbitrum::arbitrum_adapter::ArbitrumAdapter,
    shared::sequencing_transaction_parser::SequencingTransactionParser,
};
use tracing::debug;

/// The `Verifier` struct is responsible for verifying a batch of blocks and creating a new mchain
/// block.
#[derive(Default, Debug, Clone)]
pub struct Verifier {
    /// The adapter for the sequencing chain
    _arbitrum_adapter: ArbitrumAdapter,
    /// Settlement delay
    _settlement_delay: u64,
}

impl Verifier {
    /// Create a new `Verifier`
    pub fn new(config: &AppchainVerifierConfig) -> Self {
        Self {
            _arbitrum_adapter: ArbitrumAdapter {
                transaction_parser: SequencingTransactionParser::new(
                    config.sequencing_contract_address,
                ),
                bridge_address: config.arbitrum_bridge_address,
                inbox_address: config.arbitrum_inbox_address,
                ignore_delayed_messages: config.arbitrum_ignore_delayed_messages,
                allowed_settlement_addresses: config
                    .allowed_settlement_addresses
                    .iter()
                    .copied()
                    .collect::<HashSet<_>>(),
            },
            _settlement_delay: config.settlement_delay,
        }
    }

    /// Verifies blocks and receipts and creates a new mchain block
    pub fn verify_and_create_output(
        &self,
        sequencing_chain_input: &SequencingChainInput,
        settlement_chain_input: &SettlementChainInput,
    ) -> Result<VerifierOutput, VerifierError> {
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
    ) -> Result<VerifierOutput, VerifierError> {
        // TODO (SEQ-769): Implement Appchain Verifier Component
        // TODO: Implement output generation
        debug!("Generating output");
        debug!("Sequencing chain input {:?}", sequencing_chain_input);
        debug!("Settlement chain input {:?}", settlement_chain_input);
        Ok(VerifierOutput { block_verifier_inputs: vec![], batch_count: 0 })
    }
}
