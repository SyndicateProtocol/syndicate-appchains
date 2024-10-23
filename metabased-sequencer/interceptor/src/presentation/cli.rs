use crate::domain::primitives::Address;
use crate::presentation::server;
use clap::{Parser, ValueEnum};
use figment::providers::{Env, Serialized};
use figment::Figment;
use serde::{Deserialize, Serialize};
use tracing::log;
use tracing_subscriber::EnvFilter;
use url::Url;

const DEFAULT_PORT: u16 = 8456;

pub fn init_tracing_subscriber() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
        .expect("setting default subscriber failed");
}

pub async fn run() -> anyhow::Result<()> {
    let args = parse_args()?;
    let (addr, handle) = server::run(
        args.port,
        args.chain_contract_address,
        args.chain_rpc_address,
    )
    .await?;

    println!("RPC Server started on {addr}");

    // Keep the server running
    handle.stopped().await;

    Ok(())
}

fn parse_args() -> anyhow::Result<Args> {
    let args = ProfileArgs::parse();

    let _ = dotenv::dotenv();

    if let Some(env_file) = args.profile.map(|profile| match profile {
        Profile::Mainnet => "mainnet.env",
        Profile::Testnet => "testnet.env",
        Profile::Devnet => "devnet.env",
        Profile::Localnet => "localnet.env",
    }) {
        let _ = dotenv::from_filename(env_file)
            .inspect_err(|e| log::warn!("Cannot open {env_file}: {e}"));
    }

    let args = Figment::new()
        .merge(Serialized::default("port", DEFAULT_PORT))
        .merge(Env::prefixed("METABASED_"));

    let result = CliArgs::try_parse();

    let args: CliArgs = match result {
        Ok(cli_args) => args.merge(Serialized::defaults(cli_args)).extract()?,
        Err(e) => args.extract().unwrap_or_else(|_| e.exit()),
    };

    Ok(args.into())
}

#[derive(ValueEnum, Debug, Clone, Serialize, Deserialize)]
enum Profile {
    Mainnet,
    Testnet,
    Devnet,
    Localnet,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(version, about, long_about = None)]
struct ProfileArgs {
    /// Profile that chooses which .env file to load
    #[arg(short = 'o', long)]
    profile: Option<Profile>,
}

/// The Metabased sequencer is a server that exposes a JSON-RPC API that accepts incoming
/// transactions to be run on a layer-3 blockchain.
#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// Address of the layer-2 Ethereum smart contract that processes the layer-3 transactions
    #[arg(short = 'c', long)]
    chain_contract_address: Address,

    /// URL address of the layer-2 Ethereum RPC node to connect to over HTTP
    #[arg(short = 'r', long)]
    chain_rpc_address: Url,

    /// Port to listen on
    #[arg(short = 'p', long)]
    port: Option<u16>,

    /// Profile that chooses which .env file to load
    #[arg(short = 'o', long)]
    profile: Option<Profile>,
}

struct Args {
    chain_contract_address: Address,
    chain_rpc_address: Url,
    port: u16,
}

impl From<CliArgs> for Args {
    fn from(value: CliArgs) -> Self {
        Self {
            chain_contract_address: value.chain_contract_address,
            chain_rpc_address: value.chain_rpc_address,
            port: value.port.expect("port should be set by a default value"),
        }
    }
}
