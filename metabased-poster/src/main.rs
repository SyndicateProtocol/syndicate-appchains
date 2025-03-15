//! Metabased Poster is responsible for extracting the appchain root state and submitting
//! assertions to the settlement chain.

use eyre::Result;
use metabased_poster::config::Config;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    // Initialize logging
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .json()
        .with_target(true)
        .with_env_filter("info")
        .init();

    let config = Config::initialize();
    info!("Config: {:?}", config);

    // TODO (SEQ-695): Poster metrics

    // TODO - start channels

    // TODO - start poller

    // TODO (SEQ-689) - start submitter

    info!("Starting Poster service...");

    Ok(())
}
