//! The `config` module handles configuration parsing for the metabased sequencer.

use alloy::primitives::{Address, B256};
use clap::Parser;
use eyre::Result;
use url::Url;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Address of the layer-2 Ethereum smart contract
    #[arg(short = 'c', long)]
    pub chain_contract_address: Option<Address>,

    /// URL of the layer-2 Ethereum RPC node
    #[arg(short = 'r', long)]
    pub chain_rpc_url: Option<Url>,

    /// Port to listen on
    #[arg(short = 'p', long)]
    pub port: Option<u16>,

    /// Private key for signing transactions    
    #[arg(short = 'k', long)]
    pub private_key: Option<B256>,
}

/// The configuration for the metabased sequencer.
#[derive(Debug)]
pub struct Config {
    /// The address of sequencing contract on the sequencing chain
    pub chain_contract_address: Address,

    /// The URL of the sequencing chain RPC node
    pub chain_rpc_url: Url,

    /// The private key for signing transactions
    pub private_key: B256,

    /// The port to listen on
    pub port: u16,
}

impl Config {
    /// Parse the command line arguments and return a `Config` struct.
    pub fn parse() -> Result<Self> {
        let args = Args::parse();

        Ok(Self {
            chain_contract_address: args
                .chain_contract_address
                .ok_or_else(|| eyre::eyre!("Missing chain contract address"))?,
            chain_rpc_url: args
                .chain_rpc_url
                .ok_or_else(|| eyre::eyre!("Missing chain RPC URL"))?,
            private_key: args.private_key.ok_or_else(|| eyre::eyre!("Missing private key"))?,
            port: args.port.unwrap_or(8456),
        })
    }
}
