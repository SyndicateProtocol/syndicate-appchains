//! Utility functions for withdrawals

/// Parse JSON into a type
pub fn parse_json<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, String> {
    serde_json::from_str(s).map_err(|e| format!("Invalid JSON: {}", e))
}
