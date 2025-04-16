//! Maestro is a service that filters and coordinates transaction requests to sequencers

use eyre::Result;
use maestro::{config::Config, errors::Error};
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use shared::logger::set_global_default_subscriber;
use tokio::signal::unix::{signal, SignalKind};
use tracing::{info, log::warn};

#[tokio::main]
#[allow(clippy::redundant_pub_crate)]
async fn main() -> Result<()> {
    // Initialize logging
    set_global_default_subscriber()?;

    let config = Config::initialize();
    info!("Config: {:?}", config);

    config.validate().await?;

    // TODO metrics, if necessary

    match config.redis_address {
        None => {
            warn!("Redis is disabled")
        }
        Some(ref redis_address) => {
            let (_redis_client, _redis_conn) = connect(redis_address.to_string()).await?;
            info!("Connected to Redis successfully!");
        }
    }

    let (addr, handle) = maestro::server::run(config).await?;
    info!(
        %addr,
        "Maestro server running"
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

async fn connect(redis_address: String) -> Result<(Client, MultiplexedConnection), Error> {
    info!("Connecting to Redis server at {}...", redis_address);
    let client = Client::open(redis_address)?;
    info!("Got Redis client");
    let mut conn = client.get_multiplexed_async_connection().await?;
    info!("Got Redis connection");

    // Test an operation on Redis
    conn.set::<&str, &str, ()>("test_key", "test_value").await?;

    let value: String = conn.get("test_key").await?;

    info!("Redis test value set: {}", value);
    Ok((client, conn))
}
