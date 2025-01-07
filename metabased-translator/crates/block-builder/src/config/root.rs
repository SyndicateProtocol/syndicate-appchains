use crate::config::providers::{CliArgs, Logged};
use clap::Parser;
use figment::providers::{Env, Serialized};
use figment::value::{Dict, Value};
use figment::{Figment, Profile, Provider};
use serde::Deserialize;
use std::fmt::Debug;

// Prefix for environment variables for the block builder
pub const ENV_PREFIX: &str = "BLOCK_BUILDER_";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Port to listen on
    #[arg(short = 'p', long)]
    pub port: Option<u16>,

    // Timestamp of genesis block
    #[arg(short = 'g', long)]
    pub genesis_timestamp: Option<u64>,

    /// Chain ID to use
    #[arg(short = 'c', long)]
    pub chain_id: Option<u64>,

    /// Profile that owns a set of environment variables
    #[arg(short = 'o', long)]
    pub profile: Option<Profile>,
}

#[derive(Deserialize)]
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
    // Parse configuration from environment variables and CLI arguments, with defaults at lowest priority
    pub fn parse_with_args(
        env: impl Provider + Debug,
        cli: impl Provider + Debug,
    ) -> Result<Self, figment::Error> {
        let defaults = Configuration::default();
        let default_dict = Dict::from_iter([
            ("port".to_owned(), Value::from(defaults.port)),
            (
                "genesis_timestamp".to_owned(),
                Value::from(defaults.genesis_timestamp),
            ),
            ("chain_id".to_owned(), Value::from(defaults.chain_id)),
        ]);
        let serialized_defaults = Serialized::defaults(default_dict);

        Figment::new()
            .merge(serialized_defaults) // lowest priority
            .merge(Logged::new(env))
            .merge(Logged::new(cli))
            .extract()
    }

    pub fn parse() -> Result<Self, figment::Error> {
        let args = Args::parse();
        Self::parse_with_args(
            Env::prefixed(ENV_PREFIX), // maps to one without the prefix
            CliArgs::new(args),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use figment::value::{Dict, Value};
    use test_case::test_case;

    pub const TEST_DEFAULT_PORT: u16 = 8888;

    const DUMMY_VALUES: [(&str, u64); 3] = [
        ("port", 8888),
        ("genesis_timestamp", 1712500000),
        ("chain_id", 84532),
    ];

    trait Dummy {
        fn dummy() -> Self;
        fn dummy_with(key: &'_ str, value: impl Into<Value>) -> Self;
    }

    impl Dummy for Serialized<Dict> {
        fn dummy() -> Self {
            Serialized::defaults(Dict::from(
                DUMMY_VALUES.map(|(k, v)| (k.to_owned(), v.into())),
            ))
        }

        fn dummy_with(key: &'_ str, value: impl Into<Value>) -> Self {
            Serialized::defaults(Dict::from_iter(
                DUMMY_VALUES
                    .into_iter()
                    .map(|(k, v)| (k.to_owned(), v.into()))
                    .chain([(key.to_owned(), value.into())]),
            ))
        }
    }

    trait Empty {
        fn empty() -> Self;
    }

    impl Empty for Serialized<Dict> {
        fn empty() -> Self {
            Serialized::defaults(Dict::new())
        }
    }

    #[test_case(
        Serialized::dummy(),
        Serialized::empty(),
        TEST_DEFAULT_PORT
    ; "Default values are used when not set by any provider")]
    #[test_case(
        Serialized::dummy_with("port", 0),
        Serialized::empty(),
        0
    ; "ENV vars overwrite defaults")]
    #[test_case(
        Serialized::dummy_with("port", 2),
        Serialized::dummy_with("port", 3),
        3
    ; "CLI args overwrite ENV vars")]
    fn test_configuration_loads_values_from_providers_based_on_expected_priority(
        env: impl Provider + Debug,
        cli: impl Provider + Debug,
        expected_port: u16,
    ) {
        let config = Configuration::parse_with_args(env, cli).unwrap();
        let actual_port = config.port;

        assert_eq!(actual_port, expected_port);
    }

    // Helper function to create an environment with a properly prefixed variable
    fn env_with_prefixed_port(port: u16) -> Env {
        let mut env = std::collections::HashMap::new();
        env.insert("BLOCK_BUILDER_PORT".to_string(), port.to_string());
        Env::prefixed(ENV_PREFIX)
    }

    #[test]
    fn test_cli_overrides_prefixed_env_var() {
        // Create environment with prefixed port variable
        let env = env_with_prefixed_port(9999);

        // Create CLI args with port override
        let cli = Serialized::dummy_with("port", 1234);

        let config = Configuration::parse_with_args(env, cli).unwrap();

        assert_eq!(config.port, 1234);
        assert_ne!(config.port, 9999);
        assert_ne!(config.port, TEST_DEFAULT_PORT);
    }

    #[test]
    fn test_unprefixed_env_var_does_not_override_default() {
        // Create environment with unprefixed port variable that should be ignored
        let mut env = std::collections::HashMap::new();
        env.insert("PORT".to_string(), "1234".to_string());
        let env = Env::prefixed(ENV_PREFIX);

        let cli = Serialized::dummy();

        let config = Configuration::parse_with_args(env, cli).unwrap();

        assert_eq!(config.port, TEST_DEFAULT_PORT);
        assert_ne!(config.port, 1234);
    }
}
