//! Configuration for the block builder service
use alloy::{
    primitives::Address,
    signers::{
        k256::ecdsa::SigningKey,
        local::{LocalSigner, PrivateKeySigner},
    },
};
use clap::{Parser, ValueEnum};
use std::{fmt::Debug, str::FromStr};
use thiserror::Error;

const DEFAULT_PRIVATE_KEY_SIGNER: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"; // address = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266

/// Configuration for the block builder service
#[derive(Parser, Clone)]
#[allow(missing_docs)]
pub struct BlockBuilderConfig {
    // TODO(SEQ-686): Remove this variable and read the owner address from the settlement chain
    // contract instead.
    #[arg(long, env = "ROLLUP_OWNER_ADDRESS", value_parser = parse_address, default_value_t = Address::ZERO)]
    pub owner_address: Address,

    #[arg(long, env = "MINE_EMPTY_BLOCKS", default_value_t = false)]
    pub mine_empty_blocks: bool,

    #[arg(long, env = "MCHAIN_AUTH_IPC_PATH")]
    pub mchain_auth_ipc_path: String,

    #[arg(long, env = "MCHAIN_IPC_PATH")]
    pub mchain_ipc_path: String,

    /// The chain ID of the Appchain rollup (not the mchain)
    #[arg(short = 'c', long, env = "TARGET_CHAIN_ID", default_value_t = 13331370)]
    pub target_chain_id: u64,

    /// Sequencing contract address on the sequencing chain
    #[arg(short = 's', long, env = "SEQUENCING_CONTRACT_ADDRESS",
        value_parser = parse_address)]
    pub sequencing_contract_address: Address,

    /// Target rollup type for the [`block-builder`]
    #[arg(long, env = "TARGET_ROLLUP", default_value = "arbitrum")]
    pub target_rollup_type: TargetRollupType,

    // TODO(SEQ-555): make bridge and inbox addresses specific to arbitrum
    /// Bridge address on the settlement chain
    #[arg(short = 'b', long, env = "ARBITRUM_BRIDGE_ADDRESS",
        value_parser = parse_address)]
    pub bridge_address: Address,

    /// Inbox address on the settlement chain
    #[arg(short = 'i', long, env = "ARBITRUM_INBOX_ADDRESS",
        value_parser = parse_address)]
    pub inbox_address: Address,

    // TODO (SEQ-567): Move ignore_delayed_messages config value on-chain
    /// Flag used to ignore delayed messages besides deposits
    /// Default is false
    #[arg(long, env = "ARBITRUM_IGNORE_DELAYED_MESSAGES", default_value = "false")]
    pub ignore_delayed_messages: bool,
}

/// Possible target rollup types for the [`block-builder`]
#[allow(missing_docs)]
#[derive(Debug, Clone, Parser, ValueEnum)]
pub enum TargetRollupType {
    OPTIMISM,
    ARBITRUM,
}

/// Parse a string into an Ethereum `Address`.
fn parse_address(value: &str) -> Result<Address, String> {
    Address::from_str(value).map_err(|_| format!("Invalid address: {}", value))
}

/// Parse default string into a `PrivateKeySigner`.
pub fn get_default_private_key_signer() -> LocalSigner<SigningKey> {
    PrivateKeySigner::from_str(DEFAULT_PRIVATE_KEY_SIGNER)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err))
}

#[allow(missing_docs)]
pub fn get_rollup_contract_address() -> Address {
    get_default_private_key_signer().address().create(0)
}

impl Debug for BlockBuilderConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockBuilderConfig")
            .field("owner_address", &self.owner_address)
            .field("mchain_auth_ipc_path", &self.mchain_auth_ipc_path)
            .field("mchain_ipc_path", &self.mchain_ipc_path)
            .field("target_chain_id", &self.target_chain_id)
            .field("sequencing_contract_address", &self.sequencing_contract_address)
            .field("target_rollup_type", &self.target_rollup_type)
            .field("bridge_address", &self.bridge_address)
            .field("inbox_address", &self.inbox_address)
            .field("signer_key", &"<private>") // Skip showing private key
            .field("ignore_delayed_messages", &self.ignore_delayed_messages)
            .finish()
    }
}

impl Default for BlockBuilderConfig {
    fn default() -> Self {
        let zero = Address::ZERO.to_string();
        Self::parse_from([
            "",
            "-s",
            &zero,
            "-b",
            &zero,
            "-i",
            &zero,
            "--mchain-ipc-path",
            "",
            "--mchain-auth-ipc-path",
            "",
        ])
    }
}

impl BlockBuilderConfig {
    /// Validates the config values and complains about impossible ones
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.target_chain_id == 0 {
            return Err(ConfigError::InvalidChainId("Chain ID cannot be 0".to_string()));
        }

        if self.sequencing_contract_address == Address::ZERO {
            return Err(ConfigError::InvalidAddress(
                "Sequencing contract address cannot be 0".to_string(),
            ));
        }

        match self.target_rollup_type {
            // Validate Arbitrum specific configuration
            TargetRollupType::ARBITRUM => {
                if self.bridge_address == Address::ZERO {
                    return Err(ConfigError::InvalidAddress(
                        "Bridge address cannot be 0".to_string(),
                    ));
                }
                if self.inbox_address == Address::ZERO {
                    return Err(ConfigError::InvalidAddress(
                        "Inbox address cannot be 0".to_string(),
                    ));
                }
            }
            // Validate Optimism specific configuration
            TargetRollupType::OPTIMISM => {
                return Err(ConfigError::UnsupportedRollupType(
                    "Optimism is not supported yet".to_string(),
                ));
            }
        }

        Ok(())
    }
}

#[allow(missing_docs)]
/// Possible errors that can occur when initializing the block builder configuration
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid URL: {0}")]
    InvalidURL(String),
    #[error("Invalid URL: no host")]
    InvalidURLHost,
    #[error("Invalid URL scheme: {0}. Only http and https are supported")]
    InvalidURLScheme(String),
    #[error("Invalid chain ID: {0}")]
    InvalidChainId(String),
    #[error("Unsupported rollup type: {0}")]
    UnsupportedRollupType(String),
    #[error("Invalid configuration address: {0}")]
    InvalidAddress(String),
    #[error("Invalid anvil state path: {0}")]
    InvalidAnvilStatePath(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn test_validate_chain_id() {
        let config = BlockBuilderConfig { target_chain_id: 0, ..Default::default() };
        assert_matches!(config.validate(), Err(ConfigError::InvalidChainId(_)));
    }

    #[test]
    fn test_validate_rollup_type() {
        let config = BlockBuilderConfig {
            target_rollup_type: TargetRollupType::OPTIMISM,
            sequencing_contract_address: get_rollup_contract_address(),
            ..Default::default()
        };
        assert_matches!(config.validate(), Err(ConfigError::UnsupportedRollupType(_)));
    }

    #[test]
    fn test_validate_bridge_address() {
        let config = BlockBuilderConfig { bridge_address: Address::ZERO, ..Default::default() };
        assert_matches!(config.validate(), Err(ConfigError::InvalidAddress(_)));
    }

    #[test]
    fn test_validate_inbox_address() {
        let config = BlockBuilderConfig { inbox_address: Address::ZERO, ..Default::default() };
        assert_matches!(config.validate(), Err(ConfigError::InvalidAddress(_)));
    }

    #[test]
    fn test_validate_sequencing_contract_address() {
        let config =
            BlockBuilderConfig { sequencing_contract_address: Address::ZERO, ..Default::default() };
        assert_matches!(config.validate(), Err(ConfigError::InvalidAddress(_)));
    }
}
