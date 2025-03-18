//! Common configuration for the Metabased Translator.
//!
//! This module contains all possible configuration options for the Metabased Translator. Different
//! crates each inherit a subset of these options to configure themselves

use block_builder::config::BlockBuilderConfig;
use clap::Parser;
use common::eth_client::RPCClientError;
use eyre::Result;
use ingestor::config::{ChainIngestorConfig, SequencingChainConfig, SettlementChainConfig};
use metrics::config::MetricsConfig;
use slotter::config::SlotterConfig;
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

    #[error("Slotter configuration error: {0}")]
    Slotter(#[from] slotter::config::ConfigError),

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
    pub slotter: SlotterConfig,

    #[command(flatten)]
    pub sequencing: SequencingChainConfig,

    #[command(flatten)]
    pub settlement: SettlementChainConfig,

    #[command(flatten)]
    pub metrics: MetricsConfig,

    #[arg(long, env = "RESTORE_FROM_SAFE_STATE", default_value = "false")]
    pub restore_from_safe_state: bool,
}

impl MetabasedConfig {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        <Self as Parser>::parse()
    }

    /// Validate MetabasedConfig
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.block_builder.validate().map_err(ConfigError::BlockBuilder)?;
        self.slotter.validate().map_err(ConfigError::Slotter)?;
        self.sequencing.validate().map_err(ConfigError::Ingestor)?;
        self.settlement.validate().map_err(ConfigError::Ingestor)?;
        self.metrics.validate().map_err(ConfigError::Metrics)?;
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
        add_fields::<SlotterConfig>(&mut cmd);
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
        let ingestor_config = ChainIngestorConfig::default();
        Self {
            block_builder: BlockBuilderConfig::default(),
            slotter: SlotterConfig::default(),
            sequencing: ingestor_config.clone().into(),
            settlement: ingestor_config.into(),
            metrics: MetricsConfig::default(),
            restore_from_safe_state: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::rpc::types::BlockNumberOrTag;
    use async_trait::async_trait;
    use common::{
        eth_client::RPCClient,
        types::{Block, BlockAndReceipts},
    };
    use eyre::Result;
    use mockall::{mock, predicate::*};

    const ZERO: &str = "0x0000000000000000000000000000000000000000";
    const REQUIRED_ENV_VARS: &[(&str, Option<&str>)] = &[
        ("SEQUENCING_RPC_URL", Some("")),
        ("SETTLEMENT_RPC_URL", Some("")),
        ("SEQUENCING_START_BLOCK", Some("1")),
        ("SETTLEMENT_START_BLOCK", Some("1")),
        ("SEQUENCING_CONTRACT_ADDRESS", Some(ZERO)),
        ("ARBITRUM_BRIDGE_ADDRESS", Some(ZERO)),
        ("ARBITRUM_INBOX_ADDRESS", Some(ZERO)),
        ("MCHAIN_IPC_PATH", Some("")),
        ("MCHAIN_AUTH_IPC_PATH", Some("")),
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

    mock! {
        #[derive(Debug)]
        pub RPCClientMock {}

        #[async_trait]
        impl RPCClient for RPCClientMock {
            async fn get_block_by_number(&self, block_number: BlockNumberOrTag) -> Result<Block, RPCClientError>;
            async fn batch_get_blocks_and_receipts(&self, block_numbers: Vec<u64>) -> Result<Vec<BlockAndReceipts>, RPCClientError>;
        }
    }
}
