//! Metabased Sequencer is a service that processes and validates transactions
//! before submitting them to the Metabased chain.
//!
//! It provides a JSON-RPC interface for submitting transactions and checking service health.

use eyre::Result;
use metabased_sequencer::{
    config::Config,
    metrics::{start_metrics, MetricsState, RelayerMetrics},
    server::run_server,
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .json()
        .with_target(true)
        .with_env_filter("info")
        .init();

    // Parse config
    let config = Config::initialize();
    info!("Config: {:?}", config);

    // Initialize metrics
    let mut metrics = MetricsState::new();
    let relayer_metrics = RelayerMetrics::new(&mut metrics.registry);
    let metrics_handler = start_metrics(metrics, config.metrics_port).await.unwrap_or_else(|e| {
        panic!("Failed to start metrics on port {}: {}", config.metrics_port, e)
    });

    // Start server
    let (addr, handle) = run_server(&config, relayer_metrics).await?;

    info!(
        addr = %addr,
        "Metabased Sequencer v2 server running"
    );

    // Keep the server running
    tokio::select! {
        _ = handle.stopped() => {}
        _ = metrics_handler => {}
    }

    Ok(())
}
