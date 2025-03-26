//! Metabased Poster is responsible for extracting the appchain root state and submitting
//! assertions to the settlement chain.

use eyre::Result;
use metabased_poster::{
    config::Config,
    metrics::{start_metrics, MetricsState, PosterMetrics},
    poster,
};
use prometheus_client::registry::Registry;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .json()
        .with_target(true)
        .with_env_filter("info")
        .init();

    let config = Config::initialize();
    info!("Config: {:?}", config);

    let mut registry = Registry::default();
    let metrics = PosterMetrics::new(&mut registry);
    let metrics_state = MetricsState { registry };
    tokio::spawn(start_metrics(metrics_state, config.metrics_port));

    info!("Starting Poster service...");
    poster::run(&config, metrics).await
}
