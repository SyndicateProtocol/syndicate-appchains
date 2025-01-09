use clap::Parser;
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Port to listen on
    #[arg(short = 'p', long, env = "BLOCK_BUILDER_PORT", default_value_t = 8888)]
    pub port: u16,

    // Timestamp of genesis block
    #[arg(
        short = 'g',
        long,
        env = "BLOCK_BUILDER_GENESIS_TIMESTAMP",
        default_value_t = 1712500000
    )]
    pub genesis_timestamp: u64,

    /// Chain ID to use
    #[arg(
        short = 'c',
        long,
        env = "BLOCK_BUILDER_CHAIN_ID",
        default_value_t = 84532
    )]
    pub chain_id: u64,
}

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub port: u16,
    pub genesis_timestamp: u64,
    pub chain_id: u64,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            port: 8888,
            genesis_timestamp: 1712500000,
            chain_id: 84532, // Base Sepolia
        }
    }
}

impl Configuration {
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
    use serial_test::serial;
    use std::env;

    fn create_default_args() -> Args {
        let default = Configuration::default();
        Args {
            port: default.port,
            genesis_timestamp: default.genesis_timestamp,
            chain_id: default.chain_id,
        }
    }

    // Modify Configuration::parse for testing
    #[cfg(test)]
    impl Configuration {
        fn parse_from_args(args: Args) -> Self {
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
        let config = Configuration::parse_from_args(args);
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

        clean_env();
    }

    #[test]
    #[serial]
    fn test_cli_args_override_env_vars() {
        clean_env();
        env::set_var("BLOCK_BUILDER_PORT", "9999");

        let args = Args::try_parse_from(["my_test_program", "--port", "7777"]).unwrap();
        assert_eq!(args.port, 7777);

        clean_env();
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

        clean_env();
    }
}
