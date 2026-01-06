//! The `MockChain` is used for appchain block derivation.

use std::fs;
use tracing::info;

fn datadir_is_empty(datadir: &std::path::Path) -> bool {
    fs::read_dir(datadir)
        .map(|entries| {
            let files: Vec<_> =
                entries.filter_map(|e| e.ok()).filter(|e| e.file_name() != "lost+found").collect();

            if !files.is_empty() {
                info!("datadir {} is not empty, found {} files", datadir.display(), files.len());
                for file in &files {
                    info!("  {}", file.path().display());
                }
            }

            files.is_empty()
        })
        .unwrap_or(true) // If directory doesn't exist, treat as empty
}

#[tokio::main]
#[cfg(feature = "rocksdb")]
async fn main() -> eyre::Result<()> {
    use clap::Parser;
    use jsonrpsee::{
        server::{
            middleware::http::ProxyGetRequestLayer, PingConfig, RandomStringIdProvider, Server,
            ServerConfigBuilder,
        },
        ws_client::RpcServiceBuilder,
    };
    use rocksdb::DB;
    use shared::{
        service_start_utils::{start_http_server_with_metrics_only, MetricsState},
        tracing::setup_global_logging,
    };
    use std::path::Path;
    use synd_mchain::{
        config::{load_snapshot, with_onchain_config, MchainConfig},
        metrics::MchainMetrics,
        server::start_mchain,
    };
    use tokio::signal::unix::{signal, SignalKind};
    use tracing::{info, warn};

    // Initialize logging
    setup_global_logging()?;

    let cfg = with_onchain_config(MchainConfig::parse()).await;

    // Load snapshot if URL is provided
    if let Some(ref snapshot_url) = cfg.snapshot_url {
        if datadir_is_empty(Path::new(&cfg.datadir)) {
            load_snapshot(snapshot_url, &cfg.datadir).await?;
        } else {
            warn!("datadir {} is not empty, skipping snapshot load", cfg.datadir);
        }
    }

    info!("loading rocksdb db {}", cfg.datadir);
    let db = DB::open_default(cfg.datadir.clone())?;

    let mut metrics_state = MetricsState::default();
    let metrics = MchainMetrics::new(&mut metrics_state.registry);

    info!("starting synd-mchain server on port {}", cfg.port);
    tokio::spawn(start_http_server_with_metrics_only(metrics_state, cfg.metrics_port));
    let module = start_mchain(
        cfg.appchain_chain_id,
        cfg.finality_delay,
        cfg.genesis_config.clone(),
        cfg.migration_config(),
        db,
        metrics,
    );
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
