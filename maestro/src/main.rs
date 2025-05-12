//! Maestro is a service that filters and coordinates transaction requests to sequencers

use eyre::Result;
use maestro::config::Config;
use shared::{
    logger::set_global_default_subscriber,
    service_start_utils::{start_metrics_and_health, MetricsState},
};
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> Result<()> {
    // Initialize logging
    set_global_default_subscriber()?;

    let config = Config::initialize();
    info!("Config: {:?}", config);

    // Maestro metrics
    // TODO(SEQ-449): Expose Prometheus metrics endpoint for Maestro
    let metrics_state = MetricsState::default();
    tokio::spawn(start_metrics_and_health(metrics_state, config.metrics_port));

    let (addr, handle) = maestro::server::run(config).await?;
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
    };

    handle.stop()?;
    handle.stopped().await;

    Ok(())
}
