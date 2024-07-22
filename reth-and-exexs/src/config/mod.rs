use serde::{Deserialize, Serialize};
use std::env;

/// The global configuration for the program.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// The base URL for the Engine API
    pub base_url: String,
    /// The secret key for JWT authentication
    pub secret_key: String,
    /// The sequencer contract address
    pub sequencer_address: String,
    /// The L2 chain RPC URL
    pub l2_rpc_url: String,
}

impl Default for Config {
    fn default() -> Self {
        let base_url = env::var("ENGINE_API_URL").unwrap_or_else(|_| {
            panic!(
                "ENGINE_API_URL environment variable not set. \
                Please set this to the base url of the engine api"
            )
        });

        let secret_key = env::var("JWT_SECRET").unwrap_or_else(|_| {
            panic!(
                "JWT_SECRET environment variable not set. \
                Please set this to the 256 bit hex-encoded secret key used to authenticate with the engine api. \
                This should be the same as set in the `--auth.secret` flag when executing go-ethereum."
            )
        });

        let sequencer_address = env::var("CONTRACT_ADDRESS").unwrap_or_else(|_| {
            panic!(
                "CONTRACT_ADDRESS environment variable not set. \
                Please set this to the address of the sequencer contract."
            )
        });

        let l2_rpc_url = env::var("L2_RPC_URL")
            .unwrap_or_else(|_| panic!("L2_RPC_URL environment variable not set"));

        Self {
            base_url,
            secret_key,
            sequencer_address,
            l2_rpc_url,
        }
    }
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_env() {
        env::set_var("ENGINE_API_URL", "http://localhost:8545");
        env::set_var(
            "JWT_SECRET",
            "3f7b9d2e8c1a4f6e0d5h2i7k9l3m6n8p1q4r5s2t7u0v3w6x9y2z5a8b1c4d7e",
        );
        env::set_var(
            "CONTRACT_ADDRESS",
            "0x1234567890123456789012345678901234567890",
        );
        env::set_var("L2_RPC_URL", "http://localhost:8546");
    }

    fn clear_env() {
        env::remove_var("ENGINE_API_URL");
        env::remove_var("JWT_SECRET");
        env::remove_var("CONTRACT_ADDRESS");
        env::remove_var("L2_RPC_URL");
    }

    #[test]
    fn test_config_from_env() {
        setup_env();
        let config = Config::from_env();
        assert_eq!(config.base_url, "http://localhost:8545");
        assert_eq!(
            config.secret_key,
            "3f7b9d2e8c1a4f6e0d5h2i7k9l3m6n8p1q4r5s2t7u0v3w6x9y2z5a8b1c4d7e"
        );
        assert_eq!(
            config.sequencer_address,
            "0x1234567890123456789012345678901234567890"
        );
        assert_eq!(config.l2_rpc_url, "http://localhost:8546");
        clear_env();
    }

    #[test]
    #[should_panic(expected = "ENGINE_API_URL environment variable not set")]
    fn test_missing_engine_api_url() {
        clear_env();
        Config::from_env();
    }

    #[test]
    #[should_panic(expected = "JWT_SECRET environment variable not set")]
    fn test_missing_jwt_secret() {
        clear_env();
        env::set_var("ENGINE_API_URL", "http://localhost:8545");
        env::set_var(
            "CONTRACT_ADDRESS",
            "0x1234567890123456789012345678901234567890",
        );
        env::set_var("L2_RPC_URL", "http://localhost:8546");
        Config::from_env();
    }

    #[test]
    #[should_panic(expected = "CONTRACT_ADDRESS environment variable not set")]
    fn test_missing_contract_address() {
        clear_env();
        env::set_var("ENGINE_API_URL", "http://localhost:8545");
        env::set_var(
            "JWT_SECRET",
            "3f7b9d2e8c1a4f6e0d5h2i7k9l3m6n8p1q4r5s2t7u0v3w6x9y2z5a8b1c4d7e",
        );
        Config::from_env();
    }

    #[test]
    #[should_panic(expected = "L2_RPC_URL environment variable not set")]
    fn test_missing_l2_rpc_url() {
        clear_env();
        env::set_var("ENGINE_API_URL", "http://localhost:8545");
        env::set_var(
            "JWT_SECRET",
            "3f7b9d2e8c1a4f6e0d5h2i7k9l3m6n8p1q4r5s2t7u0v3w6x9y2z5a8b1c4d7e",
        );
        env::set_var(
            "CONTRACT_ADDRESS",
            "0x1234567890123456789012345678901234567890",
        );
        Config::from_env();
    }
}
