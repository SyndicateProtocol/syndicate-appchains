//! Metabased Poster is responsible for extracting the appchain root state and submitting
//! assertions to the settlement chain.

use eyre::Result;
use metabased_poster::{config::Config, metrics::PosterMetrics, poster};
use shared::{
    logger::set_global_default_subscriber,
    service_start_utils::{start_metrics_and_health, MetricsState},
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    set_global_default_subscriber()?;

    let config = Config::initialize();
    info!("Config: {:?}", config);

    let mut metrics_state = MetricsState::default();
    let metrics = PosterMetrics::new(&mut metrics_state.registry);
    tokio::spawn(start_metrics_and_health(metrics_state, config.metrics_port));

    info!("Starting Poster service...");
    poster::run(config, metrics).await
}
