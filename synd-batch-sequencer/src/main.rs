//! The Batch Sequencer is a service that pulls transactions off the queue, processes and validates
//! them, and submits them to the Appchain in batches (can be compressed or not).

use clap::Parser;
use eyre::Result;
use shared::tracing::{setup_global_tracing, ServiceTracingConfig};
use std::sync::Arc;
use synd_batch_sequencer::{batcher::run_batcher, config::BatcherConfig};
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> Result<()> {
    let _guard = setup_global_tracing(ServiceTracingConfig::from_env(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    ))?;

    let config = BatcherConfig::parse();
    info!("BatcherConfig: {:?}", config);

    let batcher_handle = run_batcher(Arc::from(config)).await?;

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
