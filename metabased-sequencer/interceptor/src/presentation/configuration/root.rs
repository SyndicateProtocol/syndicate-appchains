use crate::presentation::configuration::providers::{CliArgs, Logged};
use alloy_primitives::{Address, B256};
use clap::Parser;
use figment::providers::{Env, Serialized};
use figment::{Figment, Profile, Provider};
use serde::Deserialize;
use std::fmt::Debug;
use url::Url;

/// The Metabased sequencer is an HTTP server exposing a JSON-RPC API that accepts incoming
/// transactions to be run on a layer-3 blockchain.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Address of the layer-2 Ethereum smart contract that processes the layer-3 transactions
    #[arg(short = 'c', long)]
    pub chain_contract_address: Option<Address>,

    /// URL address of the layer-2 Ethereum RPC node to connect to over HTTP
    #[arg(short = 'r', long)]
    pub chain_rpc_address: Option<Url>,

    /// Port to listen on
    #[arg(short = 'p', long)]
    pub port_sequencer: Option<u16>,

    /// Private key for signing layer-2 transactions
    #[arg(short = 'k', long)]
    pub private_key: Option<B256>,

    /// Profile that chooses which .env file to load
    #[arg(short = 'o', long)]
    pub profile: Option<Profile>,
}

pub const DEFAULT_PORT: u16 = 8456;
// Prefix for environment variables for the sequencer
pub const ENV_PREFIX: &str = "METABASED_SEQUENCER_";

#[derive(Deserialize)]
pub struct Configuration {
    pub chain_contract_address: Address,
    pub chain_rpc_address: Url,
    pub private_key: B256,
    pub port_sequencer: u16,
}

impl Configuration {
    pub fn parse_with_args(
        // env_file: impl Provider + Debug,
        // env_profile: impl Provider + Debug,
        env: impl Provider + Debug,
        cli: impl Provider + Debug,
    ) -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Serialized::default("port_sequencer", DEFAULT_PORT))
            // .merge(Logged::new(env_file))
            // .merge(Logged::new(env_profile))
            .merge(Logged::new(env))
            .merge(Logged::new(cli))
            .extract()
    }

    pub fn parse() -> Result<Self, figment::Error> {
        let args = Args::parse();
        
        // let env_profile = args
        //     .profile
        //     .as_ref()
        //     .map(Profile::to_env_filename)
        //     .map(EnvFile::from_filename)
        //     .unwrap_or_else(EnvFile::new)
        //     .with_prefix(ENV_PREFIX);

        Self::parse_with_args(
            // EnvFile::new().with_prefix(ENV_PREFIX),
            // env_profile,
            Env::prefixed(ENV_PREFIX), // maps to one without the prefix 
            CliArgs::new(args),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use const_format::str_repeat;
    use figment::value::{Dict, Value};
    use test_case::test_case;

    const DUMMY_VALUES: [(&str, &str); 4] = [
        ("chain_contract_address", str_repeat!("0", 40)),
        ("chain_rpc_address", "http://test.dummy"),
        ("private_key", str_repeat!("0", 64)),
        ("sequencer_port", "101"),
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

    impl Empty for Serialized<()> {
        fn empty() -> Self {
            Serialized::defaults(())
        }
    }

    #[test_case(
        Serialized::dummy(),
        Serialized::empty(),
        DEFAULT_PORT
    ; "Default values are used when not set by any provider")]
    #[test_case(
        Serialized::dummy_with("port_sequencer", 0),
        Serialized::empty(),
        0
    ; "ENV vars overwrite defaults")]
    #[test_case(
        Serialized::dummy_with("port_sequencer", 2),
        Serialized::dummy_with("port_sequencer", 3),
        3
    ; "CLI args overwrite ENV vars")]
    fn test_configuration_loads_values_from_providers_based_on_expected_priority(
        env: impl Provider + Debug,
        cli: impl Provider + Debug,
        expected_port: u16,
    ) {
        let config = Configuration::parse_with_args(env, cli).unwrap();
        let actual_port = config.port_sequencer;

        assert_eq!(actual_port, expected_port);
    }

    // Helper function to create an environment with a properly prefixed variable
    fn env_with_prefixed_port(port: u16) -> Env {
        let mut env = std::collections::HashMap::new();
        env.insert(
            "METABASED_SEQUENCER_PORT_SEQUENCER".to_string(),
            port.to_string()
        );
        Env::prefixed(ENV_PREFIX)
    }


    #[test]
    fn test_cli_overrides_prefixed_env_var() {
        // Create environment with prefixed port variable
        let env = env_with_prefixed_port(9999);

        // Create CLI args with port override
        let cli = Serialized::dummy_with("port_sequencer", 8888);

        let config = Configuration::parse_with_args(env, cli).unwrap();

        assert_eq!(config.port_sequencer, 8888);
        assert_ne!(config.port_sequencer, 9999);
        assert_ne!(config.port_sequencer, DEFAULT_PORT);
    }

    #[test]
    fn test_unprefixed_env_var_does_not_override_default() {
        // Create environment with unprefixed port variable that should be ignored
        let mut env = std::collections::HashMap::new();
        env.insert("PORT_SEQUENCER".to_string(), "9999".to_string());
        let env = Env::prefixed(ENV_PREFIX);

        let cli = Serialized::dummy();

        let config = Configuration::parse_with_args(env, cli).unwrap();

        assert_eq!(config.port_sequencer, DEFAULT_PORT);
        assert_ne!(config.port_sequencer, 9999);
    }
}
