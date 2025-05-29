//! The synd-chain-ingestor serves blocks from a chain to all appchains that use the chain.

use clap::{command, Parser};
use humantime::parse_duration;
use jsonrpsee::server::{PingConfig, Server};
use shared::{
    service_start_utils::{start_metrics_and_health, MetricsState},
    tracing::setup_global_logging,
};
use std::time::Duration;
use synd_chain_ingestor::{eth_client::EthClient, ingestor, metrics::ChainIngestorMetrics, server};
use tokio::{
    signal::unix::{signal, SignalKind},
    time::sleep,
};
use tracing::{error, info};

/// CLI args for the chain ingestor executable
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
#[allow(missing_docs)]
struct Config {
    // time delay until a block is considered finalized
    #[arg(long, env = "RPC_URL")]
    rpc_url: String,
    #[arg(long, env = "DB_FILE")]
    db_file: String,
    #[arg(long, env = "START_BLOCK")]
    start_block: u64,
    #[arg(long, env = "CHANNEL_SIZE", default_value_t = 1024)]
    channel_size: usize,
    #[arg(long, env = "PARALLEL_SYNC_REQUESTS", default_value_t = 190)]
    parallel_sync_requests: u64,
    #[arg(long, env = "PORT", default_value_t = 8545)]
    port: u16,
    #[arg(long, env = "METRICS_PORT", default_value_t = 8546)]
    metrics_port: u16,
    #[arg(
        long,
        env = "REQUEST_TIMEOUT",
        default_value = "10s",
        value_parser = parse_duration
    )]
    request_timeout: Duration,
}

async fn new_provider(cfg: &Config) -> EthClient {
    EthClient::new(&cfg.rpc_url, cfg.request_timeout, Duration::from_secs(300), cfg.channel_size)
        .await
}

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
#[allow(clippy::unwrap_used)]
async fn main() {
    // Initialize logging
    set_global_default_subscriber().unwrap();

    let cfg = Config::parse();
    let mut provider = new_provider(&cfg).await;
    let mut metrics_state = MetricsState::default();
    let metrics = ChainIngestorMetrics::new(&mut metrics_state.registry);
    let (module, ctx) = server::start(
        &provider,
        &cfg.rpc_url,
        &cfg.db_file,
        cfg.start_block,
        cfg.parallel_sync_requests,
        &metrics,
    )
    .await
    .unwrap();

    info!("starting synd-chain-ingestor server on {}", cfg.port);
    let _handle = Server::builder()
        .ws_only()
        .enable_ws_ping(PingConfig::default())
        .build(format!("0.0.0.0:{}", cfg.port))
        .await
        .unwrap()
        .start(module);

    #[allow(clippy::expect_used)]
    let mut sigint = signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");
    #[allow(clippy::expect_used)]
    let mut sigterm = signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");
    tokio::spawn(async move {
        tokio::select! {
            _ = sigint.recv() => {
                info!("Received SIGINT (Ctrl+C), initiating shutdown...");
            }
            _ = sigterm.recv() => {
                info!("Received SIGTERM, initiating shutdown...");
            }
        };
        std::process::exit(0);
    });

    tokio::spawn(start_metrics_and_health(metrics_state, cfg.metrics_port));
    loop {
        if let Err(err) = ingestor::run(&ctx, &provider, &metrics).await {
            error!("ingestor failed: {}", err);
            sleep(Duration::from_secs(1)).await;
            // manually recreate the ws connection just in case.
            provider = new_provider(&cfg).await;
        }
    }
}
