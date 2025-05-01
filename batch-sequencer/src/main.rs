//! TC Sequencer is a service that processes and validates transactions
//! before submitting them to TC for sending to the Metabased chain.
//!
//! It provides a JSON-RPC interface for submitting transactions and checking service health.

use batch_sequencer::config::BatchSequencerConfig;
use batcher::batcher::run_batcher;
use eyre::Result;
use shared::{
    logger::set_global_default_subscriber,
    metrics::{start_metrics, MetricsState},
};
use tc_client::tc_client::TCClient;
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

    let tc_client = config.use_tc.then(|| {
        config.tc.as_ref().map_or_else(
            || unreachable!("TCConfig must be Some when use_tc is true (asserted during initialization)"),
            |tc_config| TCClient::new(
                tc_config,
                config.wallet_pool_address,
                config.sequencing_address,
            ),        )
    }).transpose()?;

    // Start batcher
    let batcher_handle = run_batcher(
        &config.batcher,
        tc_client,
        config.wallet_pool_address,
        config.sequencing_address,
    )
    .await?;

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
