//! The `config` module handles configuration parsing for the tc sequencer.

use alloy::primitives::Address;
use batcher::config::BatcherConfig;
use clap::Parser;
use shared::parse::parse_address;
use std::fmt::Debug;
use tc_client::config::TCConfig;

/// Common config stuct for the TC Sequencer.
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct BatchSequencerConfig {
    /// Mapping of chain IDs to their corresponding sequencing addresses
    #[arg(short = 's', long, env = "SEQUENCING_ADDRESS", value_parser = parse_address)]
    pub sequencing_address: Address,

    /// Address of the wallet pool contract
    #[arg(short = 'w', long, env = "WALLET_POOL_ADDRESS", value_parser = parse_address, default_value = "0x9d9E8B09C1f7d9cC1Cdd4a843e695fD580a390E8")]
    pub wallet_pool_address: Address,

    #[command(flatten)]
    /// The batcher config
    pub batcher: BatcherConfig,

    /// Use TC
    #[arg(short = 't', long, env = "USE_TC", default_value_t = false)]
    pub use_tc: bool,
    #[command(flatten)]
    /// The tc client config
    pub tc: Option<TCConfig>,

    /// Metrics port to listen on
    #[arg(short = 'm', long, env = "METRICS_PORT", default_value_t = 8082)]
    pub metrics_port: u16,
}

impl BatchSequencerConfig {
    /// Initialize the config from the CLI arguments and environment variables.
    pub fn initialize() -> Self {
        let config = <Self as Parser>::parse();
        assert!(!config.use_tc || config.tc.is_some(), "TCConfig is required when use_tc is true");
        config
    }
}

impl Default for BatchSequencerConfig {
    fn default() -> Self {
        Self::initialize()
    }
}
