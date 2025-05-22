//! Types for the `synd-appchain-verifier`

use crate::errors::VerifierError;
use alloy::{
    primitives::{fixed_bytes, keccak256, Address, Bytes, B256, U256},
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
use tracing::error;

const SYNDICATE_ACCUMULATOR_STORAGE_SLOT: B256 =
    fixed_bytes!("0x847fe1a0bfd701c2dbb0b62670ad8712eed4c0ff4d2c6c0917f4c8d260ed0b90"); // Keccak256("syndicate.accumulator")

/// Settlement chain input
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
            return Err(VerifierError::InvalidSettlementChainInput);
        }
        Ok(())
    }
}
/// Sequencing chain input
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequencingChainInput {
    // TRUSTLESS INPUT
    /// Start block header
    pub start_block_header: Header,
    /// End block header
    pub end_block_header: Header,
    /// Sequencing chain contract address
    pub sequencing_chain_contract_address: Address,
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
            return Err(VerifierError::InvalidSequencingChainInput);
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
        verify_proof(state_root, key, Some(expected_value), &proof.account_proof).map_err(|e| {
            error!("Error {:?}", e);
            VerifierError::InvalidSequencingChainInput
        })?;
        for p in &proof.storage_proof {
            let k = Nibbles::unpack(keccak256(p.key.as_b256()));
            let expected_value = Some(alloy::rlp::encode_fixed_size(&p.value).to_vec());
            verify_proof(proof.storage_hash, k, expected_value, &p.proof).map_err(|e| {
                error!("Error 2 {:?}", e);
                VerifierError::InvalidSequencingChainInput
            })?;
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
        if header.hash_slow() != block_hash {
            return Err(VerifierError::InvalidSequencingChainInput);
        }
        // Verify end syndicate accumulator merkle proof
        Self::verify_account_proof_response(proof, header.state_root)?;

        // Verify there is only one storage proof
        if proof.storage_proof.len() != 1 {
            return Err(VerifierError::InvalidSequencingChainInput);
        }
        // Verify storage slot
        let storage_proof = &proof.storage_proof[0];

        if storage_proof.key.as_b256() != slot {
            return Err(VerifierError::InvalidSequencingChainInput);
        }

        // Verify address
        if proof.address != sequencing_chain_contract_address {
            return Err(VerifierError::InvalidSequencingChainInput);
        }

        Ok(())
    }

    /// Verify the block headers
    #[allow(clippy::unwrap_used)]
    fn verify_block_headers(&self) -> Result<(), VerifierError> {
        if self.block_headers.is_empty() {
            return Err(VerifierError::InvalidSequencingChainInput);
        }

        // Check that the first block hash matches the expected start hash
        let first_hash = self.block_headers.first().unwrap().hash_slow();
        if first_hash != self.start_block_hash {
            return Err(VerifierError::InvalidSequencingChainInput);
        }

        // Verify each header’s parent_hash matches the previous block’s hash
        for window in self.block_headers.windows(2) {
            let prev = &window[0];
            let curr = &window[1];
            if curr.parent_hash != prev.hash_slow() {
                return Err(VerifierError::InvalidSequencingChainInput);
            }
        }

        // Check that the last block hash matches the expected end hash
        let last_hash = self.block_headers.last().unwrap().hash_slow();
        if last_hash != self.end_block_hash {
            return Err(VerifierError::InvalidSequencingChainInput);
        }

        Ok(())
    }

    /// Validate the sequencing chain input
    pub fn validate(&self) -> Result<(), VerifierError> {
        // Verify block headers
        self.verify_block_headers()?;
        // Verify accumulator
        self.verify_accumulator()?;
        // Validate start syndicate accumulator merkle proof
        Self::verify_merkle_proof(
            &self.start_syndicate_accumulator_merkle_proof,
            &self.start_block_header,
            self.start_block_hash,
            self.sequencing_chain_contract_address,
            SYNDICATE_ACCUMULATOR_STORAGE_SLOT,
        )?;
        // Validate  end syndicate accumulator merkle proof
        Self::verify_merkle_proof(
            &self.end_syndicate_accumulator_merkle_proof,
            &self.end_block_header,
            self.end_block_hash,
            self.sequencing_chain_contract_address,
            SYNDICATE_ACCUMULATOR_STORAGE_SLOT,
        )?;
        Ok(())
    }
}

// TODO: Move to a shared crate
/// `BlockVerifierInput` is the output of the `synd-appchain-verifier`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockVerifierInput {
    /// Minimum timestamp
    pub min_timestamp: u64,
    /// Maximum timestamp
    pub max_timestamp: u64,
    /// Minimum block number
    pub min_block_number: u64,
    /// Maximum block number    
    pub max_block_number: u64,
    /// Messages
    pub messages: Vec<L1IncomingMessage>,
    /// Batch
    pub batch: Bytes,
}

/// L1 incoming message
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L1IncomingMessage {
    /// Header
    pub header: L1IncomingMessageHeader,
    /// L2 message
    pub l2msg: Bytes,
}

/// L1 incoming message header
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L1IncomingMessageHeader {
    /// Kind
    pub kind: u8,
    /// Poster
    pub poster: Address,
    /// Block number
    pub block_number: u64,
    /// Timestamp
    pub timestamp: u64,
    /// Request ID
    pub request_id: B256,
    /// L1 base fee
    pub l1_base_fee: U256,
}

impl L1IncomingMessage {
    /// Hash the L1 incoming message
    fn hash(&self) -> B256 {
        let message_hash = keccak256(&self.l2msg);
        keccak256(
            (
                [self.header.kind],
                self.header.poster,
                self.header.block_number,
                self.header.timestamp,
                self.header.request_id,
                self.header.l1_base_fee,
                message_hash,
            )
                .abi_encode_packed(),
        )
    }

    /// Accumulate the L1 incoming message
    pub fn accumulate(&self, acc: B256) -> B256 {
        let message_hash = self.hash();
        keccak256((acc, message_hash).abi_encode_packed())
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
// L1IncomingMessage Helper Functions
// --------------------------------------------

/// Get the current messages from the delayed messages
pub fn get_current_messages(
    delayed_messages: Vec<L1IncomingMessage>,
    timestamp: u64,
) -> Vec<L1IncomingMessage> {
    delayed_messages.into_iter().filter(|message| message.header.timestamp == timestamp).collect()
}

// --------------------------------------------
// Batch Helper Functions
// --------------------------------------------

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

/// A batch with a timestamp
#[derive(Default, Debug, Clone)]
pub struct BatchWithTimestamp {
    /// Timestamp
    pub timestamp: u64,
    /// Batch
    pub batch: Bytes,
}

/// Get the input batches
pub fn get_input_batches_with_timestamps(
    sequencing_chain_input: &SequencingChainInput,
) -> Result<Vec<BatchWithTimestamp>, VerifierError> {
    let blocks =
        parse_syndicate_transaction_events(&sequencing_chain_input.syndicate_transaction_events)?;
    let mut batches = vec![];
    for block in blocks {
        let batch = build_batch(block.transactions, block.block_number, block.timestamp)
            .map(|batch| BatchWithTimestamp { timestamp: block.timestamp, batch })
            .map_err(|_| VerifierError::InvalidBatch)?;
        batches.push(batch);
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

    #[test]
    fn test_accumulator_storage_slot() {
        let slot = keccak256(b"syndicate.accumulator");
        assert_eq!(slot, SYNDICATE_ACCUMULATOR_STORAGE_SLOT);
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
