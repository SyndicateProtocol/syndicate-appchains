//! This module contains `config` for the `maestro` service

use crate::errors::ConfigError;
use alloy::transports::http::Client;
use clap::Parser;
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
        value_parser = parse_chain_rpc_urls_map
    )]
    pub chain_rpc_urls: HashMap<String, String>,

    /// Timeout in seconds for connection validation
    #[arg(long, env = "VALIDATION_TIMEOUT", default_value = "5s",
    value_parser = humantime::parse_duration)]
    pub validation_timeout: Duration,

    /// Skip validation of Nitro URLs
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
            "method": "eth_getBlockByNumber",
            "params": ["latest", false],
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
                .map_err(|e| ConfigError::NitroUrlConnection(chain_id.clone(), url.clone(), e))?;

            // Check for successful status code (2xx)
            if !response.status().is_success() {
                return Err(ConfigError::NitroUrlInvalidStatus(
                    chain_id.clone(),
                    url.clone(),
                    response.status().to_string(),
                ));
            }
            debug!(%chain_id, %url, "Successful JSON-RPC request");
        }
        Ok(())
    }
}

/// Parse the chain ID to URL mappings from the JSON string
fn parse_chain_rpc_urls_map(s: &str) -> Result<HashMap<String, String>, ConfigError> {
    let map: HashMap<String, String> =
        serde_json::from_str(s).map_err(|e| ConfigError::ChainIdNitroUrlParse(e.to_string()))?;
    Ok(map)
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
    fn test_parse_chain_id_nitro_urls_map_empty() {
        // Test with empty JSON object
        let result = parse_chain_rpc_urls_map("{}");
        assert!(result.is_ok());
        let map = result.unwrap();
        assert!(map.is_empty());
    }

    #[test]
    fn test_parse_chain_id_nitro_urls_map_valid() {
        // Test with valid JSON object
        let json = r#"{"1": "https://example1.com", "2": "https://example2.com"}"#;
        let result = parse_chain_rpc_urls_map(json);
        assert!(result.is_ok());

        let map = result.unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("1"), Some(&"https://example1.com".to_string()));
        assert_eq!(map.get("2"), Some(&"https://example2.com".to_string()));
    }

    #[test]
    fn test_parse_chain_id_nitro_urls_map_whitespace() {
        // Test with whitespace in JSON
        let json = r#" { "1" : "https://example1.com" , "2" : "https://example2.com" } "#;
        let result = parse_chain_rpc_urls_map(json);
        assert!(result.is_ok());

        let map = result.unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("1"), Some(&"https://example1.com".to_string()));
        assert_eq!(map.get("2"), Some(&"https://example2.com".to_string()));
    }

    #[test]
    fn test_parse_chain_id_nitro_urls_map_special_chars() {
        // Test with URLs containing special characters
        let json =
            r#"{"1": "https://example.com/path?query=value", "2": "http://192.168.1.1:8545"}"#;
        let result = parse_chain_rpc_urls_map(json);
        assert!(result.is_ok());

        let map = result.unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("1"), Some(&"https://example.com/path?query=value".to_string()));
        assert_eq!(map.get("2"), Some(&"http://192.168.1.1:8545".to_string()));
    }

    #[test]
    fn test_parse_chain_id_nitro_urls_map_malformed_urls() {
        // Test with valid JSON but malformed URLs (should still parse)
        let json = r#"{"1": "not a url", "2": "also-not-url"}"#;
        let result = parse_chain_rpc_urls_map(json);
        assert!(result.is_ok());

        let map = result.unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("1"), Some(&"not a url".to_string()));
        assert_eq!(map.get("2"), Some(&"also-not-url".to_string()));
    }
}
