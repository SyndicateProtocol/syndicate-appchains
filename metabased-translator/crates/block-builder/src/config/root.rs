//! Configuration for the block builder service

use clap::Parser;
use std::fmt::Debug;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(short = 'p', long, env = "BLOCK_BUILDER_PORT", default_value_t = 8888)]
    port: u16,

    // Timestamp of genesis block
    #[arg(
        short = 'g',
        long,
        env = "BLOCK_BUILDER_GENESIS_TIMESTAMP",
        default_value_t = 1712500000
    )]
    genesis_timestamp: u64,

    /// Chain ID to use
    #[arg(
        short = 'c',
        long,
        env = "BLOCK_BUILDER_CHAIN_ID",
        default_value_t = 84532
    )]
    chain_id: u64,
}

#[derive(Debug, Clone)]
/// Configuration for the block builder service
pub struct BlockBuilderConfig {
    /// Port number number to be used for the anvil instance
    pub port: u16,
    /// Unix timestamp for the genesis block
    pub genesis_timestamp: u64,
    /// Chain ID for the network
    pub chain_id: u64,
}

impl Default for BlockBuilderConfig {
    fn default() -> Self {
        Self {
            port: 8888,
            genesis_timestamp: 1712500000,
            chain_id: 84532, // Base Sepolia
        }
    }
}

impl BlockBuilderConfig {
    /// Parses command line arguments and environment variables into a `BlockBuilderConfig` struct.
    ///
    /// # Returns
    ///
    /// A new `BlockBuilderConfig` instance populated with values from CLI args and env vars.
    /// CLI args take precedence over env vars, which take precedence over defaults.
    ///
    /// # Example
    ///
    /// ```
    /// use block_builder::config::BlockBuilderConfig;
    ///
    /// let config = BlockBuilderConfig::parse();
    /// ```
    pub fn parse() -> Self {
        let args = Args::parse();
        let config = Self {
            port: args.port,
            genesis_timestamp: args.genesis_timestamp,
            chain_id: args.chain_id,
        };
        tracing::debug!("Got config: {:?}", config);
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial; // avoids timing issues with parallel tests
    use std::env;

    fn create_default_args() -> Args {
        let default = BlockBuilderConfig::default();
        Args {
            port: default.port,
            genesis_timestamp: default.genesis_timestamp,
            chain_id: default.chain_id,
        }
    }

    // Modify BlockBuilderConfig::parse for testing
    #[cfg(test)]
    impl BlockBuilderConfig {
        const fn parse_from_args(args: Args) -> Self {
            Self {
                port: args.port,
                genesis_timestamp: args.genesis_timestamp,
                chain_id: args.chain_id,
            }
        }
    }

    fn clean_env() {
        env::remove_var("BLOCK_BUILDER_PORT");
        env::remove_var("BLOCK_BUILDER_GENESIS_TIMESTAMP");
        env::remove_var("BLOCK_BUILDER_CHAIN_ID");
    }

    #[test]
    #[serial]
    fn test_default_values() {
        clean_env();
        let args = create_default_args();
        let config = BlockBuilderConfig::parse_from_args(args);
        assert_eq!(config.port, 8888);
        assert_eq!(config.genesis_timestamp, 1712500000);
        assert_eq!(config.chain_id, 84532);
    }

    #[test]
    #[serial]
    fn test_env_vars_override_defaults() {
        clean_env();
        env::set_var("BLOCK_BUILDER_PORT", "9999");
        env::set_var("BLOCK_BUILDER_CHAIN_ID", "12345");

        let args = Args::try_parse_from(["my_test_program"]).unwrap();
        assert_eq!(args.port, 9999);
        assert_eq!(args.chain_id, 12345);
    }

    #[test]
    #[serial]
    fn test_cli_args_override_env_vars() {
        clean_env();
        env::set_var("BLOCK_BUILDER_PORT", "9999");

        let args = Args::try_parse_from(["my_test_program", "--port", "7777"]).unwrap();
        assert_eq!(args.port, 7777);
    }

    #[test]
    #[serial]
    fn test_invalid_port_fails() {
        clean_env();
        let result = Args::try_parse_from(["test", "--port", "999999"]);
        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn test_unprefixed_env_var_does_not_override_default() {
        clean_env();
        env::set_var("PORT", "1234");

        let args = Args::try_parse_from(["test"]).unwrap();
        assert_eq!(args.port, 8888);
    }
}
