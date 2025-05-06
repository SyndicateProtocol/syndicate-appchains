//! Common configuration for the Metabased Translator.
//!
//! This module contains all possible configuration options for the Metabased Translator. Different
//! crates each inherit a subset of these options to configure themselves

use alloy::primitives::Address;
use block_builder::config::BlockBuilderConfig;
use clap::Parser;
use eyre::Result;
use ingestor::config::{SequencingChainConfig, SettlementChainConfig};
use metrics::config::MetricsConfig;
use shared::{eth_client::RPCClientError, parse::parse_address};
use std::fmt::Debug;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(
        "Settlement chain start timestamp ({0}) is greater than sequencing chain timestamp ({1})"
    )]
    SettlementStartBlockTooLate(u64, u64),

    #[error("Failed to fetch block data: {0}")]
    RPCClient(#[from] RPCClientError),

    #[error("Block builder configuration error: {0}")]
    BlockBuilder(#[from] block_builder::config::ConfigError),

    #[error("Ingestor chain configuration error: {0}")]
    Ingestor(#[from] ingestor::config::ConfigError),

    #[error("Metrics configuration error: {0}")]
    Metrics(#[from] metrics::config::ConfigError),
}

/// Common config stuct for the Metabased Translator. This contains all possible config options
/// which other crates can use
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct MetabasedConfig {
    #[command(flatten)]
    pub block_builder: BlockBuilderConfig,

    #[command(flatten)]
    pub sequencing: SequencingChainConfig,

    #[command(flatten)]
    pub settlement: SettlementChainConfig,

    #[command(flatten)]
    pub metrics: MetricsConfig,

    /// The delay to be applied to settlement chain blocks (expressed in seconds)
    #[arg(long, env = "SETTLEMENT_DELAY")]
    pub settlement_delay: Option<u64>,

    /// The address of the rollup owner on the settlement chain
    #[arg(long, env = "APPCHAIN_OWNER")]
    pub appchain_owner: Option<Address>,

    /// The chain ID of the Appchain rollup (not the mchain)
    #[arg(long, env = "APPCHAIN_CHAIN_ID")]
    pub appchain_chain_id: u64,

    /// The address of the ConfigManager contract on the settlement chain
    #[arg(
        long = "config-manager-address",
        env = "CONFIG_MANAGER_ADDRESS",
        value_parser = parse_address
    )]
    pub config_manager_address: Option<Address>,

    #[arg(long, env = "APPCHAIN_BLOCK_EXPLORER_URL")]
    pub appchain_block_explorer_url: Option<String>,
}

impl MetabasedConfig {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        <Self as Parser>::parse()
    }

    /// Validate MetabasedConfig
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.block_builder.validate().map_err(ConfigError::BlockBuilder)?;
        self.sequencing.validate().map_err(ConfigError::Ingestor)?;
        self.settlement.validate().map_err(ConfigError::Ingestor)?;
        self.metrics.validate().map_err(ConfigError::Metrics)?;
        Ok(())
    }

    /// Validates the config and ensures all mandatory fields have values (including optional fields
    /// that might have been defined by the `ConfigManager` contract)
    pub fn validate_strict(&self) -> Result<(), ConfigError> {
        self.validate()?;
        self.block_builder.validate_strict().map_err(ConfigError::BlockBuilder)?;
        self.sequencing.validate_strict().map_err(ConfigError::Ingestor)?;
        self.settlement.validate_strict().map_err(ConfigError::Ingestor)?;
        Ok(())
    }

    pub fn generate_sample_command() {
        let mut cmd = String::from("cargo run --bin metabased-translator -- \\\n");

        // Recursively get all fields from flattened configs
        fn add_fields<T: Parser + 'static>(cmd: &mut String) {
            let app = T::command();
            for arg in app.get_arguments() {
                if let Some(long) = arg.get_long() {
                    cmd.push_str(&format!("  --{} <{}> \\\n", long, long.to_uppercase()));
                }
            }
        }

        add_fields::<BlockBuilderConfig>(&mut cmd);
        add_fields::<SequencingChainConfig>(&mut cmd);
        add_fields::<SettlementChainConfig>(&mut cmd);
        add_fields::<MetricsConfig>(&mut cmd);

        // Remove trailing slash and newline
        cmd.truncate(cmd.len() - 2);
        println!("{}", cmd);
    }
}

impl Default for MetabasedConfig {
    fn default() -> Self {
        Self {
            block_builder: BlockBuilderConfig::default(),
            settlement_delay: Some(60),
            appchain_owner: None,
            sequencing: SequencingChainConfig::default(),
            settlement: SettlementChainConfig::default(),
            metrics: MetricsConfig::default(),
            config_manager_address: None,
            appchain_chain_id: 0,
            appchain_block_explorer_url: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZERO: &str = "0x0000000000000000000000000000000000000000";
    const REQUIRED_ENV_VARS: &[(&str, Option<&str>)] = &[
        ("SEQUENCING_RPC_URL", Some("")),
        ("SETTLEMENT_RPC_URL", Some("")),
        ("SEQUENCING_START_BLOCK", Some("1")),
        ("SETTLEMENT_START_BLOCK", Some("1")),
        ("SEQUENCING_CONTRACT_ADDRESS", Some(ZERO)),
        ("ARBITRUM_BRIDGE_ADDRESS", Some(ZERO)),
        ("ARBITRUM_INBOX_ADDRESS", Some(ZERO)),
        ("MCHAIN_RPC_URL", Some("")),
        ("APPCHAIN_CHAIN_ID", Some("1")),
    ];

    #[test]
    fn test_clap_parse() {
        let config = temp_env::with_vars(
            [
                REQUIRED_ENV_VARS,
                &[("SEQUENCING_BUFFER_SIZE", Some("200")), ("SETTLEMENT_BUFFER_SIZE", Some("200"))],
            ]
            .concat(),
            || MetabasedConfig::try_parse_from(["test", "--sequencing-buffer-size", "300"]),
        )
        .unwrap();
        // default value
        assert_eq!(config.settlement.settlement_syncing_batch_size, 50);
        // default value + env var override
        assert_eq!(config.settlement.settlement_buffer_size, 200);
        // defeault value + env var + cli override
        assert_eq!(config.sequencing.sequencing_buffer_size, 300);
    }

    #[test]
    fn test_generate_command() {
        MetabasedConfig::generate_sample_command();
    }
}
