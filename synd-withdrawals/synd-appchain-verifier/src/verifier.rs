//! The `synd-appchain-verifier` crate verifies batches of blocks from both the sequencing and
//! settlement chains, producing `BlockVerifierInput` objects for further processing.

use crate::{
    config::AppchainVerifierConfig,
    errors::VerifierError,
    types::{
        get_input_batches_with_timestamps, BlockVerifierInput, L1IncomingMessage,
        L1IncomingMessageHeader, SequencingChainInput, SettlementChainInput,
    },
};
use alloy::primitives::{Address, U256};
use eyre::Result;
use synd_block_builder::appchains::arbitrum::arbitrum_adapter::{ArbitrumAdapter, L1MessageType};

/// The `Verifier` struct
#[derive(Default, Debug, Clone)]
pub struct Verifier {
    /// Sequencing chain contract address
    sequencing_chain_contract_address: Address,

    /// Settlement delay
    settlement_delay: u64,
}

impl Verifier {
    /// Constructs a new [`Verifier`] from the provided config.
    pub const fn new(config: &AppchainVerifierConfig) -> Self {
        Self {
            sequencing_chain_contract_address: config.sequencing_contract_address,
            settlement_delay: config.settlement_delay,
        }
    }

    /// Verifies both the sequencing and settlement chains and creates an array of
    /// [`BlockVerifierInput`] objects.
    pub fn verify_and_create_output(
        &self,
        sequencing_chain_input: &SequencingChainInput,
        settlement_chain_input: &SettlementChainInput,
    ) -> Result<Vec<BlockVerifierInput>, VerifierError> {
        settlement_chain_input.validate()?;
        sequencing_chain_input.validate(self.sequencing_chain_contract_address)?;
        self.generate_output(sequencing_chain_input, settlement_chain_input)
    }

    fn process_delayed_message(&self, msg: L1IncomingMessage) -> L1IncomingMessage {
        assert!(
            msg.header.kind != L1MessageType::Initialize as u8,
            "Initialize message received. This should not happen."
        );
        if ArbitrumAdapter::should_ignore_delayed_message(&L1MessageType::from_u8_panic(
            msg.header.kind,
        )) {
            L1IncomingMessage {
                header: L1IncomingMessageHeader {
                    kind: L1MessageType::EndOfBlock as u8,
                    sender: Address::ZERO,
                    block_number: 0,
                    timestamp: 0,
                    request_id: msg.header.request_id,
                    base_fee_l1: U256::ZERO,
                },
                l2msg: Default::default(),
            }
        } else {
            msg
        }
    }

    fn generate_output(
        &self,
        sequencing_chain_input: &SequencingChainInput,
        settlement_chain_input: &SettlementChainInput,
    ) -> Result<Vec<BlockVerifierInput>, VerifierError> {
        let batches_with_timestamp = get_input_batches_with_timestamps(sequencing_chain_input)?;
        let delayed_messages = settlement_chain_input
            .delayed_messages
            .clone()
            .into_iter()
            .map(|m| self.process_delayed_message(m))
            .collect::<Vec<_>>();

        let start_ts = batches_with_timestamp[0].timestamp;
        let mut delayed_messages_index = 0;
        if delayed_messages[delayed_messages_index].header.timestamp + self.settlement_delay >
            start_ts
        {
            return Err(VerifierError::InvalidSettlementChainInputWithReason {
                reason: "Delayed message timestamp is greater than the start timestamp".to_string(),
            });
        }
        while delayed_messages[delayed_messages_index].header.timestamp + self.settlement_delay <=
            start_ts
        {
            delayed_messages_index += 1;
        }

        let mut block_verifier_inputs = vec![];
        for batch_with_timestamp in &batches_with_timestamp[1..] {
            let mut messages = vec![];
            while delayed_messages[delayed_messages_index].header.timestamp + self.settlement_delay <=
                batch_with_timestamp.timestamp
            {
                messages.push(delayed_messages[delayed_messages_index].clone());
                delayed_messages_index += 1;
            }
            let batch = batch_with_timestamp.batch.clone();

            // If there are no sequencing transactions or delayed messages, skip this batch
            if batch.is_empty() && messages.is_empty() {
                continue;
            }

            let block_verifier_input = BlockVerifierInput {
                min_block_number: 0,
                max_block_number: u64::MAX,
                min_timestamp: 0,
                max_timestamp: u64::MAX,
                messages,
                batch,
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
        let config = AppchainVerifierConfig {
            sequencing_contract_address: Address::ZERO,
            settlement_delay: 10,
        };
        let verifier = Verifier::new(&config);

        assert_eq!(verifier.settlement_delay, 10);
    }
}
