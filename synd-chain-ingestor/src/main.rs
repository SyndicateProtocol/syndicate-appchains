//! The `synd-chain-ingestor` serves blocks from a chain to all appchains that use the chain.

use clap::Parser;
use jsonrpsee::server::{PingConfig, Server, ServerConfigBuilder};
use shared::{
    service_start_utils::{start_metrics_and_health, MetricsState},
    tracing::setup_global_logging,
};
use std::time::Duration;
use synd_chain_ingestor::{config::Config, ingestor, metrics::ChainIngestorMetrics, server};
use tokio::{
    signal::unix::{signal, SignalKind},
    time::sleep,
};
use tracing::{error, info};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Initialize logging
    setup_global_logging()?;

    let cfg = Config::parse();
    let mut provider = cfg.new_provider().await;
    let mut metrics_state = MetricsState::default();
    let metrics = ChainIngestorMetrics::new(&mut metrics_state.registry);
    tokio::spawn(start_metrics_and_health(metrics_state, cfg.metrics_port, None));

    let (module, ctx) = server::start(
        &provider,
        &cfg.ws_url,
        &cfg.db_file,
        cfg.start_block,
        cfg.parallel_sync_requests,
        &metrics,
    )
    .await?;

    info!("starting synd-chain-ingestor server on {}", cfg.port);
    let jsonrpsee_cfg =
        ServerConfigBuilder::new().ws_only().enable_ws_ping(PingConfig::default()).build();
    let _handle = Server::builder()
        .set_config(jsonrpsee_cfg)
        .build(format!("0.0.0.0:{}", cfg.port))
        .await?
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
        }
        std::process::exit(0);
    });

    loop {
        if let Err(err) = ingestor::run(&ctx, &provider, &metrics).await {
            error!("ingestor failed: {err}");
            sleep(Duration::from_secs(1)).await;
            // manually recreate the ws connection just in case.
            provider = cfg.new_provider().await;
        }
    }
}
