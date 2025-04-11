//! This module contains `config` for the `maestro` service

use crate::errors::ConfigError;
use alloy::transports::http::Client;
use clap::Parser;
use serde_json::Value;
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
        default_value = "{}",
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

        match self.ping_nitro_urls().await {
            Ok(_) => {
                debug!("All Nitro URLs validated successfully");
                Ok(())
            }
            Err(e) => {
                error!(%e, "Nitro URL validation failed");
                Err(e)
            }
        }
    }

    /// Checks that all Nitro URLs are accessible by making a test connection
    async fn ping_nitro_urls(&self) -> Result<(), ConfigError> {
        // Validate Nitro URLs by trying to connect to each one
        if self.chain_rpc_urls.is_empty() {
            return Ok(())
        }

        // JSON-RPC request payload for eth_getBlockByNumber
        let health_check_payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_chainId",
            "params": [],
            "id": 1
        });

        let client = Client::builder()
            .timeout(self.validation_timeout)
            .build()
            .map_err(ConfigError::HttpClient)?;

        for (chain_id, url) in &self.chain_rpc_urls {
            debug!(%chain_id, %url, "Sending test JSON-RPC request");

            let response = client
                .post(url)
                .header("Content-Type", "application/json")
                .json(&health_check_payload)
                .send()
                .await
                .map_err(|_| ConfigError::RpcUrlConnection(chain_id.clone(), url.clone()))?;

            // Check for successful status code (2xx)
            if !response.status().is_success() {
                return Err(ConfigError::RpcUrlInvalidStatus(
                    chain_id.clone(),
                    url.clone(),
                    response.status().to_string(),
                ));
            }

            // check that result chain_id matches ours
            let json: Value = response.json().await?;
            let hex_chain_id = json["result"].as_str().unwrap_or("");
            let dec_chain_id = hex_to_decimal(hex_chain_id).unwrap_or(0);

            if *chain_id != dec_chain_id.to_string() {
                return Err(ConfigError::RpcUrlInvalidChainId(
                    url.clone(),
                    chain_id.to_string(),
                    dec_chain_id.to_string(),
                ));
            }

            debug!(%chain_id, %url, "Successful JSON-RPC request");
        }
        Ok(())
    }
}

fn hex_to_decimal(hex_str: &str) -> Result<u64, std::num::ParseIntError> {
    // Remove "0x" prefix if present
    let cleaned_hex = hex_str.trim_start_matches("0x");
    u64::from_str_radix(cleaned_hex, 16)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_decimal_with_prefix() {
        assert_eq!(hex_to_decimal("0x7c830").unwrap(), 510000);
        assert_eq!(hex_to_decimal("0x1A").unwrap(), 26);
        assert_eq!(hex_to_decimal("0xdeadbeef").unwrap(), 3735928559);
        assert_eq!(hex_to_decimal("0x0").unwrap(), 0);
    }

    #[test]
    fn test_hex_to_decimal_without_prefix() {
        assert_eq!(hex_to_decimal("7c830").unwrap(), 510000);
        assert_eq!(hex_to_decimal("1A").unwrap(), 26);
        assert_eq!(hex_to_decimal("deadbeef").unwrap(), 3735928559);
        assert_eq!(hex_to_decimal("0").unwrap(), 0);
    }

    #[test]
    fn test_hex_to_decimal_case_insensitive() {
        assert_eq!(hex_to_decimal("0xDeadBeef").unwrap(), 3735928559);
        assert_eq!(hex_to_decimal("0xDEADBEEF").unwrap(), 3735928559);
        assert_eq!(hex_to_decimal("0xdeadbeef").unwrap(), 3735928559);
    }

    #[test]
    fn test_hex_to_decimal_error_cases() {
        assert!(hex_to_decimal("0xG").is_err()); // Invalid hex character
        assert!(hex_to_decimal("").is_err()); // Empty string
        assert!(hex_to_decimal("0x").is_err()); // Only prefix

        // Test value too large for u64
        assert!(hex_to_decimal("0xFFFFFFFFFFFFFFFFFF").is_err());
    }
}
