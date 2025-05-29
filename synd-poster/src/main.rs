//! `Synd-Poster` is responsible for extracting the appchain root state and submitting
//! assertions to the settlement chain.

use eyre::Result;
use shared::{
    service_start_utils::{start_metrics_and_health, MetricsState},
    tracing::setup_global_logging,
};
use synd_poster::{config::Config, metrics::PosterMetrics, poster};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    setup_global_logging()?;

    let config = Config::initialize();
    info!("Config: {:?}", config);

    let mut metrics_state = MetricsState::default();
    let metrics = PosterMetrics::new(&mut metrics_state.registry);
    tokio::spawn(start_metrics_and_health(metrics_state, config.metrics_port));

    info!("Starting Poster service...");
    poster::run(config, metrics).await
}
