//! The `MockChain` is used for appchain block derivation.

use clap::Parser;
#[cfg(feature = "rocksdb")]
use {
    jsonrpsee::server::middleware::http::ProxyGetRequestLayer,
    jsonrpsee::server::{RandomStringIdProvider, Server},
    rocksdb::DB,
    shared::service_start_utils::start_http_server_with_metrics_only,
    shared::{service_start_utils::MetricsState, tracing::setup_global_logging},
    synd_mchain::{metrics::MchainMetrics, server::start_mchain},
    tokio::signal::unix::{signal, SignalKind},
    tracing::info,
};

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

#[tokio::main]
#[cfg(feature = "rocksdb")]
async fn main() -> eyre::Result<()> {
    use jsonrpsee::{
        server::{PingConfig, ServerConfigBuilder},
        ws_client::RpcServiceBuilder,
    };
    // Initialize logging
    setup_global_logging()?;

    let cfg = Config::parse();
    info!("loading rockdb db {}", cfg.datadir);
    let db = DB::open_default(cfg.datadir)?;

    let mut metrics_state = MetricsState::default();
    let metrics = MchainMetrics::new(&mut metrics_state.registry);

    info!("starting synd-mchain server on port {}", cfg.port);
    tokio::spawn(start_http_server_with_metrics_only(metrics_state, cfg.metrics_port));
    let module = start_mchain(cfg.appchain_chain_id, cfg.finality_delay, db, metrics);
    let jsonrpsee_cfg = ServerConfigBuilder::new()
        .enable_ws_ping(PingConfig::default())
        .set_id_provider(RandomStringIdProvider::new(64))
        .build();

    let http_middleware = tower::builder::ServiceBuilder::new()
        .layer(ProxyGetRequestLayer::new([("/health", "health"), ("/ready", "ready")])?);
    let handle = Server::builder()
        .set_config(jsonrpsee_cfg)
        .set_http_middleware(http_middleware)
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
    }

    _ = handle.stop();
    handle.stopped().await;
    Ok(())
}
#[tokio::main]
#[cfg(not(feature = "rocksdb"))]
async fn main() -> eyre::Result<()> {
    Ok(())
}
