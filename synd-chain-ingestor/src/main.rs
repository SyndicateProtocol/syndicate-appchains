//! The `synd-chain-ingestor` serves blocks from a chain to all appchains that use the chain.

use clap::Parser;
use jsonrpsee::server::{
    middleware::http::ProxyGetRequestLayer, PingConfig, Server, ServerConfigBuilder,
};
use shared::{
    service_start_utils::{start_http_server_with_metrics_only, MetricsState},
    tracing::{setup_global_tracing, ServiceTracingConfig},
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};
use synd_chain_ingestor::{
    config::Config,
    ingestor,
    metrics::ChainIngestorMetrics,
    server::{create_module, sync_db, Context},
};
use tokio::{
    signal::unix::{signal, SignalKind},
    time::sleep,
};
use tracing::{error, info, trace};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Initialize logging
    setup_global_tracing(ServiceTracingConfig::from_env(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    ))?;

    let cfg = Config::parse();
    let mut provider = cfg.new_provider().await;
    let mut metrics_state = MetricsState::default();
    let metrics = ChainIngestorMetrics::new(&mut metrics_state.registry);

    let is_ready = Arc::new(AtomicBool::new(false));
    tokio::spawn(start_http_server_with_metrics_only(metrics_state, cfg.metrics_port));

    // Create context with None DB
    let ctx = Arc::new(Mutex::new(Context { db: None, subs: Default::default() }));

    // Create and start the JSON-RPC server immediately
    info!("starting synd-chain-ingestor server on port {} (syncing)", cfg.port);
    let module = create_module(ctx.clone(), cfg.ws_urls.clone(), is_ready.clone());
    let jsonrpsee_cfg = ServerConfigBuilder::new().enable_ws_ping(PingConfig::default()).build();
    let http_middleware = tower::builder::ServiceBuilder::new()
        .layer(ProxyGetRequestLayer::new([("/health", "health"), ("/ready", "ready")])?);
    let _handle = Server::builder()
        .set_config(jsonrpsee_cfg)
        .set_http_middleware(http_middleware)
        .build(format!("0.0.0.0:{}", cfg.port))
        .await?
        .start(module);

    // Sync the DB
    let chain_id = provider.get_chain_id().await;
    let db = sync_db(
        &provider,
        &cfg.db_file,
        cfg.start_block,
        chain_id,
        cfg.parallel_sync_requests,
        &metrics,
    )
    .await?;

    // Update the context with the synced DB
    {
        #[allow(clippy::expect_used)]
        let mut lock =
            ctx.lock().map_err(|e| eyre::eyre!("Failed to acquire context lock: {}", e))?;
        lock.db = Some(db);
    }
    is_ready.store(true, Ordering::SeqCst);
    info!("Chain ingestor is ready (sync complete)");

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
