//! TC Sequencer is a service that processes and validates transactions
//! before submitting them to TC for sending to the Metabased chain.
//!
//! It provides a JSON-RPC interface for submitting transactions and checking service health.

use eyre::Result;
use tc_sequencer::{config::Config, server::run_server};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> Result<()> {
    // Initialize logging
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .json()
        .with_target(true)
        .with_env_filter("info")
        .init();

    // Parse config
    let config = Config::initialize();
    info!("Config: {:?}", config);

    // Start server
    let (addr, handle) = run_server(&config).await?;

    info!(
        addr = %addr,
        "TC Sequencer server running"
    );

    // Keep the server running
    handle.stopped().await;

    Ok(())
}
