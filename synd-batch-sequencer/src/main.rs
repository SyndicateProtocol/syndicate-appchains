//! TC Sequencer is a service that processes and validates transactions
//! before submitting them to TC for sending to the Appchain.
//!
//! It provides a JSON-RPC interface for submitting transactions and checking service health.

use batcher::{batcher::run_batcher, metrics::BatcherMetrics};
use eyre::Result;
use shared::{
    logger::set_global_default_subscriber,
    service_start_utils::{start_metrics_and_health, MetricsState},
};
use synd_batch_sequencer::config::BatchSequencerConfig;
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> Result<()> {
    // Initialize logging
    set_global_default_subscriber()?;

    // Parse config
    let config = BatchSequencerConfig::initialize();
    info!("BatchSequencerConfig: {:?}", config);

    // Validate config and create TC client if needed
    let tc_client = config.validate().await?;

    let mut metrics_state = MetricsState::default();
    let metrics = BatcherMetrics::new(&mut metrics_state.registry);
    tokio::spawn(start_metrics_and_health(metrics_state, config.metrics_port));

    // Start batcher
    let batcher_handle = run_batcher(
        &config.batcher,
        tc_client,
        config.wallet_pool_address,
        config.sequencing_address,
        metrics,
    )
    .await?;

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
    }

    Ok(())
}
