//! The Batch Sequencer is a service that processes and validates transactions
//! before submitting them to the Appchain.

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

    let config = BatchSequencerConfig::initialize();
    info!("BatchSequencerConfig: {:?}", config);

    let mut metrics_state = MetricsState::default();
    let metrics = BatcherMetrics::new(&mut metrics_state.registry);

    let (batcher_handle, valkey_conn) =
        run_batcher(&config.batcher, config.sequencing_address, metrics).await?;

    tokio::spawn(start_metrics_and_health(
        metrics_state,
        config.metrics_port,
        Some(valkey_health_handler),
    ));

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

/// Checks if the Valkey connection is healthy
/// This method attempts to ping the Valkey connection to check if it is healthy.
async fn valkey_health_handler(mut valkey_conn: MultiplexedConnection) -> impl IntoResponse {
    let health: Result<String, _> = valkey_conn.ping().await;
    match health {
        Ok(_) => Json(serde_json::json!({ "health": true })),
        Err(e) => {
            error!("Valkey connection is not healthy: {:?}", e);
            Json(
                serde_json::json!({ "health": false, "code": 500, "message": "Valkey connection is not healthy" }),
            )
        }
    }
}
