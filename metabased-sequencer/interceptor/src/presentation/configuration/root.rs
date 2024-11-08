use crate::presentation::configuration::providers::{CliArgs, EnvFile, Logged};
use crate::presentation::configuration::Profile;
use alloy_primitives::{Address, B256};
use clap::Parser;
use figment::providers::{Env, Serialized};
use figment::Figment;
use serde::Deserialize;
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
    pub port: Option<u16>,

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
    pub port: u16,
}

impl Configuration {
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

        Figment::new()
            .merge(Serialized::default("port", DEFAULT_PORT))
            .merge(Logged::new(EnvFile::new().with_prefix(env_prefix)))
            .merge(Logged::new(env_profile))
            .merge(Logged::new(Env::prefixed(env_prefix)))
            .merge(Logged::new(CliArgs::new(args)))
            .extract()
    }
}
