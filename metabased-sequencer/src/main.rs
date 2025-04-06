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
use shared::logger::set_global_default_subscriber;
use tracing::info;

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> Result<()> {
    // Initialize logging
    set_global_default_subscriber()?;

    // Parse config
    let config = Config::initialize();
    info!("Config: {:?}", config);

    // Initialize metrics
    let mut metrics = MetricsState::new();
    let relayer_metrics = RelayerMetrics::new(&mut metrics.registry);
    let metrics_handler = start_metrics(metrics, config.metrics_port).await;

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
