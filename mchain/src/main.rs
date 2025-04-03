//! The metachain is used for rollup block derivation.

use alloy::primitives::Address;
use block_builder::config::get_default_private_key_signer;
use clap::Parser;
use jsonrpsee::server::{RpcServiceBuilder, Server};
use mchain::mchain::start_mchain;
use rocksdb::DB;
use std::sync::Arc;
use tokio::{
    signal::unix::{signal, SignalKind},
    sync::oneshot,
};

/// CLI args for the mchain executable
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
#[allow(missing_docs)]
struct Config {
    #[arg(long, env = "DATADIR", default_value = "./datadir")]
    datadir: String,
    #[arg(long, env = "PORT", default_value_t = 8545)]
    port: u64,
    #[arg(long, env = "METRICS_PORT", default_value_t = 8546)]
    metrics_port: u64,
    #[arg(long, env = "CHAIN_ID")]
    chain_id: u64,
    #[arg(long, env = "CHAIN_OWNER", default_value_t = get_default_private_key_signer().address())]
    chain_owner: Address,
}

#[allow(clippy::redundant_pub_crate)]
fn init_signal_handler(runtime: &tokio::runtime::Runtime, shutdown_tx: oneshot::Sender<()>) {
    runtime.spawn(async move {
        #[allow(clippy::expect_used)]
        let mut sigint =
            signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");
        #[allow(clippy::expect_used)]
        let mut sigterm =
            signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");

        tokio::select! {
            _ = sigint.recv() => {
                println!("Received SIGINT (Ctrl+C), initiating shutdown...");
            }
            _ = sigterm.recv() => {
                println!("Received SIGTERM, initiating shutdown...");
            }
        };

        assert!(shutdown_tx.send(()).is_ok(), "Failed to send shutdown signal")
    });
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let cfg = Config::parse();
    let db = Arc::new(DB::open_default(cfg.datadir)?);
    let module = start_mchain(cfg.chain_id, cfg.chain_owner, db).await?;
    let _handle = Server::builder()
        .set_rpc_middleware(RpcServiceBuilder::new().rpc_logger(1024))
        .build(format!("0.0.0.0:{}", cfg.port))
        .await?
        .start(module);
    let runtime = tokio::runtime::Runtime::new()?;
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    init_signal_handler(&runtime, shutdown_tx);
    Ok(shutdown_rx.await?)
}
