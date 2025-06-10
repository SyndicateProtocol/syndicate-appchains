//! `Synd-Proposer` is responsible for extracting the appchain root state and submitting
//! assertions to the settlement chain `AssertionPoster` contract.

use eyre::Result;
use shared::{
    service_start_utils::{start_metrics_and_health, MetricsState},
    tracing::setup_global_logging,
};
use synd_proposer::{config::Config, metrics::ProposerMetrics, proposer};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    setup_global_logging()?;

    let config = Config::initialize();
    info!("Config: {:?}", config);

    // Validate config early and exit with non-zero code if invalid
    if let Err(e) = config.validate() {
        error!("Configuration validation failed: {}", e);
        std::process::exit(1);
    }

    let mut metrics_state = MetricsState::default();
    let metrics = ProposerMetrics::new(&mut metrics_state.registry);
    tokio::spawn(start_metrics_and_health(metrics_state, config.metrics_port, None));

    info!("Starting Proposer service...");
    match proposer::run(config, metrics).await {
        Ok(()) => Ok(()),
        Err(e) => {
            error!("Proposer service failed: {}", e);
            std::process::exit(1);
        }
    }
}
