//! The `MockChain` is used for appchain block derivation.

use alloy::primitives::U256;
use clap::Parser;
use jsonrpsee::server::{RandomStringIdProvider, RpcServiceBuilder, Server};
use shared::{
    append_only_db::AppendOnlyDB,
    fixed_size_append_only_db::FixedSizeAppendOnlyDB,
    logger::set_global_default_subscriber,
    service_start_utils::{start_metrics_and_health, MetricsState},
    single_value_db::SingleValueDB,
};
use synd_mchain::{db::ArbitrumDB, metrics::MchainMetrics, server::start_mchain};
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

/// CLI args for the `synd-mchain` executable
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
    metrics_port: u16,
    #[arg(long, env = "APPCHAIN_CHAIN_ID")]
    appchain_chain_id: u64,
}
/// VERSION must be bumped whenever a breaking change is made
const VERSION: u64 = 2;

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> eyre::Result<()> {
    // Initialize logging
    #[allow(clippy::unwrap_used)]
    set_global_default_subscriber().unwrap();

    let cfg = Config::parse();
    info!("loading db {}", cfg.datadir);
    let db = ArbitrumDB {
        block: AppendOnlyDB::open(
            &(cfg.datadir.clone() + "/block.db"),
            VERSION.to_be_bytes().as_slice(),
        )?,
        message_acc: FixedSizeAppendOnlyDB::open(
            &(cfg.datadir.clone() + "/message.db"),
            &U256::from(VERSION).to_be_bytes(),
        )?,
        state: SingleValueDB::open(&(cfg.datadir + "/state.db"))?,
    };
    let mut metrics_state = MetricsState::default();
    let metrics = MchainMetrics::new(&mut metrics_state.registry);

    info!("starting synd-mchain server on port {}", cfg.port);
    let module = start_mchain(cfg.appchain_chain_id, cfg.finality_delay, db, metrics);

    let handle = Server::builder()
        .ws_only()
        .set_id_provider(RandomStringIdProvider::new(64))
        .set_rpc_middleware(RpcServiceBuilder::new().rpc_logger(1024))
        .build(format!("0.0.0.0:{}", cfg.port))
        .await?
        .start(module);
    tokio::spawn(start_metrics_and_health(metrics_state, cfg.metrics_port));

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

    _ = handle.stop();
    handle.stopped().await;
    Ok(())
}
