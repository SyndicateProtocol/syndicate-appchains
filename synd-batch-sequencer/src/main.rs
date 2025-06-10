//! The Batch Sequencer is a service that processes and validates transactions
//! before submitting them to the Appchain.

use batcher::batcher::run_batcher;
use eyre::Result;
use shared::tracing::{setup_global_tracing, ServiceTracingConfig};
use synd_batch_sequencer::config::BatchSequencerConfig;
use tokio::signal::unix::{signal, SignalKind};
use tracing::{error, info};

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> Result<()> {
    let _guard = setup_global_tracing(ServiceTracingConfig::from_env(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    ))?;

    let config = BatchSequencerConfig::initialize();
    info!("BatchSequencerConfig: {:?}", config);

    // Validate config early and exit with non-zero code if invalid
    if let Err(e) = config.validate().await {
        error!("Configuration validation failed: {}", e);
        std::process::exit(1);
    }

    let batcher_handle =
        match run_batcher(&config.batcher, config.sequencing_address, config.metrics_port).await {
            Ok(handle) => handle,
            Err(e) => {
                error!("Failed to start batcher: {}", e);
                std::process::exit(1);
            }
        };

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
