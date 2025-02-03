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
use url::Url;

const DEFAULT_PRIVATE_KEY_SIGNER: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

/// Configuration for the block builder service
#[derive(Parser, Clone)]
#[allow(missing_docs)]
pub struct BlockBuilderConfig {
    #[arg(short = 'u', long, env = "BLOCK_BUILDER_MCHAIN_URL",
        default_value = "http://127.0.0.1:8888",
        value_parser = parse_url)]
    pub mchain_url: Url,

    #[arg(
        short = 'g',
        long,
        env = "BLOCK_BUILDER_GENESIS_TIMESTAMP",
        default_value_t = 1712500000
    )]
    pub genesis_timestamp: u64,

    #[arg(short = 'c', long, env = "BLOCK_BUILDER_TARGET_CHAIN_ID", default_value_t = 13331370)]
    pub target_chain_id: u64,

    /// Sequencing contract address on the sequencing chain
    #[arg(short = 's', long, env = "BLOCK_BUILDER_SEQUENCING_CONTRACT_ADDRESS",
        value_parser = parse_address)]
    pub sequencing_contract_address: Address,

    /// Target rollup type for the [`block-builder`]
    #[arg(long, env = "BLOCK_BUILDER_TARGET_ROLLUP", default_value = "arbitrum")]
    pub target_rollup_type: TargetRollupType,

    /// Arbitrum rollup address on the m-chain
    #[arg(short = 'm', long, env = "BLOCK_BUILDER_ARBITRUM_MCHAIN_ROLLUP_ADDRESS",
        value_parser = parse_address,
        default_value = "0x5FbDB2315678afecb367f032d93F642f64180aa3")]
    pub mchain_rollup_address: Address,

    // TODO(SEQ-555): make bridge and inbox addresses specific to arbitrum
    /// Bridge address on the settlement chain
    #[arg(short = 'b', long, env = "BLOCK_BUILDER_ARBITRUM_BRIDGE_ADDRESS",
        value_parser = parse_address)]
    pub bridge_address: Address,

    /// Inbox address on the settlement chain
    #[arg(short = 'i', long, env = "BLOCK_BUILDER_ARBITRUM_INBOX_ADDRESS",
        value_parser = parse_address)]
    pub inbox_address: Address,

    // path to the directory where anvil will keep its state
    #[arg(long, env = "BLOCK_BUILDER_ANVIL_STATE_PATH", default_value = "")]
    pub anvil_state_path: String,

    // interval at which anvil saves state to disk (in seconds)
    #[arg(long, env = "BLOCK_BUILDER_ANVIL_STATE_INTERVAL", default_value_t = 1)]
    pub anvil_state_interval: u64,

    // number of states to be kept in memory by anvil
    #[arg(long, env = "BLOCK_BUILDER_ANVIL_PRUNE_HISTORY", default_value_t = 50)]
    pub anvil_prune_history: u64,
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

/// Parse default string into a valid [`URL`].
fn parse_url(value: &str) -> Result<Url, ConfigError> {
    Url::parse(value).map_or_else(
        |_err| Err(ConfigError::InvalidURL(value.to_string())),
        |url| {
            if !url.has_host() {
                return Err(ConfigError::InvalidURLHost);
            }
            match url.scheme() {
                "http" | "https" => Ok(url),
                _ => Err(ConfigError::InvalidURLScheme(url.scheme().to_string())),
            }
        },
    )
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
            .field("mchain_url", &self.mchain_url)
            .field("genesis_timestamp", &self.genesis_timestamp)
            .field("target_chain_id", &self.target_chain_id)
            .field("sequencing_contract_address", &self.sequencing_contract_address)
            .field("target_rollup_type", &self.target_rollup_type)
            .field("mchain_rollup_address", &self.mchain_rollup_address)
            .field("bridge_address", &self.bridge_address)
            .field("inbox_address", &self.inbox_address)
            .field("signer_key", &"<private>") // Skip showing private key
            .field("anvil_state_path", &self.anvil_state_path)
            .field("anvil_state_interval", &self.anvil_state_interval)
            .field("anvil_prune_history", &self.anvil_prune_history)
            .finish()
    }
}

impl Default for BlockBuilderConfig {
    fn default() -> Self {
        let zero = Address::ZERO.to_string();
        println!("{}", zero);
        Self::parse_from(["", "-s", &zero, "-b", &zero, "-i", &zero])
    }
}

impl BlockBuilderConfig {
    /// Validates the config values and complains about impossible ones
    pub fn validate(&self) -> Result<(), ConfigError> {
        parse_url(self.mchain_url.as_ref())?;

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
                if self.mchain_rollup_address == Address::ZERO {
                    return Err(ConfigError::InvalidAddress(
                        "MChain rollup address cannot be 0".to_string(),
                    ));
                }
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

        if self.anvil_state_path.is_empty() {
            return Err(ConfigError::InvalidAnvilStatePath(
                "Anvil state path cannot be empty".to_string(),
            ));
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
    use url::Url;

    #[test]
    fn test_default_parsing() {
        let zero = Address::ZERO.to_string();
        let config =
            BlockBuilderConfig::parse_from(["test", "-s", &zero, "-b", &zero, "-i", &zero]);
        assert_eq!(
            config.mchain_url,
            Url::parse("http://127.0.0.1:8888").expect("Failed to parse default URL")
        );
        assert_eq!(config.genesis_timestamp, 1712500000);
        assert_eq!(config.target_chain_id, 13331370);
        assert_eq!(
            config.sequencing_contract_address.to_string(),
            "0x0000000000000000000000000000000000000000"
        );
        assert_eq!(config.anvil_state_path, "");
        assert_eq!(config.anvil_state_interval, 1);
        assert_eq!(config.anvil_prune_history, 50);
    }

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
    fn test_validate_mchain_rollup_address() {
        let config =
            BlockBuilderConfig { mchain_rollup_address: Address::ZERO, ..Default::default() };
        assert_matches!(config.validate(), Err(ConfigError::InvalidAddress(_)));
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

    #[test]
    fn test_parse_url_valid() {
        let valid_urls = [
            "http://127.0.0.1:8888",
            "https://localhost:8000",
            "http://example.com:3000",
            "https://test.domain:8545",
        ];

        for url in valid_urls {
            assert!(parse_url(url).is_ok(), "URL should be valid: {}", url);
        }
    }

    #[test]
    fn test_parse_url_invalid_format() {
        let invalid_urls = ["not_a_url", "http://", "://test.com", "http:///", "", "123.456"];

        for url in invalid_urls {
            match parse_url(url) {
                Err(ConfigError::InvalidURL(error_url)) => {
                    assert_eq!(error_url, url, "Error should contain the invalid URL");
                }
                _ => panic!("Expected InvalidURL error for: {}", url),
            }
        }
    }

    #[test]
    fn test_parse_url_invalid_host_scheme() {
        let invalid_host_schemes = ["file://localhost.com", "data://example.com"];

        for url in invalid_host_schemes {
            match parse_url(url) {
                Err(ConfigError::InvalidURLScheme(_)) => {}
                Err(err) => panic!("Expected InvalidURLScheme error for: {}, got: {}", url, err),
                Ok(_) => panic!("Expected InvalidURLScheme error for: {}", url),
            }
        }
    }

    #[test]
    fn test_parse_url_no_host() {
        let urls_without_host = ["file:///path/to/file", "data:text/plain,Hello", "localhost:999"];

        for url in urls_without_host {
            match parse_url(url) {
                Err(ConfigError::InvalidURLHost) => {}
                _ => panic!("Expected InvalidURLHost error for: {}", url),
            }
        }
    }

    #[test]
    fn test_parse_url_with_port() {
        let result = parse_url("http://localhost:8545");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().port().unwrap(), 8545);
    }

    #[test]
    fn test_parse_url_without_port() {
        let result = parse_url("https://example.com");
        assert!(result.is_ok());
        // HTTPS default port is 443
        assert_eq!(result.unwrap().port().unwrap_or(443), 443);
    }

    #[test]
    fn test_parse_url_with_path() {
        let result = parse_url("http://localhost:8080/api/v1?param=value");
        assert!(result.is_ok());
        let url = result.unwrap();
        assert_eq!(url.path(), "/api/v1");
        assert_eq!(url.query(), Some("param=value"));
    }
}
