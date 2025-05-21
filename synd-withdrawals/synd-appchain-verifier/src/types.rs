//! Types for the `synd-appchain-verifier`

use crate::errors::VerifierError;
use alloy::{
    primitives::{keccak256, Address, Bytes, B256, U256},
    rpc::types::EIP1186AccountProofResponse,
    sol_types::SolValue as _,
};
use alloy_trie::{proof::verify_proof, Nibbles, TrieAccount};
use serde::{Deserialize, Serialize};

/// Settlement chain input
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettlementChainInput {
    /// Start accumulator
    pub start_accumulator: B256,
    /// Delayed messages Accumulator
    pub delayed_messages_accumulator: B256,
    /// Delayed messages
    pub delayed_messages: Vec<L1IncomingMessage>,
}

impl SettlementChainInput {
    /// Validate the settlement chain input
    pub fn validate(&self) -> Result<(), VerifierError> {
        let mut acc = self.start_accumulator;
        for delayed_message in &self.delayed_messages {
            acc = delayed_message.accumulate(acc);
        }
        if acc != self.delayed_messages_accumulator {
            return Err(VerifierError::InvalidSettlementChainInput);
        }
        Ok(())
    }
}
/// Sequencing chain input
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequencingChainInput {
    /// Sequencing chain contract address
    pub sequencing_chain_contract_address: Address,
    /// Syndicate Accumulator Merkle Proof
    pub syndicate_accumulator_merkle_proof: EIP1186AccountProofResponse,

    /// End block hash
    pub end_block_hash: B256,

    /// Start accumulator
    pub start_accumulator: B256,

    /// End accumulator
    pub end_accumulator: B256,

    /// Syndicate batches
    pub syndicate_batches: Vec<SyndBatch>,

    // Trusted input
    /// Start batch number
    pub start_batch_number: u64,
}

impl SequencingChainInput {
    fn verify_accumulator(&self) -> Result<(), VerifierError> {
        let mut acc = self.start_accumulator;
        for syndicate_batch in &self.syndicate_batches {
            acc = syndicate_batch.accumulate(acc);
        }
        if acc != self.end_accumulator {
            return Err(VerifierError::InvalidSequencingChainInput);
        }
        Ok(())
    }

    fn verify_account_proof_response(
        proof: &EIP1186AccountProofResponse,
        block_hash: B256,
    ) -> Result<(), VerifierError> {
        let key = Nibbles::unpack(keccak256(proof.address));
        println!("Key {:?}", key);
        let expected_value = alloy::rlp::encode(TrieAccount {
            balance: proof.balance,
            code_hash: proof.code_hash,
            nonce: proof.nonce,
            storage_root: proof.storage_hash,
        });
        println!("Expected value {:?} - length {:?}", expected_value, expected_value.len());
        verify_proof(block_hash, key, Some(expected_value), &proof.account_proof).map_err(|e| {
            println!("Error {:?}", e);
            VerifierError::InvalidSequencingChainInput
        })?;
        println!("Proof {:?}", proof);
        for p in &proof.storage_proof {
            let k = Nibbles::unpack(p.key.as_b256());
            verify_proof(proof.storage_hash, k, Some(p.value.to_be_bytes_vec()), &p.proof)
                .map_err(|e| {
                    println!("Error 2 {:?}", e);
                    VerifierError::InvalidSequencingChainInput
                })?;
        }
        println!("Proof 2 {:?}", proof);
        Ok(())
    }

    fn verify_merkle_proof(&self) -> Result<(), VerifierError> {
        // Verify syndicate accumulator merkle proof
        Self::verify_account_proof_response(
            &self.syndicate_accumulator_merkle_proof,
            self.end_block_hash,
        )?;

        // Verify there is only one storage proof
        assert!(self.syndicate_accumulator_merkle_proof.storage_proof.len() == 1);

        // Verify storage slot
        let storage_slot = keccak256("syndicate.accumulator");
        let storage_proof = &self.syndicate_accumulator_merkle_proof.storage_proof[0];

        if storage_proof.key.as_b256() != storage_slot {
            return Err(VerifierError::InvalidSequencingChainInput);
        }

        if storage_proof.value != self.end_accumulator.into() {
            return Err(VerifierError::InvalidSequencingChainInput);
        }

        // Verify address
        if self.syndicate_accumulator_merkle_proof.address != self.sequencing_chain_contract_address
        {
            return Err(VerifierError::InvalidSequencingChainInput);
        }

        Ok(())
    }

    /// Validate the sequencing chain input
    pub fn validate(&self) -> Result<(), VerifierError> {
        // Verify accumulator
        self.verify_accumulator()?;
        // Validate merkle proof
        self.verify_merkle_proof()?;
        Ok(())
    }
}

/// Output for the verifier
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifierOutput {
    /// Array of block verifier inputs
    pub block_verifier_inputs: Vec<BlockVerifierInput>,
    /// Batch count
    pub batch_count: u64,
}

// TODO: Move to a shared crate
/// `BlockVerifierInput` is the output of the `synd-appchain-verifier`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockVerifierInput {
    /// Appchain block hash
    pub block_hash: B256,
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
    pub fn hash(&self) -> B256 {
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
pub struct SyndBatch {
    /// Batch number
    pub batch_number: u64,
    /// Block number
    pub block_number: u64,
    /// Timestamp
    pub timestamp: u64,
    /// Messages
    pub payload: Bytes,
}

impl SyndBatch {
    /// Hash the batch
    pub fn hash(&self) -> B256 {
        keccak256(
            (self.batch_number, self.block_number, self.timestamp, &self.payload)
                .abi_encode_packed(),
        )
    }

    /// Accumulate the batch
    pub fn accumulate(&self, acc: B256) -> B256 {
        keccak256((acc, self.hash()).abi_encode_packed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::providers::{Provider as _, ProviderBuilder, RootProvider};
    use std::str::FromStr;

    #[tokio::test]
    // TODO: fix test
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
        let result = SequencingChainInput::verify_account_proof_response(&proof, block_hash);
        println!("Result {:?}", result);
        assert!(result.is_ok());
    }
}
