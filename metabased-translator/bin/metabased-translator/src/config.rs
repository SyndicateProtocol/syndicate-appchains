//! Common configuration for the Metabased Translator.
//!
//! This module contains all possible configuration options for the Metabased Translator. Different
//! crates each inherit a subset of these options to configure themselves

use block_builder::config::BlockBuilderConfig;
use clap::Parser;
use eyre::Result;
use ingestor::{
    config::{ChainIngestorConfig, SequencingChainConfig, SettlementChainConfig},
    eth_client::{RPCClient, RPCClientError},
};
use metrics::config::MetricsConfig;
use slotter::config::SlotterConfig;
use std::{fmt::Debug, sync::Arc};
use thiserror::Error;
use tracing::{debug, error};

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(
        "Settlement chain start timestamp ({0}) is greater than sequencing chain timestamp ({1})"
    )]
    SettlementStartBlockTooLate(u64, u64),

    #[error("Failed to fetch block data: {0}")]
    RPCClient(#[from] RPCClientError),
}

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

    pub async fn set_initial_timestamp(
        &mut self,
        settlement_client: &Arc<dyn RPCClient>,
        sequencing_client: &Arc<dyn RPCClient>,
    ) -> Result<(), ConfigError> {
        let seq_start_block = sequencing_client
            .get_block_by_number(self.sequencing.sequencing_start_block)
            .await
            .map_err(ConfigError::RPCClient)?;

        let set_start_block = settlement_client
            .get_block_by_number(self.settlement.settlement_start_block)
            .await
            .map_err(ConfigError::RPCClient)?;

        if seq_start_block.timestamp < set_start_block.timestamp {
            return Err(ConfigError::SettlementStartBlockTooLate(
                set_start_block.timestamp,
                seq_start_block.timestamp,
            ));
        }

        self.slotter.start_slot_timestamp = set_start_block.timestamp;
        self.block_builder.genesis_timestamp = set_start_block.timestamp;
        debug!("Genesis timestamp set to: {:?}", set_start_block.timestamp);
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
            datadir: "./datadir".to_string(),
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
    use async_trait::async_trait;
    use common::types::{Block, BlockAndReceipts};
    use eyre::Result;
    use mockall::{mock, predicate::*};
    use serial_test::serial;
    use std::{env, time::Duration};
    use tokio;

    fn clean_env() {
        // Block Builder
        env::remove_var("BLOCK_BUILDER_MCHAIN_URL");
        env::remove_var("BLOCK_BUILDER_CHAIN_ID");
        env::remove_var("BLOCK_BUILDER_SEQUENCING_CONTRACT_ADDRESS");
        env::remove_var("BLOCK_BUILDER_ARBITRUM_BRIDGE_ADDRESS");
        env::remove_var("BLOCK_BUILDER_ARBITRUM_INBOX_ADDRESS");

        // Slotter
        env::remove_var("SLOTTER_SLOT_DURATION");

        // Sequencer Chain
        env::remove_var("SEQUENCING_BUFFER_SIZE");
        env::remove_var("SEQUENCING_POLLING_INTERVAL");
        env::remove_var("SEQUENCING_RPC_URL");
        env::remove_var("SEQUENCING_START_BLOCK");

        // Settlement Chain
        env::remove_var("SETTLEMENT_BUFFER_SIZE");
        env::remove_var("SETTLEMENT_POLLING_INTERVAL");
        env::remove_var("SETTLEMENT_RPC_URL");
        env::remove_var("SETTLEMENT_START_BLOCK");

        // Metrics
        env::remove_var("METRICS_PORT");
    }

    #[test]
    #[serial]
    fn test_default_values() {
        clean_env();
        let zero = "0x0000000000000000000000000000000000000000";
        env::set_var("SEQUENCING_RPC_URL", "");
        env::set_var("SETTLEMENT_RPC_URL", "");
        env::set_var("SEQUENCING_START_BLOCK", "1");
        env::set_var("SETTLEMENT_START_BLOCK", "1");
        env::set_var("BLOCK_BUILDER_SEQUENCING_CONTRACT_ADDRESS", zero);
        env::set_var("BLOCK_BUILDER_ARBITRUM_BRIDGE_ADDRESS", zero);
        env::set_var("BLOCK_BUILDER_ARBITRUM_INBOX_ADDRESS", zero);
        let config = MetabasedConfig::try_parse_from(["test"]).unwrap();

        // Block Builder
        assert_eq!(config.block_builder.mchain_url.as_str(), "http://127.0.0.1:8888/");
        assert_eq!(config.block_builder.target_chain_id, 13331370);

        // Slotter
        assert_eq!(config.slotter.slot_duration, 2);

        // Chains
        assert_eq!(config.sequencing.sequencing_buffer_size, 100);
        assert_eq!(config.sequencing.sequencing_polling_interval, Duration::from_secs(1));
        assert_eq!(config.sequencing.sequencing_rpc_url, "");
        assert_eq!(config.settlement.settlement_buffer_size, 100);
        assert_eq!(config.settlement.settlement_polling_interval, Duration::from_secs(1));
        assert_eq!(config.settlement.settlement_rpc_url, "");

        // Metrics
        assert_eq!(config.metrics.metrics_port, 9090)
    }

    #[test]
    #[serial]
    fn test_env_vars_override_defaults() {
        clean_env();
        let zero = "0x0000000000000000000000000000000000000000";
        env::set_var("BLOCK_BUILDER_MCHAIN_URL", "http://127.0.0.1:9999/");
        env::set_var("SLOTTER_SLOT_DURATION", "3");
        env::set_var("SEQUENCING_BUFFER_SIZE", "200");
        env::set_var("SEQUENCING_RPC_URL", "");
        env::set_var("SETTLEMENT_RPC_URL", "");
        env::set_var("SEQUENCING_START_BLOCK", "1");
        env::set_var("SETTLEMENT_START_BLOCK", "1");
        env::set_var("BLOCK_BUILDER_SEQUENCING_CONTRACT_ADDRESS", zero);
        env::set_var("BLOCK_BUILDER_ARBITRUM_BRIDGE_ADDRESS", zero);
        env::set_var("BLOCK_BUILDER_ARBITRUM_INBOX_ADDRESS", zero);

        let config = MetabasedConfig::try_parse_from(["test"]).unwrap();
        assert_eq!(config.block_builder.mchain_url.as_str(), "http://127.0.0.1:9999/");
        assert_eq!(config.slotter.slot_duration, 3);
        assert_eq!(config.sequencing.sequencing_buffer_size, 200);
    }

    #[test]
    #[serial]
    fn test_cli_args_override_env_vars() {
        clean_env();
        let zero = "0x0000000000000000000000000000000000000000";
        env::set_var("BLOCK_BUILDER_MCHAIN_URL", "http://127.0.0.1:9999/");
        env::set_var("SEQUENCING_RPC_URL", "");
        env::set_var("SETTLEMENT_RPC_URL", "");
        env::set_var("SEQUENCING_START_BLOCK", "1");
        env::set_var("SETTLEMENT_START_BLOCK", "1");
        env::set_var("BLOCK_BUILDER_SEQUENCING_CONTRACT_ADDRESS", zero);
        env::set_var("BLOCK_BUILDER_ARBITRUM_BRIDGE_ADDRESS", zero);
        env::set_var("BLOCK_BUILDER_ARBITRUM_INBOX_ADDRESS", zero);

        let config =
            MetabasedConfig::try_parse_from(["test", "-u", "http://127.0.0.1:7777/"]).unwrap();
        assert_eq!(config.block_builder.mchain_url.as_str(), "http://127.0.0.1:7777/");
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
            async fn get_block_by_number(&self, block_number: u64) -> Result<Block, RPCClientError>;
            async fn get_block_and_receipts(&self, block_number: u64) -> Result<BlockAndReceipts, RPCClientError>;

        }
    }

    #[tokio::test]
    async fn test_set_initial_timestamp_success() {
        let mut config = MetabasedConfig::parse();
        config.settlement.settlement_start_block = 100;
        config.sequencing.sequencing_start_block = 200;

        let mut mock_settlement_client = MockRPCClientMock::new();
        let mut mock_sequencing_client = MockRPCClientMock::new();

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

        let settlement_client: Arc<dyn RPCClient> = Arc::new(mock_settlement_client);
        let sequencing_client: Arc<dyn RPCClient> = Arc::new(mock_sequencing_client);

        let result = config.set_initial_timestamp(&settlement_client, &sequencing_client).await;

        assert!(result.is_ok());
        assert_eq!(config.slotter.start_slot_timestamp, 6000);
        assert_eq!(config.block_builder.genesis_timestamp, 6000);
    }

    #[tokio::test]
    async fn test_set_initial_timestamp_fail() {
        let mut config = MetabasedConfig::parse();
        config.settlement.settlement_start_block = 100;
        config.sequencing.sequencing_start_block = 200;

        let mut mock_settlement_client = MockRPCClientMock::new();
        let mut mock_sequencing_client = MockRPCClientMock::new();

        mock_settlement_client
            .expect_get_block_by_number()
            .with(eq(100))
            .times(1)
            .returning(|_| Ok(Block { timestamp: 6000, ..Default::default() }));
        mock_sequencing_client
            .expect_get_block_by_number()
            .with(eq(200))
            .times(1)
            .returning(|_| Ok(Block { timestamp: 5000, ..Default::default() }));

        let settlement_client: Arc<dyn RPCClient> = Arc::new(mock_settlement_client);
        let sequencing_client: Arc<dyn RPCClient> = Arc::new(mock_sequencing_client);

        let result = config.set_initial_timestamp(&settlement_client, &sequencing_client).await;

        assert!(result.is_err());
    }
}
