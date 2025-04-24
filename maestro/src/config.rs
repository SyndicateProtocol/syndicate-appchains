//! This module contains `config` for the `maestro` service

use clap::Parser;
use std::time::Duration;

/// Configuration for Maestro
#[allow(clippy::doc_markdown)]
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Port to listen on
    #[arg(short = 'p', long, env = "PORT", default_value_t = 8080)]
    pub port: i32,

    /// Redis address to listen on
    /// Example: "redis://0.0.0.0:6379"
    #[arg(short = 'r', long, env = "REDIS_URL")]
    pub redis_url: String,

    /// Timeout in seconds for connection validation
    #[arg(long, env = "VALIDATION_TIMEOUT", default_value = "5s",
    value_parser = humantime::parse_duration)]
    pub validation_timeout: Duration,

    /// Skip validation of RPC URLs
    #[arg(long, env = "SKIP_VALIDATION", default_value_t = false)]
    pub skip_validation: bool,

    /// Interval at which to prune the Redis stream
    #[arg(long, env = "PRUNE_INTERVAL", default_value = "24h",
    value_parser = humantime::parse_duration)]
    pub prune_interval: Duration,

    /// Redis stream max age of messages to prune
    #[arg(long, env = "PRUNE_MAX_AGE", default_value = "24h",
    value_parser = humantime::parse_duration)]
    pub prune_max_age: Duration,
}

impl Config {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        Self::parse()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            redis_url: String::new(),
            validation_timeout: Duration::from_secs(5),
            skip_validation: false,
            prune_interval: Duration::from_secs(60 * 60 * 24),
            prune_max_age: Duration::from_secs(60 * 60 * 24),
        }
    }
}
