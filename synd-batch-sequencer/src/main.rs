//! The Batch Sequencer is a service that processes and validates transactions
//! before submitting them to the Appchain.

use batcher::batcher::run_batcher;
use eyre::Result;
use shared::tracing::{setup_global_tracing, ServiceTracingConfig};
use synd_batch_sequencer::config::BatchSequencerConfig;
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> Result<()> {
    let _guard = setup_global_tracing(ServiceTracingConfig::from_env(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    ))?;

    let config = BatchSequencerConfig::initialize();
    info!("BatchSequencerConfig: {:?}", config);

    let batcher_handle =
        run_batcher(&config.batcher, config.sequencing_address, config.metrics_port).await?;

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
