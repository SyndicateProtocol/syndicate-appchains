//! This module contains `config` for the `maestro` service

use clap::Parser;

/// Configuration for the tc sequencer
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Port to listen on
    #[arg(short = 'p', long, env = "PORT", default_value_t = 8080)]
    pub port: i32,

    /// Port to listen on
    #[arg(short = 'r', long, env = "REDIS_ADDRESS", default_value = "0.0.0.0:6379")]
    pub redis_address: String,
}

impl Config {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        Self::parse()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { port: 8080, redis_address: "0.0.0.0:6379".to_string() }
    }
}
