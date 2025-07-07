//! The `synd-chain-ingestor` serves blocks from a chain to all appchains that use the chain.

use clap::Parser;
use jsonrpsee::server::{PingConfig, Server, ServerConfigBuilder};
use shared::{
    service_start_utils::{ready_handler_http, start_http_server_with_aux_handlers, MetricsState},
    tracing::setup_global_logging,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use synd_chain_ingestor::{config::Config, ingestor, metrics::ChainIngestorMetrics, server};
use tokio::{
    signal::unix::{signal, SignalKind},
    time::sleep,
};
use tracing::{error, info, trace};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Initialize logging
    setup_global_logging()?;

    let cfg = Config::parse();
    let mut provider = cfg.new_provider().await;
    let mut metrics_state = MetricsState::default();
    let metrics = ChainIngestorMetrics::new(&mut metrics_state.registry);

    let is_ready = Arc::new(AtomicBool::new(false));
    let ready_handler = ready_handler_http(is_ready.clone(), "chain ingestor is not ready".into());

    tokio::spawn(start_http_server_with_aux_handlers(
        metrics_state,
        cfg.port,
        None,
        Some(ready_handler),
    ));

    let (module, ctx) = server::start(
        &provider,
        cfg.ws_urls.clone(),
        &cfg.db_file,
        cfg.start_block,
        cfg.parallel_sync_requests,
        &metrics,
    )
    .await?;

    is_ready.store(true, Ordering::SeqCst);
    info!("Chain ingestor marked as ready");

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
            trace!("ingestor connection restarted");
        }
    }
}
