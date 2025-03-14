//! Common configuration for the Metabased Translator.
//!
//! This module contains all possible configuration options for the Metabased Translator. Different
//! crates each inherit a subset of these options to configure themselves

use alloy::rpc::types::BlockNumberOrTag;
use block_builder::config::BlockBuilderConfig;
use clap::Parser;
use common::eth_client::{RPCClient, RPCClientError};
use eyre::Result;
use ingestor::config::{ChainIngestorConfig, SequencingChainConfig, SettlementChainConfig};
use metrics::config::MetricsConfig;
use slotter::config::SlotterConfig;
use std::{fmt::Debug, sync::Arc};
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

    pub async fn set_initial_timestamp(
        &mut self,
        settlement_client: &Arc<dyn RPCClient>,
        sequencing_client: &Arc<dyn RPCClient>,
    ) -> Result<(), ConfigError> {
        let seq_start_block = sequencing_client
            .get_block_by_number(BlockNumberOrTag::Number(self.sequencing.sequencing_start_block))
            .await
            .map_err(ConfigError::RPCClient)?;

        let set_start_block = settlement_client
            .get_block_by_number(BlockNumberOrTag::Number(self.settlement.settlement_start_block))
            .await
            .map_err(ConfigError::RPCClient)?;

        if seq_start_block.timestamp < set_start_block.timestamp {
            return Err(ConfigError::SettlementStartBlockTooLate(
                set_start_block.timestamp,
                seq_start_block.timestamp,
            ));
        }

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
    use common::types::{Block, BlockAndReceipts};
    use eyre::Result;
    use mockall::{mock, predicate::*};
    use tokio;

    #[test]
    fn test_clap_parse() {
        let zero = "0x0000000000000000000000000000000000000000";
        let config = temp_env::with_vars(
            [
                ("SEQUENCING_RPC_URL", Some("")),
                ("SETTLEMENT_RPC_URL", Some("")),
                ("SEQUENCING_START_BLOCK", Some("1")),
                ("SETTLEMENT_START_BLOCK", Some("1")),
                ("BLOCK_BUILDER_SEQUENCING_CONTRACT_ADDRESS", Some(zero)),
                ("BLOCK_BUILDER_ARBITRUM_BRIDGE_ADDRESS", Some(zero)),
                ("BLOCK_BUILDER_ARBITRUM_INBOX_ADDRESS", Some(zero)),
                ("BLOCK_BUILDER_MCHAIN_IPC_PATH", Some("")),
                ("BLOCK_BUILDER_MCHAIN_AUTH_IPC_PATH", Some("")),
                ("SEQUENCING_BUFFER_SIZE", Some("200")),
                ("SETTLEMENT_BUFFER_SIZE", Some("200")),
            ],
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

    #[tokio::test]
    async fn test_set_initial_timestamp_success() {
        let mut config = MetabasedConfig::parse();
        config.settlement.settlement_start_block = 100;
        config.sequencing.sequencing_start_block = 200;

        let mut mock_settlement_client = MockRPCClientMock::new();
        let mut mock_sequencing_client = MockRPCClientMock::new();

        mock_settlement_client
            .expect_get_block_by_number()
            .with(eq(BlockNumberOrTag::Number(100)))
            .times(1)
            .returning(|_| Ok(Block { timestamp: 6000, ..Default::default() }));

        mock_sequencing_client
            .expect_get_block_by_number()
            .with(eq(BlockNumberOrTag::Number(200)))
            .times(1)
            .returning(|_| Ok(Block { timestamp: 6000, ..Default::default() }));

        let settlement_client: Arc<dyn RPCClient> = Arc::new(mock_settlement_client);
        let sequencing_client: Arc<dyn RPCClient> = Arc::new(mock_sequencing_client);

        let result = config.set_initial_timestamp(&settlement_client, &sequencing_client).await;

        assert!(result.is_ok());
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
            .with(eq(BlockNumberOrTag::Number(100)))
            .times(1)
            .returning(|_| Ok(Block { timestamp: 6000, ..Default::default() }));
        mock_sequencing_client
            .expect_get_block_by_number()
            .with(eq(BlockNumberOrTag::Number(200)))
            .times(1)
            .returning(|_| Ok(Block { timestamp: 5000, ..Default::default() }));

        let settlement_client: Arc<dyn RPCClient> = Arc::new(mock_settlement_client);
        let sequencing_client: Arc<dyn RPCClient> = Arc::new(mock_sequencing_client);

        let result = config.set_initial_timestamp(&settlement_client, &sequencing_client).await;

        assert!(result.is_err());
    }
}
