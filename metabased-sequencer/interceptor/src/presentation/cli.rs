use crate::domain::primitives::Address;
use crate::presentation::server;
use clap::Parser;
use tracing_subscriber::EnvFilter;
use url::Url;

pub fn init_tracing_subscriber() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
        .expect("setting default subscriber failed");
}

pub async fn run() -> anyhow::Result<()> {
    let args = Args::parse();
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

/// The Metabased sequencer is a server that exposes a JSON-RPC API that accepts incoming
/// transactions to be run on a layer-3 blockchain.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Address of the layer-2 Ethereum smart contract that processes the layer-3 transactions
    #[arg(short = 'c', long)]
    chain_contract_address: Address,

    /// URL address of the layer-2 Ethereum RPC node to connect to over HTTP
    #[arg(short = 'r', long)]
    chain_rpc_address: Url,

    /// Port to listen on
    #[arg(short = 'p', long, default_value_t = 8456)]
    port: u16,
}
