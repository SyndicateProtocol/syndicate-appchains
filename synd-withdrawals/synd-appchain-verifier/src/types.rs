//! Types for the `synd-appchain-verifier`

use crate::errors::VerifierError;
use alloy::{
    primitives::{fixed_bytes, keccak256, map::HashMap, Address, Bytes, B256},
    rpc::types::{EIP1186AccountProofResponse, Header},
    sol_types::SolValue as _,
};
use alloy_trie::{proof::verify_proof, Nibbles, TrieAccount};
use serde::{Deserialize, Serialize};
use synd_block_builder::appchains::{
    arbitrum::batch::{
        Batch, BatchMessage, L1IncomingMessage as ArbL1IncomingMessage,
        L1IncomingMessageHeader as ArbL1IncomingMessageHeader,
    },
    shared::SequencingTransactionParser,
};
use withdrawals_shared::types::L1IncomingMessage;

const SYNDICATE_ACCUMULATOR_STORAGE_SLOT: B256 =
    fixed_bytes!("0x847fe1a0bfd701c2dbb0b62670ad8712eed4c0ff4d2c6c0917f4c8d260ed0b90"); // Keccak256("syndicate.accumulator")

/// Settlement chain input
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SettlementChainInput {
    // TRUSTLESS INPUT
    /// Delayed messages
    pub delayed_messages: Vec<L1IncomingMessage>,
    /// Start delayed messages accumulator
    pub start_delayed_messages_accumulator: B256,

    // TRUSTED INPUT
    /// End delayed messages accumulator
    pub end_delayed_messages_accumulator: B256,
}

impl SettlementChainInput {
    /// Validate the settlement chain input
    pub fn validate(&self) -> Result<(), VerifierError> {
        let mut acc = self.start_delayed_messages_accumulator;
        for delayed_message in &self.delayed_messages {
            acc = delayed_message.accumulate(acc);
        }
        if acc != self.end_delayed_messages_accumulator {
            return Err(VerifierError::InvalidSettlementChainInput {
                reason: "Invalid end delayed messages accumulator".to_string(),
                expected: self.end_delayed_messages_accumulator.to_string(),
                actual: acc.to_string(),
            });
        }
        Ok(())
    }
}
/// Sequencing chain input
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SequencingChainInput {
    // TRUSTLESS INPUT
    /// Start syndicate accumulator merkle proof
    pub start_syndicate_accumulator_merkle_proof: EIP1186AccountProofResponse,
    /// End syndicate accumulator merkle proof
    pub end_syndicate_accumulator_merkle_proof: EIP1186AccountProofResponse,
    /// Syndicate transactions
    pub syndicate_transaction_events: Vec<SyndicateTransactionEvent>,
    /// Block headers
    pub block_headers: Vec<Header>,

    // TRUSTED INPUT
    /// Start block hash
    pub start_block_hash: B256,
    /// End block hash
    pub end_block_hash: B256,
}

impl SequencingChainInput {
    fn verify_accumulator(&self) -> Result<(), VerifierError> {
        let mut acc = self.start_syndicate_accumulator_merkle_proof.storage_proof[0].value.into();
        for syndicate_transaction in &self.syndicate_transaction_events {
            acc = syndicate_transaction.accumulate(acc);
        }
        let expected_end_accumulator: B256 =
            self.end_syndicate_accumulator_merkle_proof.storage_proof[0].value.into();
        if acc != expected_end_accumulator {
            return Err(VerifierError::InvalidSequencingChainInput {
                reason: "Invalid end accumulator".to_string(),
                expected: expected_end_accumulator.to_string(),
                actual: acc.to_string(),
            });
        }
        Ok(())
    }

    fn verify_account_proof_response(
        proof: &EIP1186AccountProofResponse,
        state_root: B256,
    ) -> Result<(), VerifierError> {
        let key: Nibbles = Nibbles::unpack(keccak256(proof.address));
        let expected_value = alloy::rlp::encode(TrieAccount {
            nonce: proof.nonce,
            balance: proof.balance,
            storage_root: proof.storage_hash,
            code_hash: proof.code_hash,
        });
        verify_proof(state_root, key, Some(expected_value), &proof.account_proof)
            .map_err(|e| VerifierError::ErrorVerifyingProof(e.to_string()))?;
        for p in &proof.storage_proof {
            let k = Nibbles::unpack(keccak256(p.key.as_b256()));
            let expected_value = Some(alloy::rlp::encode_fixed_size(&p.value).to_vec());
            verify_proof(proof.storage_hash, k, expected_value, &p.proof)
                .map_err(|e| VerifierError::ErrorVerifyingProof(e.to_string()))?;
        }
        Ok(())
    }

    fn verify_merkle_proof(
        proof: &EIP1186AccountProofResponse,
        header: &Header,
        block_hash: B256,
        sequencing_chain_contract_address: Address,
        slot: B256,
    ) -> Result<(), VerifierError> {
        // Verify header
        let actual_block_hash = header.hash_slow();
        if actual_block_hash != block_hash {
            return Err(VerifierError::InvalidSequencingChainInput {
                reason: "Invalid block hash".to_string(),
                expected: block_hash.to_string(),
                actual: actual_block_hash.to_string(),
            });
        }
        // Verify end syndicate accumulator merkle proof
        Self::verify_account_proof_response(proof, header.state_root)?;

        // Verify there is only one storage proof
        if proof.storage_proof.len() != 1 {
            return Err(VerifierError::InvalidSequencingChainInput {
                reason: "Invalid number of storage proofs".to_string(),
                expected: "1".to_string(),
                actual: proof.storage_proof.len().to_string(),
            });
        }
        // Verify storage slot
        let storage_proof = &proof.storage_proof[0];

        if storage_proof.key.as_b256() != slot {
            return Err(VerifierError::InvalidSequencingChainInput {
                reason: "Invalid storage slot".to_string(),
                expected: slot.to_string(),
                actual: storage_proof.key.as_b256().to_string(),
            });
        }

        // Verify address
        if proof.address != sequencing_chain_contract_address {
            return Err(VerifierError::InvalidSequencingChainInput {
                reason: "Invalid address".to_string(),
                expected: sequencing_chain_contract_address.to_string(),
                actual: proof.address.to_string(),
            });
        }

        Ok(())
    }

    /// Verify the block headers
    #[allow(clippy::unwrap_used)]
    fn verify_block_headers(&self) -> Result<(), VerifierError> {
        if self.block_headers.is_empty() {
            return Err(VerifierError::InvalidSequencingChainInputWithReason {
                reason: "Empty block headers".to_string(),
            });
        }

        // Check that the first block hash matches the expected start hash
        let first_hash = self.block_headers.first().unwrap().hash_slow();
        if first_hash != self.start_block_hash {
            return Err(VerifierError::InvalidSequencingChainInput {
                reason: "Invalid start block hash".to_string(),
                expected: self.start_block_hash.to_string(),
                actual: first_hash.to_string(),
            });
        }

        // Verify each header’s parent_hash matches the previous block’s hash
        for window in self.block_headers.windows(2) {
            let prev = &window[0];
            let curr = &window[1];
            if curr.parent_hash != prev.hash_slow() {
                return Err(VerifierError::InvalidSequencingChainInput {
                    reason: "Invalid parent hash".to_string(),
                    expected: prev.hash_slow().to_string(),
                    actual: curr.parent_hash.to_string(),
                });
            }
        }

        // Check that the last block hash matches the expected end hash
        let last_hash = self.block_headers.last().unwrap().hash_slow();
        if last_hash != self.end_block_hash {
            return Err(VerifierError::InvalidSequencingChainInput {
                reason: "Invalid end block hash".to_string(),
                expected: self.end_block_hash.to_string(),
                actual: last_hash.to_string(),
            });
        }

        Ok(())
    }

    /// Validate the sequencing chain input
    #[allow(clippy::unwrap_used)]
    pub fn validate(
        &self,
        sequencing_chain_contract_address: Address,
    ) -> Result<(), VerifierError> {
        // Verify block headers
        self.verify_block_headers()?;
        // Verify accumulator
        self.verify_accumulator()?;
        // Validate start syndicate accumulator merkle proof
        Self::verify_merkle_proof(
            &self.start_syndicate_accumulator_merkle_proof,
            self.block_headers.first().unwrap(),
            self.start_block_hash,
            sequencing_chain_contract_address,
            SYNDICATE_ACCUMULATOR_STORAGE_SLOT,
        )?;
        // Validate  end syndicate accumulator merkle proof
        Self::verify_merkle_proof(
            &self.end_syndicate_accumulator_merkle_proof,
            self.block_headers.last().unwrap(),
            self.end_block_hash,
            sequencing_chain_contract_address,
            SYNDICATE_ACCUMULATOR_STORAGE_SLOT,
        )?;
        Ok(())
    }
}

/// Synd batch
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyndicateTransactionEvent {
    /// Block number
    pub block_number: u64,
    /// Timestamp
    pub timestamp: u64,
    /// Sender
    pub sender: Address,
    /// Payload
    pub payload: Bytes,
}

impl SyndicateTransactionEvent {
    /// Accumulate the batch
    pub fn accumulate(&self, acc: B256) -> B256 {
        keccak256(
            (
                acc,
                keccak256(
                    (self.sender, self.block_number, self.timestamp, keccak256(&self.payload))
                        .abi_encode_packed(),
                ),
            )
                .abi_encode_packed(),
        )
    }

    /// Parse the payload depending on the compression scheme
    pub fn parse_payload(&self) -> Result<Vec<Bytes>, VerifierError> {
        Ok(SequencingTransactionParser::decode_event_data(&self.payload)?)
    }
}

/// Syndicate block
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyndicateBlock {
    /// Block number
    pub block_number: u64,
    /// Timestamp
    pub timestamp: u64,
    /// Transactions
    pub transactions: Vec<Bytes>,
}

/// Parse the syndicate transaction events
pub fn parse_syndicate_transaction_events(
    syndicate_transaction_events: &Vec<SyndicateTransactionEvent>,
) -> Result<Vec<SyndicateBlock>, VerifierError> {
    let mut aggregated_events_by_block: Vec<SyndicateBlock> = vec![];
    let mut current_block = syndicate_transaction_events[0].block_number;
    let mut current_timestamp = syndicate_transaction_events[0].timestamp;
    let mut payload: Vec<Bytes> = vec![];

    for event in syndicate_transaction_events {
        if event.block_number == current_block {
            payload.append(&mut event.parse_payload()?);
        } else {
            let block = SyndicateBlock {
                block_number: current_block,
                timestamp: current_timestamp,
                transactions: payload,
            };
            aggregated_events_by_block.push(block);
            current_block = event.block_number;
            current_timestamp = event.timestamp;
            payload = event.parse_payload()?;
        }
    }

    // Always push the final accumulated block
    aggregated_events_by_block.push(SyndicateBlock {
        block_number: current_block,
        timestamp: current_timestamp,
        transactions: payload,
    });

    Ok(aggregated_events_by_block)
}

// --------------------------------------------
// Batch Helpers
// --------------------------------------------

/// A batch with a timestamp
#[derive(Default, Debug, Clone)]
pub struct BatchWithTimestamp {
    /// Timestamp
    pub timestamp: u64,
    /// Batch
    pub batch: Bytes,
}

/// Builds a batch of transactions into an Arbitrum batch
pub fn build_batch(
    txs: Vec<Bytes>,
    block_number: u64,
    timestamp: u64,
) -> Result<Bytes, VerifierError> {
    let mut messages = vec![];
    if !txs.is_empty() {
        messages.push(BatchMessage::L2(ArbL1IncomingMessage {
            header: ArbL1IncomingMessageHeader { block_number, timestamp },
            l2_msg: txs,
        }));
    };

    let batch = Batch(messages);
    let encoded_batch = batch.encode()?;
    Ok(encoded_batch)
}

/// Get the input batches
pub fn get_input_batches_with_timestamps(
    sequencing_chain_input: &SequencingChainInput,
) -> Result<Vec<BatchWithTimestamp>, VerifierError> {
    let syndicate_blocks =
        parse_syndicate_transaction_events(&sequencing_chain_input.syndicate_transaction_events)?;

    let mut map_syndicate_block_number_to_block = HashMap::new();
    for block in syndicate_blocks {
        map_syndicate_block_number_to_block.insert(block.block_number, block);
    }

    let sequencing_headers = sequencing_chain_input.block_headers.clone();
    let mut batches = vec![];

    for header in sequencing_headers {
        match map_syndicate_block_number_to_block.get(&header.number) {
            // If there is a batch for this block, build it
            Some(block) => {
                let batch =
                    build_batch(block.transactions.clone(), block.block_number, block.timestamp)
                        .map(|batch| BatchWithTimestamp { timestamp: block.timestamp, batch })
                        .map_err(|_| VerifierError::InvalidBatch)?;
                batches.push(batch);
            }
            // If there is no batch for this block, build an empty batch
            None => {
                let empty_batch =
                    BatchWithTimestamp { timestamp: header.timestamp, batch: Bytes::new() };
                batches.push(empty_batch);
            }
        }
    }

    Ok(batches)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::U256,
        providers::{Provider as _, ProviderBuilder, RootProvider},
    };
    use std::str::FromStr;
    use withdrawals_shared::types::{L1IncomingMessage, L1IncomingMessageHeader};

    #[test]
    fn test_accumulator_storage_slot() {
        let slot = keccak256(b"syndicate.accumulator");
        assert_eq!(slot, SYNDICATE_ACCUMULATOR_STORAGE_SLOT);
    }

    #[test]
    fn test_l1_incoming_message_accumulate() {
        let header = L1IncomingMessageHeader {
            kind: 1,
            sender: Address::repeat_byte(0x11),
            block_number: 1,
            timestamp: 123,
            request_id: B256::repeat_byte(0x01),
            base_fee_l1: U256::from(1000),
        };
        let msg = L1IncomingMessage { header, l2msg: Bytes::from(vec![0x01, 0x02, 0x03]) };
        let acc = B256::repeat_byte(0x00);
        let new_acc = msg.accumulate(acc);
        assert_ne!(acc, new_acc);
    }

    #[test]
    fn test_syndicate_transaction_event_accumulate() {
        let event = SyndicateTransactionEvent {
            block_number: 100,
            timestamp: 1000000,
            sender: Address::repeat_byte(0xAA),
            payload: Bytes::from(vec![0xDE, 0xAD, 0xBE, 0xEF]),
        };
        let acc = B256::ZERO;
        let new_acc = event.accumulate(acc);
        assert_ne!(acc, new_acc);
    }

    #[test]
    fn test_parse_empty_syndicate_transaction_event_payload() {
        let event = SyndicateTransactionEvent {
            block_number: 0,
            timestamp: 0,
            sender: Address::repeat_byte(0x00),
            payload: Bytes::new(),
        };
        let parsed = event.parse_payload();
        assert!(parsed.is_err());
    }

    #[tokio::test]
    async fn test_verify_account_proof_response() {
        let client: RootProvider = ProviderBuilder::default()
            .connect("https://syndicate-exo.g.alchemy.com/v2/K6cAUXQhrUT3KJPd9a-glciOF5ZA_F8Y")
            .await
            .unwrap();

        let address = Address::from_str("0x180972BF154c9Aea86c43149D83B7Ea078c33f48").unwrap();
        let test_slot = B256::ZERO;
        let proof: EIP1186AccountProofResponse = client
            .raw_request(
                "eth_getProof".into(),
                (address, vec![U256::from_be_bytes(test_slot.0)], "latest"),
            )
            .await
            .unwrap();

        let block = client
            .get_block_by_number(alloy::eips::BlockNumberOrTag::Latest)
            .await
            .unwrap()
            .unwrap();
        let state_root = block.header.state_root;

        let result = SequencingChainInput::verify_account_proof_response(&proof, state_root);

        assert!(result.is_ok());
    }
}
