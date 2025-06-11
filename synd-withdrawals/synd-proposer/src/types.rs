//! The `types` module handles types for the Proposer.
use alloy::{
    primitives::{Address, Bytes, B256, U256},
    rpc::types::{EIP1186AccountProofResponse, Header},
};
use serde::{Deserialize, Serialize};
use synd_seqchain_verifier::types::{L1ChainInput, L1IncomingMessage};

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
