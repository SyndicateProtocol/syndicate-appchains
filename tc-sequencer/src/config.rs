//! The `config` module handles configuration parsing for the tc sequencer.

use batcher::config::BatcherConfig;
use clap::Parser;
use std::fmt::Debug;
use tc_client::config::TCConfig;

/// Common config stuct for the TC Sequencer.
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct TCSequencerConfig {
    #[command(flatten)]
    /// The batcher config
    pub batcher: BatcherConfig,
    #[command(flatten)]
    /// The tc client config
    pub tc: TCConfig,
}

impl TCSequencerConfig {
    /// Initialize the config from the CLI arguments and environment variables.
    pub fn initialize() -> Self {
        <Self as Parser>::parse()
    }
}

impl Default for TCSequencerConfig {
    fn default() -> Self {
        Self::initialize()
    }
}
