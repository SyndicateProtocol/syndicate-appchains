//! The `parse` module contains functions for parsing data into types.

use alloy::primitives::Address;
use serde::Deserialize;
use std::{collections::HashMap, hash::Hash, str::FromStr};
use thiserror::Error;
use url::Url;

/// Parse default string into a valid [`URL`].
pub fn parse_url(value: &str) -> Result<Url, Error> {
    Url::parse(value).map_or_else(
        |_err| Err(Error::URL(URLErrorType::InvalidURL(value.to_string()))),
        |url| {
            if !url.has_host() {
                return Err(Error::URL(URLErrorType::InvalidHost));
            }
            match url.scheme() {
                "http" | "https" => Ok(url),
                _ => Err(Error::URL(URLErrorType::InvalidScheme(url.scheme().to_string()))),
            }
        },
    )
}

/// Parse a string into an Ethereum `Address`.
pub fn parse_address(value: &str) -> Result<Address, Error> {
    Address::from_str(value).map_err(|_| Error::EthereumAddress(value.to_string()))
}

/// Parse comma-separated addresses.
pub fn parse_addresses(value: &str) -> Result<Vec<Address>, Error> {
    if value.trim().is_empty() {
        return Ok(Default::default())
    }
    value.split(',').map(|x| parse_address(x.trim())).collect()
}

/// Parse a JSON string into a map
pub fn parse_map<K, V>(s: &str) -> Result<HashMap<K, V>, Error>
where
    K: Hash + Eq + for<'de> Deserialize<'de>,
    V: for<'de> Deserialize<'de>,
{
    serde_json::from_str(s).map_err(|e| Error::InvalidMap(e.to_string()))
}

#[allow(missing_docs)]
#[derive(Debug, Error)]
/// Possible parsing errors
pub enum Error {
    #[error("URL error: {0}")]
    URL(URLErrorType),
    #[error("Invalid address: {0}")]
    EthereumAddress(String),
    #[error("Invalid map: {0}")]
    InvalidMap(String),
}

#[allow(missing_docs)]
/// Possible errors that can occur when parsing a URL
#[derive(Debug, Error)]
pub enum URLErrorType {
    #[error("Invalid URL: {0}")]
    InvalidURL(String),
    #[error("No host")]
    InvalidHost,
    #[error("Invalid scheme: {0}. Only http and https are supported")]
    InvalidScheme(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url_valid() {
        let valid_urls = [
            "http://127.0.0.1:8888",
            "https://localhost:8000",
            "http://example.com:3000",
            "https://test.domain:8545",
        ];

        for url in valid_urls {
            assert!(parse_url(url).is_ok(), "URL should be valid: {}", url);
        }
    }

    #[test]
    fn test_parse_url_invalid_format() {
        let invalid_urls = ["not_a_url", "http://", "://test.com", "http:///", "", "123.456"];

        for url in invalid_urls {
            match parse_url(url) {
                Err(Error::URL(URLErrorType::InvalidURL(error_url))) => {
                    assert_eq!(error_url, url, "Error should contain the invalid URL");
                }
                _ => panic!("Expected InvalidURL error for: {}", url),
            }
        }
    }

    #[test]
    fn test_parse_url_invalid_host_scheme() {
        let invalid_host_schemes = ["file://localhost.com", "data://example.com"];

        for url in invalid_host_schemes {
            match parse_url(url) {
                Err(Error::URL(URLErrorType::InvalidScheme(_))) => {}
                Err(err) => panic!("Expected InvalidScheme error for: {}, got: {:?}", url, err),
                Ok(_) => panic!("Expected InvalidScheme error for: {}", url),
            }
        }
    }

    #[test]
    fn test_parse_url_no_host() {
        let urls_without_host = ["file:///path/to/file", "data:text/plain,Hello", "localhost:999"];

        for url in urls_without_host {
            match parse_url(url) {
                Err(Error::URL(URLErrorType::InvalidHost)) => {}
                _ => panic!("Expected InvalidHost error for: {}", url),
            }
        }
    }

    #[test]
    fn test_parse_url_with_port() {
        let result = parse_url("http://localhost:8545");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().port().unwrap(), 8545);
    }

    #[test]
    fn test_parse_url_without_port() {
        let result = parse_url("https://example.com");
        assert!(result.is_ok());
        // HTTPS default port is 443
        assert_eq!(result.unwrap().port().unwrap_or(443), 443);
    }

    #[test]
    fn test_parse_url_with_path() {
        let result = parse_url("http://localhost:8080/api/v1?param=value");
        assert!(result.is_ok());
        let url = result.unwrap();
        assert_eq!(url.path(), "/api/v1");
        assert_eq!(url.query(), Some("param=value"));
    }

    #[test]
    fn parse_address_valid() {
        // A valid Ethereum address (20 bytes, Hexadecimal format with "0x" prefix)
        let input = "0x742d35cc6634c0532925a3b844bc454e4438f44e";
        let result = parse_address(input);

        // Ensure no error and the address matches
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Address::from_str(input).unwrap());
    }

    #[test]
    fn parse_address_invalid() {
        // An invalid Ethereum address (wrong length)
        let invalid_input = "0x123"; // Too short
        match parse_address(invalid_input) {
            Err(Error::EthereumAddress(_)) => {}
            _ => panic!("Expected EthereumAddress error for: {}", invalid_input),
        }
    }

    #[test]
    fn test_parse_map_empty() {
        // Test with empty JSON object
        let result = parse_map("{}");
        assert!(result.is_ok());
        let map: HashMap<String, String> = result.unwrap();
        assert!(map.is_empty());
    }

    #[test]
    fn test_parse_map_valid() {
        // Test with valid JSON object
        let json = r#"{"1": "https://example1.com", "2": "https://example2.com"}"#;
        let result = parse_map(json);
        assert!(result.is_ok());

        let map: HashMap<String, String> = result.unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("1"), Some(&"https://example1.com".to_string()));
        assert_eq!(map.get("2"), Some(&"https://example2.com".to_string()));
    }

    #[test]
    fn test_parse_map_valid_u64_address() {
        // Test with valid JSON object
        let json = r#"{"1": "0x742d35cc6634c0532925a3b844bc454e4438f44e", "2": "0x742d35cc6634c0532925a3b844bc454e4438f44e"}"#;
        let result: Result<HashMap<u64, Address>, Error> = parse_map(json);
        assert!(result.is_ok());

        let map: HashMap<u64, Address> = result.unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(
            map.get(&1),
            Some(&Address::from_str("0x742d35cc6634c0532925a3b844bc454e4438f44e").unwrap())
        );
        assert_eq!(
            map.get(&2),
            Some(&Address::from_str("0x742d35cc6634c0532925a3b844bc454e4438f44e").unwrap())
        );
    }

    #[test]
    fn test_parse_map_whitespace() {
        // Test with whitespace in JSON
        let json = r#" { "1" : "https://example1.com" , "2" : "https://example2.com" } "#;
        let result = parse_map(json);
        assert!(result.is_ok());

        let map: HashMap<String, String> = result.unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("1"), Some(&"https://example1.com".to_string()));
        assert_eq!(map.get("2"), Some(&"https://example2.com".to_string()));
    }

    #[test]
    fn test_parse_map_special_chars() {
        // Test with URLs containing special characters
        let json =
            r#"{"1": "https://example.com/path?query=value", "2": "http://192.168.1.1:8545"}"#;
        let result = parse_map(json);
        assert!(result.is_ok());

        let map: HashMap<String, String> = result.unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("1"), Some(&"https://example.com/path?query=value".to_string()));
        assert_eq!(map.get("2"), Some(&"http://192.168.1.1:8545".to_string()));
    }
}
