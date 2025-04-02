//! Maestro is a service that filters and coordinates transaction requests to sequencers

use eyre::Result;
use maestro::{config::Config, errors::Error};
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
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

    let (_redis_client, _redis_conn) = connect(config).await?;

    info!("Connected to Redis successfully!");

    // Keep the server running
    tokio::select! {
        _ = handle.stopped() => {}
    }
    Ok(())
}

async fn connect(config: Config) -> Result<(Client, MultiplexedConnection), Error> {
    info!("Connecting to Redis server at {}...", config.redis_address);
    let client = Client::open(config.redis_address)?;
    info!("Got Redis client");
    let mut conn = client.get_multiplexed_async_connection().await?;
    info!("Got Redis connection");

    // Test an operation on Redis
    conn.set::<&str, &str, ()>("test_key", "test_value").await?;

    let value: String = conn.get("test_key").await?;

    info!("Redis test value set: {}", value);
    Ok((client, conn))
}
