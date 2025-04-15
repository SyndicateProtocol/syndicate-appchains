//! This module contains `config` for the `Batcher` service

use alloy::transports::http::{reqwest::Error as ReqwestError, Client};
use clap::Parser;
use serde_json::Value;
use shared::parse::parse_map;
use std::collections::HashMap;
use thiserror::Error;
use tracing::{debug, error};

/// Errors that can occur during Batcher configuration or validation
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Failed to create the HTTP client
    #[error("failed to build HTTP client: {0}")]
    HttpClient(#[from] ReqwestError),

    /// Failed to connect to the given RPC URL
    #[error("failed to connect to RPC URL for chain {0} at {1}")]
    RpcUrlConnection(String, String),

    /// RPC endpoint responded with a non-success status code
    #[error("invalid HTTP status from RPC URL {1} for chain {0}: {2}")]
    RpcUrlInvalidStatus(String, String, String),

    /// RPC endpoint returned a mismatched chain ID
    #[error("RPC URL {0} returned mismatched chain ID. Expected: {1}, Got: {2}")]
    RpcUrlInvalidChainId(String, String, String),

    /// RPC response could not be parsed
    #[error("failed to parse JSON-RPC response: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

/// Configuration for Batcher
#[allow(clippy::doc_markdown)]
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct BatcherConfig {
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

    /// Batch size
    #[arg(short = 'b', long, env = "BATCH_SIZE", default_value_t = 10)]
    pub batch_size: usize,
}

impl BatcherConfig {
    /// Initializes the configuration by parsing CLI arguments and environment variables.
    pub fn initialize() -> Self {
        Self::parse()
    }

    /// Validates the configuration
    pub async fn validate(&self) -> Result<(), ConfigError> {
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

        // JSON-RPC request payload for eth_chainId
        let health_check_payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_chainId",
            "params": [],
            "id": 1
        });

        let client = Client::builder().build().map_err(ConfigError::HttpClient)?;

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

impl Default for BatcherConfig {
    fn default() -> Self {
        Self { port: 8080, redis_address: None, chain_rpc_urls: HashMap::new(), batch_size: 10 }
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
