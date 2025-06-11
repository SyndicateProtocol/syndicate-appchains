//! The `types` module handles types for the Proposer.
use alloy::primitives::{Address, Bytes, B256, U256};
use serde::{Deserialize, Serialize};
use synd_appchain_verifier::types::{SequencingChainInput, SettlementChainInput};
use synd_seqchain_verifier::types::L1ChainInput;

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
