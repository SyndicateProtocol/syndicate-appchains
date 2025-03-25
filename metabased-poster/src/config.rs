//! The `config` module handles configuration parsing for the metabased poster.

use alloy::primitives::Address;
use clap::Parser;
use std::{fmt::Debug, str::FromStr, time::Duration};
use thiserror::Error;
use url::Url;

/// Error type for configuration errors
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidURL(String),
    /// Invalid URL: no host
    #[error("Invalid URL: no host")]
    InvalidURLHost,
    /// Invalid URL scheme
    #[error("Invalid URL scheme: {0}. Only http and https are supported")]
    InvalidURLScheme(String),
}

/// Configuration for the metabased sequencer
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// URL of the settlement chain RPC node
    #[arg(short = 's', long, env = "SETTLEMENT_RPC_URL", value_parser = parse_url)]
    pub settlement_rpc_url: Url,

    /// URL of the app-chain RPC node
    #[arg(short = 'a', long, env = "APPCHAIN_RPC_URL", value_parser = parse_url)]
    pub appchain_rpc_url: Url,

    /// Address of the assertion poster contract
    #[arg(short = 'b', long, env = "ASSERTION_POSTER_CONTRACT_ADDRESS", value_parser = parse_address)]
    pub assertion_poster_contract_address: Address,

    /// Private key    
    #[arg(short = 'k', long, env = "POSTER_PRIVATE_KEY")]
    pub private_key: String,

    /// The interval between each block polling
    #[arg( long, env = "POSTER_POLLING_INTERVAL", default_value = "10m", value_parser = humantime::parse_duration )]
    pub polling_interval: Duration,

    /// Port for metrics
    #[arg(short = 'm', long, env = "POSTER_METRICS_PORT", default_value_t = 9191)]
    pub metrics_port: u16,
}

/// Parse a string into an Ethereum `Address`.
fn parse_address(value: &str) -> Result<Address, String> {
    Address::from_str(value).map_err(|_| format!("Invalid address: {}", value))
}

/// Parse default string into a valid [`URL`].
fn parse_url(value: &str) -> Result<Url, ConfigError> {
    Url::parse(value).map_or_else(
        |_err| Err(ConfigError::InvalidURL(value.to_string())),
        |url| {
            if !url.has_host() {
                return Err(ConfigError::InvalidURLHost);
            }
            match url.scheme() {
                "http" | "https" => Ok(url),
                _ => Err(ConfigError::InvalidURLScheme(url.scheme().to_string())),
            }
        },
    )
}

impl Config {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        Self::parse()
    }
}
