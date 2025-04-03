//! Maestro is a service that filters and coordinates transaction requests to sequencers

use eyre::Result;
use maestro::{
    config::Config,
    redis_manager,
    redis_manager::{StreamManager, TransactionRequest},
};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{error, info, Level};
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

    let (_redis_client, redis_conn) = redis_manager::connect(config).await?;

    info!("Connected to Redis successfully!");

    // Initialize stream manager
    let mut stream_manager = StreamManager::new(redis_conn.clone());

    // Initialize the consumer group
    stream_manager.init_consumer_group().await?;

    // Example transaction enqueue (for testing)
    let example_tx = TransactionRequest {
        tx_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
        sender: "0xabcdef1234567890abcdef1234567890abcdef12".to_string(),
        raw_tx: "0x02f8b303808459682f0085059ba5a4dc82520894a0b86991c6218b36c1d19d4a2e9eb0ce3606eb4880de0b6b3a764000080c080a0a7a56c53a21c1f0b636f222b5bf148c15ecec50e7a9425c0ebb681b61a5c9b7a0144d7a2d62f771de4225cce7c8463f35848ff6c10f4f791b8ed0337a06f0fd65".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs(),
    };
    let tx_id = stream_manager.enqueue_transaction(example_tx).await?;
    info!("Enqueued example transaction with ID: {}", tx_id);

    // Start processing in a separate task
    let mut stream_manager_clone = StreamManager::new(redis_conn.clone());
    tokio::spawn(async move {
        match stream_manager_clone.start_processing_loop().await {
            Ok(_) => unreachable!("Processing loop should run indefinitely"),
            Err(e) => error!("Stream processing loop failed: {}", e),
        }
    });

    // Keep the server running
    tokio::select! {
        _ = handle.stopped() => {}
    }
    Ok(())
}
