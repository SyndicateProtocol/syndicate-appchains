//! Maestro is a service that filters and coordinates transaction requests to sequencers

use eyre::Result;
use shared::{
    service_start_utils::{start_http_server_with_metrics_only, MetricsState},
    tracing::{setup_global_tracing, ServiceTracingConfig},
};
use synd_maestro::{config::Config, metrics::MaestroMetrics};
use tokio::signal::unix::{signal, SignalKind};
use tracing::{error, info};

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> Result<()> {
    let _guard = setup_global_tracing(ServiceTracingConfig::from_env(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    ))?;

    let config = Config::initialize();
    info!("Config: {:?}", config);

    let mut metrics_state = MetricsState::default();
    let metrics = MaestroMetrics::new(&mut metrics_state.registry);
    tokio::spawn(start_http_server_with_metrics_only(metrics_state, config.metrics_port));

    // Server::run creates the service and returns a shutdown function that also handles stopping
    // the server handle.
    let (addr, shutdown_fn) = synd_maestro::server::run(config, metrics).await?;
    info!(
        %addr,
        "Maestro server running"
    );

    #[allow(clippy::expect_used)]
    let mut sigint = signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");
    #[allow(clippy::expect_used)]
    let mut sigterm = signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");

    tokio::select! {
        _ = sigint.recv() => {
            info!("Received SIGINT (Ctrl+C), initiating shutdown...");
        }
        _ = sigterm.recv() => {
            info!("Received SIGTERM, initiating shutdown...");
        }
    }

    info!("Initiating Maestro shutdown...");
    if let Err(e) = shutdown_fn().await {
        error!("Error during shutdown: {:?}", e);
    }
    info!("Maestro shutdown completed.");
    Ok(())
}
