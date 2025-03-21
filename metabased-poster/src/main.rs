//! Metabased Poster is responsible for extracting the appchain root state and submitting
//! assertions to the settlement chain.

use eyre::Result;
use metabased_poster::{config::Config, poster};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    // Initialize logging
    FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .json()
        .with_target(true)
        .with_env_filter("info")
        .init();

    let config = Config::initialize();
    info!("Config: {:?}", config);

    let runtime = tokio::runtime::Runtime::new()?;

    info!("Starting Poster service...");
    runtime.block_on(poster::run(&config))?;
    Ok(())
}
