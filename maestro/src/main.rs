//! Maestro is a service that filters and coordinates transaction requests to sequencers

use eyre::Result;
use maestro::config::Config;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> Result<()> {
    // Initialize logging
    FmtSubscriber::builder().with_max_level(Level::DEBUG).json().with_target(true).init();

    let config = Config::initialize();
    info!("Config: {:?}", config);

    // TODO metrics, if necessary

    let (addr, handle) = maestro::server::run(config.port).await?;

    info!(
        addr = %addr,
        "Maestro server running"
    );

    // Keep the server running
    tokio::select! {
        _ = handle.stopped() => {}
    }
    Ok(())
}
