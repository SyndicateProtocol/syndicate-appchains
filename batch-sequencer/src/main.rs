//! TC Sequencer is a service that processes and validates transactions
//! before submitting them to TC for sending to the Metabased chain.
//!
//! It provides a JSON-RPC interface for submitting transactions and checking service health.

use batcher::{batcher::run_batcher, config::BatcherConfig};
use eyre::Result;
use shared::{
    logger::set_global_default_subscriber,
    metrics::{start_metrics, MetricsState},
};
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> Result<()> {
    // Initialize logging
    set_global_default_subscriber()?;

    // Parse config
    let config = BatcherConfig::initialize();
    info!("BatcherConfig: {:?}", config);

    // Start batcher
    let batcher_handle = run_batcher(&config).await?;

    // Batcher metrics
    // TODO(SEQ-868): Batcher metrics
    let metrics_state = MetricsState::default();
    tokio::spawn(start_metrics(metrics_state, config.metrics_port));

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
        _ = batcher_handle => {
            info!("Batcher stopped, initiating shutdown...");
        }
    };

    Ok(())
}
