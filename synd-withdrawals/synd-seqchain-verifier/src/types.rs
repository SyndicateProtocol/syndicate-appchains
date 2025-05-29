//! Types for the `synd-seqchain-verifier`

use crate::errors::VerifierError;
use alloy::{
    primitives::{fixed_bytes, keccak256, Address, Bytes, B256, U256},
    rpc::types::{EIP1186AccountProofResponse, Header},
    sol_types::SolValue as _,
};
use alloy_trie::{proof::verify_proof, Nibbles, TrieAccount};
use serde::{Deserialize, Serialize};

const BATCH_ACCUMULATOR_STORAGE_SLOT: B256 =
    fixed_bytes!("0x0000000000000000000000000000000000000000000000000000000000000007");
const BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT: B256 =
    fixed_bytes!("0xa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c688"); // Keccak256("0x7")

/// Calculate the slot for the batch accumulator
#[allow(clippy::unwrap_used)]
fn calculate_slot(start_slot: B256, index: U256) -> B256 {
    B256::from(
        U256::from_be_bytes::<32>(start_slot.as_slice().try_into().unwrap()) + index -
            U256::from(1),
    )
}

/// L1 chain input
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct L1ChainInput {
    /// Start batch accumulator merkle proof
    pub start_batch_accumulator_merkle_proof: EIP1186AccountProofResponse,
    /// Start batch count merkle proof
    pub start_batch_count_merkle_proof: EIP1186AccountProofResponse,

    /// End batch accumulator merkle proof
    pub end_batch_accumulator_merkle_proof: EIP1186AccountProofResponse,
    /// End batch count merkle proof
    pub end_batch_count_merkle_proof: EIP1186AccountProofResponse,

    /// Start delayed message accumulator
    pub start_delayed_message_accumulator: B256,
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
        self.start_batch_accumulator_merkle_proof.storage_proof[0].value.into()
    }

    fn start_batch_count(&self) -> U256 {
        self.start_batch_count_merkle_proof.storage_proof[0].value
    }

    fn end_batch_accumulator(&self) -> B256 {
        self.end_batch_accumulator_merkle_proof.storage_proof[0].value.into()
    }

    fn end_batch_count(&self) -> U256 {
        self.end_batch_count_merkle_proof.storage_proof[0].value
    }

    fn verify_accumulator(&self) -> Result<(), VerifierError> {
        let mut acc = self.start_batch_accumulator();
        let mut prev_delayed_acc = self.start_delayed_message_accumulator;
        for batch in &self.batches {
            acc = batch.accumulate(acc, prev_delayed_acc);
            prev_delayed_acc = batch.delayed_acc;
        }
        if acc != self.end_batch_accumulator() {
            return Err(VerifierError::InvalidL1ChainInput {
                reason: "Invalid end accumulator".to_string(),
                expected: self.end_batch_accumulator().to_string(),
                actual: acc.to_string(),
            });
        }
        Ok(())
    }

    #[allow(clippy::unwrap_used)]
    fn verify_delayed_message_accumulator(&self) -> Result<(), VerifierError> {
        let mut acc = self.start_delayed_message_accumulator;
        for message in &self.delayed_messages {
            acc = message.accumulate(acc);
        }

        let last_batch = self.batches.last().unwrap();
        if acc != last_batch.delayed_acc {
            return Err(VerifierError::InvalidL1ChainInput {
                reason: "Invalid delayed accumulator".to_string(),
                expected: last_batch.delayed_acc.to_string(),
                actual: acc.to_string(),
            });
        }

        let last_delayed_message = self.delayed_messages.last().unwrap();
        let delayed_message_count: U256 = last_delayed_message.header.request_id.into();
        if delayed_message_count != last_batch.after_delayed_messages_read {
            return Err(VerifierError::InvalidL1ChainInput {
                reason: "Invalid delayed message count".to_string(),
                expected: delayed_message_count.to_string(),
                actual: last_delayed_message.header.request_id.to_string(),
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
        contract_address: Address,
        slot: B256,
    ) -> Result<(), VerifierError> {
        // Verify header
        let actual_block_hash = header.hash_slow();
        if actual_block_hash != block_hash {
            return Err(VerifierError::InvalidL1ChainInput {
                reason: "Invalid block hash".to_string(),
                expected: block_hash.to_string(),
                actual: actual_block_hash.to_string(),
            });
        }
        // Verify end merkle proof
        Self::verify_account_proof_response(proof, header.state_root)?;

        // Verify there is only one storage proof
        if proof.storage_proof.len() != 1 {
            return Err(VerifierError::InvalidL1ChainInput {
                reason: "Invalid number of storage proofs".to_string(),
                expected: "1".to_string(),
                actual: proof.storage_proof.len().to_string(),
            });
        }
        // Verify storage slot
        let storage_proof = &proof.storage_proof[0];

        if storage_proof.key.as_b256() != slot {
            return Err(VerifierError::InvalidL1ChainInput {
                reason: "Invalid storage slot".to_string(),
                expected: slot.to_string(),
                actual: storage_proof.key.as_b256().to_string(),
            });
        }

        // Verify address
        if proof.address != contract_address {
            return Err(VerifierError::InvalidL1ChainInput {
                reason: "Invalid address".to_string(),
                expected: contract_address.to_string(),
                actual: proof.address.to_string(),
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

        // Validate start batch count merkle proof
        Self::verify_merkle_proof(
            &self.start_batch_count_merkle_proof,
            &self.start_block_header,
            self.start_block_hash,
            arbitrum_contract_address,
            BATCH_ACCUMULATOR_STORAGE_SLOT,
        )?;
        let start_acc_slot =
            calculate_slot(BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT, self.start_batch_count());
        Self::verify_merkle_proof(
            &self.start_batch_accumulator_merkle_proof,
            &self.start_block_header,
            self.start_block_hash,
            arbitrum_contract_address,
            start_acc_slot,
        )?;

        // Validate end batch count merkle proof
        Self::verify_merkle_proof(
            &self.end_batch_count_merkle_proof,
            &self.end_block_header,
            self.end_block_hash,
            arbitrum_contract_address,
            BATCH_ACCUMULATOR_STORAGE_SLOT,
        )?;
        let end_acc_slot =
            calculate_slot(BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT, self.end_batch_count());
        Self::verify_merkle_proof(
            &self.end_batch_accumulator_merkle_proof,
            &self.end_block_header,
            self.end_block_hash,
            arbitrum_contract_address,
            end_acc_slot,
        )?;

        Ok(())
    }
}

// TODO: Move to a shared crate
/// `BlockVerifierInput` is the output of the `synd-seqchain-verifier`
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
    /// Sender
    pub sender: Address,
    /// Block number
    pub block_number: u64,
    /// Timestamp
    pub timestamp: u64,
    /// Request ID
    pub request_id: B256,
    /// L1 base fee
    pub base_fee_l1: U256,
}

impl L1IncomingMessage {
    /// Hash the L1 incoming message
    fn hash(&self) -> B256 {
        let message_hash = keccak256(&self.l2msg);
        keccak256(
            (
                [self.header.kind],
                self.header.sender,
                self.header.block_number,
                self.header.timestamp,
                self.header.request_id,
                self.header.base_fee_l1,
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

/// Arbitrum batch
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArbitrumBatch {
    /// Batch sequence number
    pub batch_sequence_number: U256,
    /// Before accumulator
    pub before_acc: B256,
    /// After accumulator  
    pub after_acc: B256,
    /// Delayed accumulator
    pub delayed_acc: B256,
    /// Number of delayed messages read
    pub after_delayed_messages_read: U256,
    /// Time bounds
    pub time_bounds: TimeBounds,
    /// Batch data location
    pub data_location: u8,
    /// Batch data
    pub data: Bytes,
}

impl ArbitrumBatch {
    /// Hash the batch
    pub fn hash(&self) -> B256 {
        let header = (
            self.time_bounds.min_timestamp,
            self.time_bounds.max_timestamp,
            self.time_bounds.min_block_number,
            self.time_bounds.max_block_number,
            self.after_delayed_messages_read,
        )
            .abi_encode_packed();

        let data_hash = keccak256(&self.data);

        keccak256((header, data_hash).abi_encode_packed())
    }
    /// Accumulate the batch
    pub fn accumulate(&self, acc: B256, delayed_acc: B256) -> B256 {
        keccak256((acc, self.hash(), delayed_acc).abi_encode_packed())
    }
}

/// Time bounds
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::providers::{Provider as _, ProviderBuilder, RootProvider};
    use std::str::FromStr;

    #[test]
    fn test_calculate_slot() {
        let index = U256::from(16);
        let slot = calculate_slot(BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT, index);

        let result =
            fixed_bytes!("0xa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c697");
        assert_eq!(slot, result);
    }

    #[tokio::test]
    #[ignore]
    async fn test_verify_account_proof_response() {
        let client: RootProvider = ProviderBuilder::default()
            .connect("https://syndicate-exo.g.alchemy.com/v2/K6cAUXQhrUT3KJPd9a-glciOF5ZA_F8Y")
            .await
            .unwrap();
        let proof: EIP1186AccountProofResponse = client
            .raw_request(
                "eth_getProof".into(),
                (
                    Address::from_str("0x180972BF154c9Aea86c43149D83B7Ea078c33f48").unwrap(),
                    vec![U256::from(1)],
                    "latest",
                ),
            )
            .await
            .unwrap();
        println!("Proof {:?}", proof);
        let block_hash = client
            .get_block_by_number(alloy::eips::BlockNumberOrTag::Latest)
            .await
            .unwrap()
            .unwrap()
            .header
            .hash;
        println!("Block hash {:?}", block_hash);
        let result = L1ChainInput::verify_account_proof_response(&proof, block_hash);
        println!("Result {:?}", result);
        assert!(result.is_ok());
    }
}
