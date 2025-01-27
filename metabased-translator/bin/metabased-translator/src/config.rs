//! Common configuration for the Metabased Translator.
//!
//! This module contains all possible configuration options for the Metabased Translator. Different
//! crates each inherit a subset of these options to configure themselves

use block_builder::config::BlockBuilderConfig;
use clap::Parser;
use ingestor::config::{SequencingChainConfig, SettlementChainConfig};
use slotting::config::SlottingConfig;
use std::fmt::Debug;
use tracing::debug;

/// Common config stuct for the Metabased Translator. This contains all possible config options
/// which other crates can use
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct MetabasedConfig {
    #[command(flatten)]
    pub block_builder: BlockBuilderConfig,

    #[command(flatten)]
    pub slotter: SlottingConfig,

    #[command(flatten)]
    pub sequencing: SequencingChainConfig,

    #[command(flatten)]
    pub settlement: SettlementChainConfig,
}

impl Default for MetabasedConfig {
    fn default() -> Self {
        let config = Self::parse_from(["test"]);
        debug!("Created default MetabasedConfig: {:?}", config);
        config
    }
}

impl MetabasedConfig {
    /// Parse the [`MetabasedConfig`] from configuration sources like CLI args and env vars
    pub fn parse() -> Self {
        let config = <Self as Parser>::parse();
        debug!("Parsed MetabasedConfig: {:?}", config);
        config
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
        add_fields::<SlottingConfig>(&mut cmd);
        add_fields::<SequencingChainConfig>(&mut cmd);
        add_fields::<SettlementChainConfig>(&mut cmd);

        // Remove trailing slash and newline
        cmd.truncate(cmd.len() - 2);
        println!("{}", cmd);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    fn clean_env() {
        // Block Builder
        env::remove_var("BLOCK_BUILDER_PORT");
        env::remove_var("BLOCK_BUILDER_GENESIS_TIMESTAMP");
        env::remove_var("BLOCK_BUILDER_CHAIN_ID");

        // Slotter
        env::remove_var("SLOTTER_SLOT_DURATION_MS");
        env::remove_var("SLOTTER_START_SLOT_NUMBER");
        env::remove_var("SLOTTER_START_SLOT_TIMESTAMP");

        // Sequencer Chain
        env::remove_var("SEQUENCING_BUFFER_SIZE");
        env::remove_var("SEQUENCING_POLLING_INTERVAL_SECS");
        env::remove_var("SEQUENCING_RPC_URL");
        env::remove_var("SEQUENCING_START_BLOCK");

        // Settlement Chain
        env::remove_var("SETTLEMENT_BUFFER_SIZE");
        env::remove_var("SETTLEMENT_POLLING_INTERVAL_SECS");
        env::remove_var("SETTLEMENT_RPC_URL");
        env::remove_var("SETTLEMENT_START_BLOCK");
    }

    #[test]
    #[serial]
    fn test_default_values() {
        clean_env();
        let config = MetabasedConfig::try_parse_from(["test"]).unwrap();

        // Block Builder
        assert_eq!(config.block_builder.port, 8888);
        assert_eq!(config.block_builder.genesis_timestamp, 1712500000);
        assert_eq!(config.block_builder.chain_id, 84532);

        // Slotter
        assert_eq!(config.slotter.slot_duration_ms, 2_000);
        assert_eq!(config.slotter.start_slot_number, 0);
        assert_eq!(config.slotter.start_slot_timestamp, 0);

        // Chains
        assert_eq!(config.sequencing.sequencing_buffer_size, 100);
        assert_eq!(config.sequencing.sequencing_polling_interval_secs, 1);
        assert_eq!(config.sequencing.sequencing_rpc_url, "http://localhost:8545");
        assert_eq!(config.settlement.settlement_buffer_size, 100);
        assert_eq!(config.settlement.settlement_polling_interval_secs, 1);
        assert_eq!(config.settlement.settlement_rpc_url, "http://localhost:8545");
    }

    #[test]
    #[serial]
    fn test_env_vars_override_defaults() {
        clean_env();
        env::set_var("BLOCK_BUILDER_PORT", "9999");
        env::set_var("SLOTTER_SLOT_DURATION_MS", "3000");
        env::set_var("SEQUENCING_BUFFER_SIZE", "200");

        let config = MetabasedConfig::try_parse_from(["test"]).unwrap();
        assert_eq!(config.block_builder.port, 9999);
        assert_eq!(config.slotter.slot_duration_ms, 3000);
        assert_eq!(config.sequencing.sequencing_buffer_size, 200);
    }

    #[test]
    #[serial]
    fn test_cli_args_override_env_vars() {
        clean_env();
        env::set_var("BLOCK_BUILDER_PORT", "9999");

        let config = MetabasedConfig::try_parse_from(["test", "--port", "7777"]).unwrap();
        assert_eq!(config.block_builder.port, 7777);
    }

    #[test]
    fn test_generate_command() {
        MetabasedConfig::generate_sample_command();
    }
}
