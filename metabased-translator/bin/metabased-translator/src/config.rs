//! Common configuration for the Metabased Translator.
//!
//! This module contains all possible configuration options for the Metabased Translator. Different
//! crates each inherit a subset of these options to configure themselves

use block_builder::config::BlockBuilderConfig;
use clap::Parser;
use eyre::{eyre, Error, Result};
use ingestor::{
    config::{SequencingChainConfig, SettlementChainConfig},
    eth_client::RPCClient,
};
use metrics::config::MetricsConfig;
use slotting::config::SlottingConfig;
use std::{fmt::Debug, sync::Arc};
use tracing::debug;

/// Common config stuct for the Metabased Translator. This contains all possible config options
/// which other crates can use
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct MetabasedConfig {
    #[arg(long, env = "DATADIR", default_value = "./datadir")]
    pub datadir: String,

    #[command(flatten)]
    pub block_builder: BlockBuilderConfig,

    #[command(flatten)]
    pub slotter: SlottingConfig,

    #[command(flatten)]
    pub sequencing: SequencingChainConfig,

    #[command(flatten)]
    pub settlement: SettlementChainConfig,

    #[command(flatten)]
    pub metrics: MetricsConfig,
}

impl Default for MetabasedConfig {
    fn default() -> Self {
        let config = Self::parse_from([""]);
        debug!("Created default MetabasedConfig: {:?}", config);
        config
    }
}

impl MetabasedConfig {
    /// Initializes the configuration, fetching the earliest block timestamp dynamically.
    pub async fn initialize() -> Result<Self, Error> {
        let config = <Self as Parser>::parse();
        Ok(config)
    }

    pub async fn set_initial_timestamp(
        &mut self,
        settlement_client: Arc<dyn RPCClient>,
        sequencing_client: Arc<dyn RPCClient>,
    ) -> Result<()> {
        let seq_start_timestamp =
            fetch_block_timestamp(sequencing_client, self.sequencing.sequencing_start_block)
                .await?;
        let set_start_timestamp =
            fetch_block_timestamp(settlement_client, self.settlement.settlement_start_block)
                .await?;

        if seq_start_timestamp < set_start_timestamp {
            return Err(eyre!(
            "Invalid blockchain state: settlement chain initial timestamp ({}) is greater than sequencing chain initial timestamp ({})",
            set_start_timestamp,
            seq_start_timestamp
        ));
        }
        // Set start_slot_timestamp to the minimum of both chains
        self.slotter.start_slot_timestamp = set_start_timestamp;
        self.block_builder.genesis_timestamp = set_start_timestamp;

        Ok(())
    }

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
        add_fields::<MetricsConfig>(&mut cmd);

        // Remove trailing slash and newline
        cmd.truncate(cmd.len() - 2);
        println!("{}", cmd);
    }
}

async fn fetch_block_timestamp(
    client: Arc<dyn RPCClient>,
    block_number: u64,
) -> Result<u64, Error> {
    let block = client.get_block_by_number(block_number).await?;
    // Ethereum timestamps are in seconds, convert to milliseconds.
    Ok(block.timestamp * 1000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use common::types::{Block, Receipt};
    use mockall::{mock, predicate::*};
    use serial_test::serial;
    use std::{env, time::Duration};
    use tokio::test;

    fn clean_env() {
        // Block Builder
        env::remove_var("BLOCK_BUILDER_MCHAIN_URL");
        env::remove_var("BLOCK_BUILDER_CHAIN_ID");

        // Slotter
        env::remove_var("SLOTTER_SLOT_DURATION_MS");
        env::remove_var("SLOTTER_START_SLOT_NUMBER");

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

        // Metrics
        env::remove_var("METRICS_PORT");
    }

    #[test]
    #[serial]
    async fn test_default_values() {
        clean_env();
        let config = MetabasedConfig::try_parse_from(["test"]).unwrap();

        // Block Builder
        assert_eq!(config.block_builder.mchain_url.as_str(), "http://127.0.0.1:8888/");
        assert_eq!(config.block_builder.target_chain_id, 13331370);

        // Slotter
        assert_eq!(config.slotter.slot_duration_ms, 2_000);

        // Chains
        assert_eq!(config.sequencing.sequencing_buffer_size, 100);
        assert_eq!(config.sequencing.sequencing_polling_interval, Duration::from_secs(1));
        assert_eq!(config.sequencing.sequencing_rpc_url, "http://localhost:8545");
        assert_eq!(config.settlement.settlement_buffer_size, 100);
        assert_eq!(config.settlement.settlement_polling_interval, Duration::from_secs(1));
        assert_eq!(config.settlement.settlement_rpc_url, "http://localhost:8546");

        // Metrics
        assert_eq!(config.metrics.metrics_port, 9090)
    }

    #[test]
    #[serial]
    async fn test_env_vars_override_defaults() {
        clean_env();
        env::set_var("BLOCK_BUILDER_MCHAIN_URL", "http://127.0.0.1:9999/");
        env::set_var("SLOTTER_SLOT_DURATION_MS", "3000");
        env::set_var("SEQUENCING_BUFFER_SIZE", "200");

        let config = MetabasedConfig::try_parse_from(["test"]).unwrap();
        assert_eq!(config.block_builder.mchain_url.as_str(), "http://127.0.0.1:9999/");
        assert_eq!(config.slotter.slot_duration_ms, 3000);
        assert_eq!(config.sequencing.sequencing_buffer_size, 200);
    }

    #[test]
    #[serial]
    async fn test_cli_args_override_env_vars() {
        clean_env();
        env::set_var("BLOCK_BUILDER_MCHAIN_URL", "http://127.0.0.1:9999/");

        let config =
            MetabasedConfig::try_parse_from(["test", "-u", "http://127.0.0.1:7777/"]).unwrap();
        assert_eq!(config.block_builder.mchain_url.as_str(), "http://127.0.0.1:7777/");
    }

    #[test]
    async fn test_generate_command() {
        MetabasedConfig::generate_sample_command();
    }

    // Mock RPCClient
    mock! {
        #[derive(Debug)]
        pub RPCClientMock {}

        #[async_trait]
        impl RPCClient for RPCClientMock {
            async fn get_block_by_number(&self, block_number: u64) -> Result<Block, Error>;
            async fn get_block_receipts(&self, block_number: u64) -> Result<Vec<Receipt>, Error>;
        }
    }

    #[tokio::test]
    async fn test_set_initial_timestamp_success() {
        let mut config = MetabasedConfig::default();
        config.settlement.settlement_start_block = 100;
        config.sequencing.sequencing_start_block = 200;

        let mut mock_settlement_client = MockRPCClientMock::new();
        let mut mock_sequencing_client = MockRPCClientMock::new();

        // Mock responses
        mock_settlement_client
            .expect_get_block_by_number()
            .with(eq(100))
            .times(1)
            .returning(|_| Ok(Block { timestamp: 6000, ..Default::default() }));

        mock_sequencing_client
            .expect_get_block_by_number()
            .with(eq(200))
            .times(1)
            .returning(|_| Ok(Block { timestamp: 6000, ..Default::default() }));

        // Run the function
        let settlement_client = Arc::new(mock_settlement_client);
        let sequencing_client = Arc::new(mock_sequencing_client);
        let result = config.set_initial_timestamp(settlement_client, sequencing_client).await;

        assert!(result.is_ok());
        assert_eq!(config.slotter.start_slot_timestamp, 6000000);
        assert_eq!(config.block_builder.genesis_timestamp, 6000000);
    }
}
