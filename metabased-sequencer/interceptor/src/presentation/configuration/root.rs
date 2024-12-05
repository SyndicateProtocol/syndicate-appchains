use crate::presentation::configuration::providers::{CliArgs, EnvFile, Logged};
use crate::presentation::configuration::Profile;
use alloy_primitives::{Address, B256};
use clap::Parser;
use figment::providers::{Env, Serialized};
use figment::{Figment, Provider};
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

#[derive(Deserialize)]
pub struct Configuration {
    pub chain_contract_address: Address,
    pub chain_rpc_address: Url,
    pub private_key: B256,
    pub port_sequencer: u16,
}

impl Configuration {
    pub fn parse_with_args(
        env_file: impl Provider + Debug,
        env_profile: impl Provider + Debug,
        env: impl Provider + Debug,
        cli: impl Provider + Debug,
    ) -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Serialized::default("port_sequencer", DEFAULT_PORT))
            .merge(Logged::new(env_file))
            .merge(Logged::new(env_profile))
            .merge(Logged::new(env))
            .merge(Logged::new(cli))
            .extract()
    }

    pub fn parse() -> Result<Self, figment::Error> {
        let args = Args::parse();

        let env_prefix = "METABASED_";
        let env_profile = args
            .profile
            .as_ref()
            .map(Profile::to_env_filename)
            .map(EnvFile::from_filename)
            .unwrap_or_else(EnvFile::new)
            .with_prefix(env_prefix);

        Self::parse_with_args(
            EnvFile::new().with_prefix(env_prefix),
            env_profile,
            Env::prefixed(env_prefix),
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

    const DUMMY_VALUES: [(&str, &str); 3] = [
        ("chain_contract_address", str_repeat!("0", 40)),
        ("chain_rpc_address", "http://test.dummy"),
        ("private_key", str_repeat!("0", 64)),
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
        Serialized::empty(),
        Serialized::empty(),
        DEFAULT_PORT
    ; "Default values are used when not set by any provider")]
    #[test_case(
        Serialized::dummy_with("port_sequencer", 0),
        Serialized::empty(),
        Serialized::empty(),
        Serialized::empty(),
        0
    ; "ENV file overwrites defaults")]
    #[test_case(
        Serialized::dummy_with("port_sequencer", 0),
        Serialized::default("port_sequencer", 1),
        Serialized::empty(),
        Serialized::empty(),
        1
    ; "ENV profile overwrites ENV file")]
    #[test_case(
        Serialized::dummy_with("port_sequencer", 0),
        Serialized::default("port_sequencer", 1),
        Serialized::default("port_sequencer", 2),
        Serialized::empty(),
        2
    ; "ENV vars overwrite ENV files")]
    #[test_case(
        Serialized::dummy_with("port_sequencer", 0),
        Serialized::default("port_sequencer", 1),
        Serialized::default("port_sequencer", 2),
        Serialized::default("port_sequencer", 3),
        3
    ; "CLI args overwrite ENV vars")]
    fn test_configuration_loads_values_from_providers_based_on_expected_priority(
        env_file: impl Provider + Debug,
        env_profile: impl Provider + Debug,
        env: impl Provider + Debug,
        cli: impl Provider + Debug,
        expected_port: u16,
    ) {
        let config = Configuration::parse_with_args(env_file, env_profile, env, cli).unwrap();
        let actual_port = config.port_sequencer;

        assert_eq!(actual_port, expected_port);
    }
}
