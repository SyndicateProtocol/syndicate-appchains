//! TC Sequencer is a service that processes and validates transactions
//! before submitting them to TC for sending to the Metabased chain.
//!
//! It provides a JSON-RPC interface for submitting transactions and checking service health.

use eyre::Result;
use shared::logger::set_global_default_subscriber;
use tc_sequencer::{config::Config, server::run_server};
use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> Result<()> {
    // Initialize logging
    set_global_default_subscriber()?;

    // Parse config
    let config = Config::initialize();
    info!("Config: {:?}", config);

    // Start server
    let (addr, handle) = run_server(&config).await?;

    info!(
        addr = %addr,
        "TC Sequencer server running"
    );

    #[allow(clippy::expect_used)]
    let mut sigint = signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");
    #[allow(clippy::expect_used)]
    let mut sigterm = signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");

    tokio::select! {
        _ = sigint.recv() => {
            println!("Received SIGINT (Ctrl+C), initiating shutdown...");
        }
        _ = sigterm.recv() => {
            println!("Received SIGTERM, initiating shutdown...");
        }
    };

    handle.stop()?;
    handle.stopped().await;

    Ok(())
}
