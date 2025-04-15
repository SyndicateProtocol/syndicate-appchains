//! This module contains `config` for the `maestro` service

use crate::errors::ConfigError;
use alloy::providers::{Provider, ProviderBuilder};
use clap::Parser;
use shared::parse::parse_map;
use std::{collections::HashMap, time::Duration};
use tracing::{debug, error};

/// Configuration for Maestro
#[allow(clippy::doc_markdown)]
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Port to listen on
    #[arg(short = 'p', long, env = "PORT", default_value_t = 8080)]
    pub port: i32,

    /// Redis address to listen on
    /// Example: "0.0.0.0:6379"
    #[arg(short = 'r', long, env = "REDIS_ADDRESS")]
    pub redis_address: Option<String>,

    /// Chain ID to RPC URL mappings as a JSON string object
    /// Example: '{"1": "https://example.com", "2": "https://another.com"}'
    #[arg(
        short = 'c',
        long,
        env = "CHAIN_RPC_URLS",
        value_parser = parse_map::<String, String>
    )]
    pub chain_rpc_urls: HashMap<String, String>,

    /// Timeout in seconds for connection validation
    #[arg(long, env = "VALIDATION_TIMEOUT", default_value = "5s",
    value_parser = humantime::parse_duration)]
    pub validation_timeout: Duration,

    /// Skip validation of RPC URLs
    #[arg(long, env = "SKIP_VALIDATION", default_value_t = false)]
    pub skip_validation: bool,
}

impl Config {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        Self::parse()
    }

    /// Validates the configuration
    pub async fn validate(&self) -> Result<(), ConfigError> {
        // Skip validation if requested
        if self.skip_validation {
            debug!("Skipping config validation");
            return Ok(());
        }

        match self.ping_rpc_urls().await {
            Ok(_) => {
                debug!("All RPC URLs validated successfully");
                Ok(())
            }
            Err(e) => {
                error!(%e, "RPC URL validation failed");
                Err(e)
            }
        }
    }

    /// Checks that all RPC URLs are accessible by making a test connection
    async fn ping_rpc_urls(&self) -> Result<(), ConfigError> {
        // Validate RPC URLs by trying to connect to each one
        if self.chain_rpc_urls.is_empty() {
            return Ok(())
        }

        for (chain_id, url) in &self.chain_rpc_urls {
            debug!(%chain_id, %url, "Sending test JSON-RPC request");

            let provider = ProviderBuilder::new().connect(url).await?;
            let resp_chain_id = provider.get_chain_id().await?;

            if resp_chain_id.to_string() != *chain_id {
                return Err(ConfigError::RpcUrlInvalidChainId(
                    url.clone(),
                    chain_id.to_string(),
                    resp_chain_id.to_string(),
                ));
            }
            debug!(%chain_id, %url, "Successful JSON-RPC request");
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            redis_address: None,
            chain_rpc_urls: HashMap::new(),
            validation_timeout: Duration::from_secs(5),
            skip_validation: false,
        }
    }
}
