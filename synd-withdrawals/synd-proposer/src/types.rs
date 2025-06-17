//! The `types` module handles types for the Proposer.
use alloy::{
    primitives::{Address, Bytes, B256, U256},
    rpc::types::{EIP1186AccountProofResponse, Header},
};
use serde::{Deserialize, Serialize};
use synd_seqchain_verifier::types::L1ChainInput;
use withdrawals_shared::types::L1IncomingMessage;

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct NitroBlock {
    pub number: U256,
    pub l1_block_number: U256,
    pub timestamp: U256,
    pub hash: B256,
    pub send_root: B256,
}

// Appchain verifier inputs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SettlementChainInput {
    /// Trustless input
    pub delayed_messages: Vec<L1IncomingMessage>,
    pub start_delayed_messages_accumulator: B256,

    /// Trusted input
    pub end_delayed_messages_accumulator: B256,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SequencingChainInput {
    /// Trustless input
    pub start_syndicate_accumulator_merkle_proof: EIP1186AccountProofResponse,
    pub end_syndicate_accumulator_merkle_proof: EIP1186AccountProofResponse,
    pub syndicate_transaction_events: Vec<SyndicateTransactionEvent>,
    pub block_headers: Vec<Header>,

    /// Trusted input
    pub start_block_hash: B256,
    pub end_block_hash: B256,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SyndicateTransactionEvent {
    pub block_number: u64,
    pub timestamp: u64,
    pub sender: Address,
    pub payload: Bytes,
}

// Verify sequencing chain input & output (First call to the TEE synd-enclave)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VerifySequencingChainConfig {
    pub arbitrum_bridge_address: Address,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VerifySequencingChainInput {
    pub seq_config_hash: B256,
    pub verify_sequencing_chain_config: VerifySequencingChainConfig,
    pub l1_chain_input: L1ChainInput,
    pub sequencing_start_block_hash: B256,
    pub sequencing_pre_image_data: Vec<Vec<u8>>,
}

impl VerifySequencingChainInput {
    pub fn hash(&self) -> B256 {
        use alloy::primitives::keccak256;
        let mut data = Vec::new();
        data.extend_from_slice(&self.seq_config_hash.0);
        data.extend_from_slice(&self.l1_chain_input.start_block_hash.0);
        data.extend_from_slice(&self.l1_chain_input.end_block_hash.0);
        data.extend_from_slice(&self.sequencing_start_block_hash.0);
        keccak256(&data)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VerifySequencingChainOutput {
    pub sequencing_block_hash: B256,
    pub signature: Bytes,
}

// Verify appchain input & output (Second call to the TEE synd-enclave)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VerifyAppchainConfig {
    pub sequencing_contract_address: Address,
    pub settlement_delay: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VerifyAppchainInput {
    pub seq_config_hash: B256,
    pub l1_start_block_hash: B256,
    pub l1_end_block_hash: B256,
    pub appchain_config_hash: B256,
    pub verify_appchain_config: VerifyAppchainConfig,
    pub settlement_chain_input: SettlementChainInput,
    pub sequencing_chain_input: SequencingChainInput,
    pub appchain_pre_image_data: Vec<Vec<u8>>,
    pub verify_sequencing_chain_output: VerifySequencingChainOutput,
    pub appchain_start_block_hash: B256,
}

impl VerifyAppchainInput {
    pub fn hash(&self) -> B256 {
        use alloy::primitives::keccak256;
        let mut data = Vec::new();
        data.extend_from_slice(&self.appchain_config_hash.0);
        data.extend_from_slice(&self.appchain_start_block_hash.0);
        data.extend_from_slice(&self.seq_config_hash.0);
        data.extend_from_slice(&self.sequencing_chain_input.start_block_hash.0);
        data.extend_from_slice(&self.sequencing_chain_input.end_block_hash.0);
        data.extend_from_slice(&self.sequencing_chain_input.start_block_hash.0);
        data.extend_from_slice(&self.settlement_chain_input.end_delayed_messages_accumulator.0);
        data.extend_from_slice(&self.l1_start_block_hash.0);
        data.extend_from_slice(&self.l1_end_block_hash.0);
        keccak256(&data)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VerifyAppchainOutput {
    pub sequencing_block_hash: B256,
    pub appchain_block_hash: B256,
    pub appchain_send_root: B256,
    pub signature: Bytes,
}

// Block verifier inputs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockVerifierInput {
    pub min_timestamp: u64,
    pub max_timestamp: u64,
    pub min_block_number: u64,
    pub max_block_number: u64,
    pub messages: Vec<L1IncomingMessage>,
    pub batch: Bytes,
}

// Keep your existing types that weren't in the Go file
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeqVerifyOutput {
    pub block_hash: B256,
    pub signature: Bytes,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppVerifyOutput {
    pub block_hash: B256,
    pub send_root: B256,
    pub seq_block_hash: B256,
    pub signature: Bytes,
}

// The Batch type based on wavmio.Batch
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Batch {
    pub min_timestamp: u64,
    pub max_timestamp: u64,
    pub min_block_number: u64,
    pub max_block_number: u64,
    pub batch: Vec<u8>,
    pub messages: Vec<L1IncomingMessage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BatchWithCount {
    pub delayed_message_count: u64,
    pub batch: Batch,
}

// ValidationInput type that's used in the Go code
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationInput {
    pub block_hash: B256,
    pub preimage_data: Vec<Vec<u8>>,
    pub batches: Vec<Batch>,
}






use crate::errors::AppchainVerifierError;
use alloy::{
    primitives::{fixed_bytes, keccak256, map::HashMap, Address, Bytes, B256},
    rpc::types::{EIP1186AccountProofResponse, Header},
    sol_types::SolValue as _,
};
use serde::{Deserialize, Serialize};
use synd_block_builder::appchains::{
    arbitrum::batch::{
        Batch, BatchMessage, L1IncomingMessage as ArbL1IncomingMessage,
        L1IncomingMessageHeader as ArbL1IncomingMessageHeader,
    },
    shared::SequencingTransactionParser,
};
use withdrawals_shared::{
    error::VerifierError,
    merkle_proof::verify_merkle_proof,
    types::{get_delayed_messages_accumulator, L1IncomingMessage},
};

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
    pub fn validate(&self) -> Result<(), AppchainVerifierError> {
        let acc = get_delayed_messages_accumulator(
            &self.delayed_messages,
            self.start_delayed_messages_accumulator,
        );
        if acc != self.end_delayed_messages_accumulator {
            return Err(VerifierError::InvalidInput {
                reason: "Invalid end delayed messages accumulator".to_string(),
                expected: self.end_delayed_messages_accumulator.to_string(),
                actual: acc.to_string(),
            })?;
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
    fn verify_accumulator(&self) -> Result<(), AppchainVerifierError> {
        let mut acc = self.start_syndicate_accumulator_merkle_proof.storage_proof[0].value.into();
        for syndicate_transaction in &self.syndicate_transaction_events {
            acc = syndicate_transaction.accumulate(acc);
        }
        let expected_end_accumulator: B256 =
            self.end_syndicate_accumulator_merkle_proof.storage_proof[0].value.into();
        if acc != expected_end_accumulator {
            return Err(VerifierError::InvalidInput {
                reason: "Invalid end accumulator".to_string(),
                expected: expected_end_accumulator.to_string(),
                actual: acc.to_string(),
            })?;
        }
        Ok(())
    }

    /// Verify the block headers
    #[allow(clippy::unwrap_used)]
    fn verify_block_headers(&self) -> Result<(), AppchainVerifierError> {
        if self.block_headers.is_empty() {
            return Err(AppchainVerifierError::InvalidSequencingChainInputWithReason {
                reason: "Empty block headers".to_string(),
            });
        }

        // Check that the first block hash matches the expected start hash
        let first_hash = self.block_headers.first().unwrap().hash_slow();
        if first_hash != self.start_block_hash {
            return Err(VerifierError::InvalidInput {
                reason: "Invalid start block hash".to_string(),
                expected: self.start_block_hash.to_string(),
                actual: first_hash.to_string(),
            })?;
        }

        // Verify each header’s parent_hash matches the previous block’s hash
        for window in self.block_headers.windows(2) {
            let prev = &window[0];
            let curr = &window[1];
            if curr.parent_hash != prev.hash_slow() {
                return Err(VerifierError::InvalidInput {
                    reason: "Invalid parent hash".to_string(),
                    expected: prev.hash_slow().to_string(),
                    actual: curr.parent_hash.to_string(),
                })?;
            }
        }

        // Check that the last block hash matches the expected end hash
        let last_hash = self.block_headers.last().unwrap().hash_slow();
        if last_hash != self.end_block_hash {
            return Err(VerifierError::InvalidInput {
                reason: "Invalid end block hash".to_string(),
                expected: self.end_block_hash.to_string(),
                actual: last_hash.to_string(),
            })?;
        }

        Ok(())
    }

    /// Validate the sequencing chain input
    #[allow(clippy::unwrap_used)]
    pub fn validate(
        &self,
        sequencing_chain_contract_address: Address,
    ) -> Result<(), AppchainVerifierError> {
        // Verify block headers
        self.verify_block_headers()?;
        // Verify accumulator
        self.verify_accumulator()?;
        // Validate start syndicate accumulator merkle proof
        verify_merkle_proof(
            &self.start_syndicate_accumulator_merkle_proof,
            self.block_headers.first().unwrap(),
            self.start_block_hash,
            sequencing_chain_contract_address,
            vec![SYNDICATE_ACCUMULATOR_STORAGE_SLOT],
        )?;
        // Validate  end syndicate accumulator merkle proof
        verify_merkle_proof(
            &self.end_syndicate_accumulator_merkle_proof,
            self.block_headers.last().unwrap(),
            self.end_block_hash,
            sequencing_chain_contract_address,
            vec![SYNDICATE_ACCUMULATOR_STORAGE_SLOT],
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
                    (self.sender, self.block_number, keccak256(&self.payload)).abi_encode_packed(),
                ),
            )
                .abi_encode_packed(),
        )
    }

    /// Parse the payload depending on the compression scheme
    pub fn parse_payload(&self) -> Result<Vec<Bytes>, AppchainVerifierError> {
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
) -> Result<Vec<SyndicateBlock>, AppchainVerifierError> {
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
) -> Result<Bytes, AppchainVerifierError> {
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
) -> Result<Vec<BatchWithTimestamp>, AppchainVerifierError> {
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
                        .map_err(|_| AppchainVerifierError::InvalidBatch)?;
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
    use alloy::primitives::U256;
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
            sender: Address::repeat_byte(0x00),
            payload: Bytes::new(),
        };
        let parsed = event.parse_payload();
        assert!(parsed.is_err());
    }
}









//! Types for the `synd-seqchain-verifier`

use alloy::{
    primitives::{fixed_bytes, keccak256, Address, Bytes, B256, U256},
    rpc::types::{EIP1186AccountProofResponse, Header},
    sol_types::SolValue as _,
};
use serde::{Deserialize, Serialize};
use withdrawals_shared::{
    error::VerifierError,
    merkle_proof::verify_merkle_proof,
    types::{get_delayed_messages_accumulator, L1IncomingMessage},
};

// Storage slot of the batch accumulator
// <https://github.com/SyndicateProtocol/nitro-contracts/blob/9a100a86242176b633a1d907e5efd41296922144/src/bridge/AbsBridge.sol#L51>
// Since the batch accumulator is a dynamic array, this slot contains the length of the array
const BATCH_ACCUMULATOR_STORAGE_SLOT: B256 =
    fixed_bytes!("0x0000000000000000000000000000000000000000000000000000000000000007");
// Storage slot of the first element in the batch accumulator array
// Dynamic types are stored starting at the keccak256 of the original storage slot plus an offset
// This value is Keccak256("0x7")
const BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT: B256 =
    fixed_bytes!("0xa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c688");

/// Calculate the slot for the batch accumulator
#[allow(clippy::unwrap_used)]
pub fn calculate_slot(start_slot: B256, index: U256) -> B256 {
    B256::from(
        U256::from_be_bytes::<32>(start_slot.as_slice().try_into().unwrap()) + index -
            U256::from(1),
    )
}

/// L1 chain input
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct L1ChainInput {
    /// Start batch accumulator merkle proof
    /// Storage proof 0: count
    /// Storage proof 1: accumulator
    pub start_batch_accumulator_merkle_proof: EIP1186AccountProofResponse,

    /// End batch accumulator merkle proof
    /// Storage proof 0: count
    /// Storage proof 1: accumulator
    pub end_batch_accumulator_merkle_proof: EIP1186AccountProofResponse,

    /// Start block header
    pub start_block_header: Header,
    /// End block header
    pub end_block_header: Header,

    /// Delayed messages
    pub delayed_messages: Vec<L1IncomingMessage>,
    /// Batches
    pub batches: Vec<ArbitrumBatch>,

    // Trusted input
    /// Start block hash
    pub start_block_hash: B256,
    /// End block hash
    pub end_block_hash: B256,
}

impl L1ChainInput {
    fn start_batch_accumulator(&self) -> B256 {
        self.start_batch_accumulator_merkle_proof.storage_proof[1].value.into()
    }

    fn start_batch_count(&self) -> U256 {
        self.start_batch_accumulator_merkle_proof.storage_proof[0].value
    }

    fn end_batch_accumulator(&self) -> B256 {
        self.end_batch_accumulator_merkle_proof.storage_proof[1].value.into()
    }

    fn end_batch_count(&self) -> U256 {
        self.end_batch_accumulator_merkle_proof.storage_proof[0].value
    }

    fn verify_accumulator(&self) -> Result<(), VerifierError> {
        let mut acc = self.start_batch_accumulator();
        for batch in &self.batches {
            acc = batch.accumulate(acc);
        }
        if acc != self.end_batch_accumulator() {
            return Err(VerifierError::InvalidInput {
                reason: "Invalid end accumulator".to_string(),
                expected: self.end_batch_accumulator().to_string(),
                actual: acc.to_string(),
            });
        }
        Ok(())
    }

    #[allow(clippy::unwrap_used)]
    fn verify_delayed_message_accumulator(&self) -> Result<(), VerifierError> {
        if self.delayed_messages.is_empty() {
            return Ok(());
        }

        let acc = get_delayed_messages_accumulator(
            &self.delayed_messages,
            self.batches.first().unwrap().delayed_acc,
        );

        let last_batch = self.batches.last().unwrap();
        if acc != last_batch.delayed_acc {
            return Err(VerifierError::InvalidInput {
                reason: "Invalid delayed accumulator".to_string(),
                expected: last_batch.delayed_acc.to_string(),
                actual: acc.to_string(),
            });
        }

        let last_delayed_message = self.delayed_messages.last().unwrap();
        let delayed_message_count: U256 = last_delayed_message.header.request_id.into();
        if delayed_message_count != last_batch.after_delayed_messages_read {
            return Err(VerifierError::InvalidInput {
                reason: "Invalid delayed message count".to_string(),
                expected: delayed_message_count.to_string(),
                actual: last_batch.after_delayed_messages_read.to_string(),
            });
        }
        Ok(())
    }

    /// Validate the sequencing chain input
    #[allow(clippy::unwrap_used)]
    pub fn validate(&self, arbitrum_contract_address: Address) -> Result<(), VerifierError> {
        // Verify accumulator
        self.verify_accumulator()?;
        self.verify_delayed_message_accumulator()?;

        // Validate start batch accumulator merkle proof
        let start_acc_slot =
            calculate_slot(BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT, self.start_batch_count());
        verify_merkle_proof(
            &self.start_batch_accumulator_merkle_proof,
            &self.start_block_header,
            self.start_block_hash,
            arbitrum_contract_address,
            vec![BATCH_ACCUMULATOR_STORAGE_SLOT, start_acc_slot],
        )?;

        // Validate end batch accumulator merkle proof
        let end_acc_slot =
            calculate_slot(BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT, self.end_batch_count());
        verify_merkle_proof(
            &self.end_batch_accumulator_merkle_proof,
            &self.end_block_header,
            self.end_block_hash,
            arbitrum_contract_address,
            vec![BATCH_ACCUMULATOR_STORAGE_SLOT, end_acc_slot],
        )?;

        Ok(())
    }
}

/// Time bounds
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TimeBounds {
    /// Minimum timestamp
    pub min_timestamp: u64,
    /// Maximum timestamp
    pub max_timestamp: u64,
    /// Minimum block number
    pub min_block_number: u64,
    /// Maximum block number
    pub max_block_number: u64,
}

/// Arbitrum batch
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ArbitrumBatch {
    /// Delayed accumulator
    pub delayed_acc: B256,
    /// Number of delayed messages read
    pub after_delayed_messages_read: U256,
    /// Time bounds
    pub time_bounds: TimeBounds,
    /// Batch data
    pub data: Bytes,
}

impl ArbitrumBatch {
    /// Hash the batch
    pub fn hash(&self) -> B256 {
        #[allow(clippy::unwrap_used)]
        let after_delayed_messages_read: u64 = self.after_delayed_messages_read.try_into().unwrap();
        let header = (
            self.time_bounds.min_timestamp,
            self.time_bounds.max_timestamp,
            self.time_bounds.min_block_number,
            self.time_bounds.max_block_number,
            after_delayed_messages_read,
        )
            .abi_encode_packed();

        keccak256((header, &self.data).abi_encode_packed())
    }
    /// Accumulate the batch
    pub fn accumulate(&self, acc: B256) -> B256 {
        keccak256((acc, self.hash(), self.delayed_acc).abi_encode_packed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::{bytes, fixed_bytes, FixedBytes, Uint, U256},
        rpc::types::EIP1186StorageProof,
    };
    use std::str::FromStr;
    use withdrawals_shared::types::L1IncomingMessageHeader;

    //`EigenDA` message header flag
    // <https://github.com/Layr-Labs/nitro-contracts/blob/278fdbc39089fa86330f0c23f0a05aee61972c84/src/bridge/SequencerInbox.sol#L133>
    // This is real exo batch
    const BATCH_DATA: FixedBytes<1377> = fixed_bytes!("0xed0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000003a000000000000000000000000000000000000000000000000000000000000350ea000000000000000000000000000000000000000000000000000000000000008800000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000003200000000000000000000000000000000000000000000000000000000000000060d18df624719d00e345b1b0797827cf4246576a391eb85b77b9b8f3db3b2f4f7c00000000000000000000000000000000000000000000000000000000003bd1b9f4a6148b02394957923a4fe4e54e23b88193c18bac3f2e3bf1333b1f7a013724000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000003bd16400000000000000000000000000000000000000000000000000000000000000020001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000261570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100f609605b079c9a369ff81b018e0201707b95dcb2652735301db3201325bee4a374eeaa19e5d55af0cdc1103fda3ebe814bbff3df4ddc5346d2bdf71dc2ddc9ecb3614c5c4f28a7a6b90459130fd59809c5036796abeaf06e34bbb6cf9b3d45f80552e37f59a2bcfad73f79f3c91a2a66bbbf2ec0716fa3b2ee6d198361005bc8699202e75aa170163f15107b30fdfd1612bcad04cdfba6441ead6331e2515e399cb9166f1b1c4d3a0bc1bcfe19bc5ddedc7024f63734d1d1ba6b209139cb41be8f8984bb6d48eeaca9506962198807869d73836a18a42c1016ea161e5fb50af87919e7fa34b14376456c043cfaf9a17dbd0f9dbbc31f34a8c941c74e9fd98193000000000000000000000000000000000000000000000000000000000000000200010000000000000000000000000000000000000000000000000000000000002db90222187e61d6bea8fadb67459b5869f5beb80836fa419bf86e5f52ee9a4e0401cf5a16aa3ef193c3e4f736ee4472f08e1e9cfe04d90287e0da1731f6bbe900000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000021000000000000000000000000000000000000000000000000000000000000003700000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002100000000000000000000000000000000000000000000000000000000000000370000000000000000000000000000000000000000000000000000000000000004");

    #[test]
    fn test_calculate_slot() {
        let index = U256::from(16);
        let slot = calculate_slot(BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT, index);

        let result =
            fixed_bytes!("0xa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c697");
        assert_eq!(slot, result);
    }

    #[test]
    fn test_verify_accumulator() {
        // Using data from real batch from Exo:
        // https://holesky.etherscan.io/inputdatadecoder?tx=0xab80c93423f60da385f0b3f4ca1a162aafd922aa0aa3ce031d4e37d0f729c1a9
        // https://holesky.etherscan.io/tx/0xab80c93423f60da385f0b3f4ca1a162aafd922aa0aa3ce031d4e37d0f729c1a9#eventlog

        let start_acc =
            fixed_bytes!("0x1bd4bb833a82241a66c6a1c61678fb5c05d7a91f2f5bae0c6dc1cb993181f29b");
        let start_acc_uint: Uint<256, 4> =
            U256::from_be_bytes::<32>(start_acc.as_slice().try_into().unwrap());
        let start_batch_accumulator_merkle_proof = EIP1186AccountProofResponse {
            storage_proof: vec![
                EIP1186StorageProof::default(),
                EIP1186StorageProof { value: start_acc_uint, ..Default::default() },
            ],
            ..Default::default()
        };
        let end_acc =
            fixed_bytes!("0xce20c7f55228e3becbbb511ae6b1687d899fe9198ae203cae0a6825c8871183d");
        let end_acc_uint: Uint<256, 4> =
            U256::from_be_bytes::<32>(end_acc.as_slice().try_into().unwrap());
        let end_batch_accumulator_merkle_proof = EIP1186AccountProofResponse {
            storage_proof: vec![
                EIP1186StorageProof::default(),
                EIP1186StorageProof { value: end_acc_uint, ..Default::default() },
            ],
            ..Default::default()
        };

        let batch = ArbitrumBatch {
            delayed_acc: fixed_bytes!(
                "0xdfc8a2a4675e75cf9321ccfdc5d3d0cd97d8754b2b2d3d693a180d2deb01330c"
            ),
            time_bounds: TimeBounds {
                min_timestamp: 1662211476,
                max_timestamp: 1749331476,
                min_block_number: 3200322,
                max_block_number: 4640322,
            },
            after_delayed_messages_read: U256::from(24),
            data: BATCH_DATA.into(),
        };

        let input = L1ChainInput {
            start_batch_accumulator_merkle_proof,
            end_batch_accumulator_merkle_proof,
            start_block_header: Header::default(),
            end_block_header: Header::default(),
            delayed_messages: vec![],
            batches: vec![batch],
            start_block_hash: B256::ZERO,
            end_block_hash: B256::ZERO,
        };

        input.verify_accumulator().unwrap();
    }

    #[test]
    fn test_delayed_message_accumulator() {
        // Using data from a real deposit on Exo
        // <https://holesky.etherscan.io/tx/0x6808715298914e24afc7f0b4df89794df640c44ec4ea1673e3d8b2c9a60844be>
        let first_batch = ArbitrumBatch {
            delayed_acc: fixed_bytes!(
                "0xf96de52c50990e0b0bfeed04059c8baf6ae634d4a8d416b53e36f2f2c889fcdf"
            ),
            after_delayed_messages_read: U256::from(22),
            ..Default::default()
        };

        let last_batch = ArbitrumBatch {
            delayed_acc: fixed_bytes!(
                "0xdfc8a2a4675e75cf9321ccfdc5d3d0cd97d8754b2b2d3d693a180d2deb01330c"
            ),
            after_delayed_messages_read: U256::from(23),
            ..Default::default()
        };

        let message = L1IncomingMessage {
            header: L1IncomingMessageHeader {
                kind: 12,
                sender: Address::from_str("0x3A0BB3a5B69711cc64b09240D2694d9f0F07fD07").unwrap(),
                block_number: 3673425,
                timestamp: 1744667292,
                request_id: B256::from_str(
                    "0x0000000000000000000000000000000000000000000000000000000000000017",
                )
                .unwrap(),
                base_fee_l1: U256::ZERO,
            },
            l2msg: bytes!(
                "0x28fab3a5b69711cc64b09240d2694d9f0f07ebf60000000000000000000000000000000000000000000000008ac7230489e80000"
            ),
        };

        let input = L1ChainInput {
            start_batch_accumulator_merkle_proof: EIP1186AccountProofResponse::default(),
            end_batch_accumulator_merkle_proof: EIP1186AccountProofResponse::default(),
            start_block_header: Header::default(),
            end_block_header: Header::default(),
            delayed_messages: vec![message],
            batches: vec![first_batch, last_batch],
            start_block_hash: B256::ZERO,
            end_block_hash: B256::ZERO,
        };

        input.verify_delayed_message_accumulator().unwrap();
    }
}
