//! The metachain is used for rollup block derivation.

use alloy::primitives::Address;
use clap::Parser;
use jsonrpsee::server::{RpcServiceBuilder, Server};
use mchain::mchain::start_mchain;
use rocksdb::DB;
use shared::logger::set_global_default_subscriber;
use tokio::signal::unix::{signal, SignalKind};

/// CLI args for the mchain executable
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
#[allow(missing_docs)]
struct Config {
    // time delay until a block is considered finalized
    #[arg(long, env = "FINALITY_DELAY", default_value_t = 60)]
    finality_delay: u64,
    #[arg(long, env = "DATADIR", default_value = "./datadir")]
    datadir: String,
    #[arg(long, env = "PORT", default_value_t = 8545)]
    port: u64,
    #[arg(long, env = "METRICS_PORT", default_value_t = 8546)]
    metrics_port: u64,
    #[arg(long, env = "CHAIN_ID")]
    chain_id: u64,
    #[arg(long, env = "CHAIN_OWNER", default_value = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")]
    chain_owner: Address,
}

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> eyre::Result<()> {
    // Initialize logging
    set_global_default_subscriber()?;

    let cfg = Config::parse();
    let db = DB::open_default(cfg.datadir)?;
    let module = start_mchain(cfg.chain_id, cfg.chain_owner, cfg.finality_delay, db).await;
    let handle = Server::builder()
        .set_rpc_middleware(RpcServiceBuilder::new().rpc_logger(1024))
        .build(format!("0.0.0.0:{}", cfg.port))
        .await?
        .start(module);

    #[allow(clippy::expect_used)]
    let mut sigint = signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");
    #[allow(clippy::expect_used)]
    let mut sigterm = signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");

    tokio::select! {
        _ = sigint.recv() => {
            println!("Received SIGINT (Ctrl+C), initiating shutdown...");
        }
        _ = sigterm.recv() => {
            println!("Received SIGTERM, initiating shutdown...");
        }
    };

    handle.stop()?;
    handle.stopped().await;
    Ok(())
}
