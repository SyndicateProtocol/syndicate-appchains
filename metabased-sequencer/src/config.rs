//! The `config` module handles configuration parsing for the metabased sequencer.

use alloy::primitives::{Address, B256};
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
    /// Address of the sequencing contract
    #[arg(short = 'c', long, env = "SEQUENCING_CONTRACT_ADDRESS", value_parser = parse_address)]
    pub chain_contract_address: Address,

    /// URL of the sequencing chain RPC node
    #[arg(short = 'r', long, env = "SEQUENCING_CHAIN_RPC_URL", value_parser = parse_url)]
    pub chain_rpc_url: Url,

    /// Port to listen on
    #[arg(short = 'p', long, env = "SEQUENCER_PORT", default_value_t = 8456)]
    pub port: u16,

    /// Port for metrics
    #[arg(short = 'm', long, env = "SEQUENCER_METRICS_PORT", default_value_t = 9191)]
    pub metrics_port: u16,

    /// Private key for signing transactions    
    #[arg(short = 'k', long, env = "SEQUENCER_PRIVATE_KEY")]
    pub private_key: B256,

    /// Confirmations to wait for when relaying transaction
    #[arg(long, env = "SEQUENCER_CONFIRMATIONS", default_value_t = 2)]
    pub tx_confirmations: u64,

    /// Timeout when relaying transaction
    #[arg(
        long,
        env = "SEQUENCER_TIMEOUT",
        default_value = "60s",
        value_parser = humantime::parse_duration
    )]
    pub tx_timeout: Duration,
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
