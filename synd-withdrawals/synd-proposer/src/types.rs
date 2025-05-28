//! The `types` module handles types for the Proposer.

use alloy::primitives::{Bytes, B256, U256};
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

//WIP types
pub struct VerifyPayload1 {
    pub l1_chain_input: L1ChainInput,
    /// Trustless preimage data
    pub seq_preimage_data: Vec<Vec<u8>>,
}

// Enclave types below

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeqTrustedInput {
    pub seq_config_hash: B256,
    pub l1_start_block_hash: B256,
    pub l1_end_block_hash: B256,
    pub seq_start_block_hash: B256,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeqVerifyInput {
    pub trusted_input: SeqTrustedInput,
    /// Trustless preimage data
    pub preimage_data: Vec<Vec<u8>>,
    /// The output of the verifier - should be replaced with verifier inputs instead
    pub batches: Vec<Batch>, // You'll need to define Batch based on wavmio.Batch
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppTrustedInput {
    pub seq_trusted_input: SeqTrustedInput,
    pub app_config_hash: B256,
    pub app_start_block_hash: B256,
    pub set_delayed_message_acc: B256,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppVerifyExtraInput {
    /// Trustless old output
    pub output: AppVerifyOutput,
    /// Trusted new inputs which replace the old ones
    pub l1_end_block_hash: B256,
    pub set_delayed_message_acc: B256,
    /// Trustless message hashes to derive the new accumulator from the old one
    pub delayed_message_hashes: Vec<B256>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppVerifyInput {
    /// Appchain trusted input
    pub trusted_input: AppTrustedInput,
    /// Optional extra input data to concat results with a previous AppVerifyOutput
    pub extra_input: Option<AppVerifyExtraInput>,
    /// Seq trustless output for either TrustedInput or ExtraInput
    pub seq_output: SeqVerifyOutput,
    /// Trustless preimage data
    pub preimage_data: Vec<Vec<u8>>,
    /// The output of the verifier - should be replaced with verifier inputs instead
    pub batches: Vec<Batch>,
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

// Helper methods
impl SeqTrustedInput {
    pub fn hash(&self) -> B256 {
        use alloy::primitives::keccak256;
        let mut data = Vec::new();
        data.extend_from_slice(&self.seq_config_hash.0);
        data.extend_from_slice(&self.l1_start_block_hash.0);
        data.extend_from_slice(&self.l1_end_block_hash.0);
        data.extend_from_slice(&self.seq_start_block_hash.0);
        keccak256(&data)
    }
}

impl AppTrustedInput {
    pub fn hash(&self) -> B256 {
        use alloy::primitives::keccak256;
        let mut data = Vec::new();
        data.extend_from_slice(&self.app_config_hash.0);
        data.extend_from_slice(&self.app_start_block_hash.0);
        data.extend_from_slice(&self.seq_trusted_input.seq_config_hash.0);
        data.extend_from_slice(&self.seq_trusted_input.seq_start_block_hash.0);
        data.extend_from_slice(&self.set_delayed_message_acc.0);
        data.extend_from_slice(&self.seq_trusted_input.l1_start_block_hash.0);
        data.extend_from_slice(&self.seq_trusted_input.l1_end_block_hash.0);
        keccak256(&data)
    }
}
