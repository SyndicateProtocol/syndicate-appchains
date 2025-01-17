//! The `ingestor` module  handles block polling from a remote Ethereum chain and forwards them to a consumer using a channel

mod eth_client;
mod ingestor;

use eyre::eyre;
use std::{error::Error, time::Duration};
use tracing::info;

/// This function initializes the `Ingestor` to poll blocks from an Ethereum chain
/// and logs received blocks. It sets up logging, handles errors gracefully, and
/// spawns a background task to process incoming blocks.
///
/// **This function is intended for internal testing purposes only.**
/// It demonstrates how to use the `Ingestor` to poll and log blocks from an Ethereum chain.
/// It is not designed for production use and should be adapted or replaced for
/// deployment in production environments.
///
/// # Arguments
/// - `rpc_url`: The RPC endpoint URL of the Ethereum chain.
/// - `start_block`: The block number to start polling from.
/// - `polling_interval`: The time interval between each block polling.
///
/// # Returns
/// A tuple containing the `Ingestor` instance and a `Receiver` for consuming blocks.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = "https://base.llamarpc.com"; //"https://eth.llamarpc.com";
    let start_block = 19486923;
    let polling_interval = Duration::from_secs(1);

    // Create the ingestor and receiver
    let (mut ingestor, mut receiver) =
        ingestor::Ingestor::new(rpc_url, start_block, 100, polling_interval)
            .await
            .map_err(|e| eyre!("Failed to create ingestor: {:?}", e))?;

    // Spawn a task to log what the receiver receives
    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            info!("Received block number: {:?}", message.block.number);
            info!("Received block: {:?}", message);
        }
    });

    // Start polling
    ingestor
        .start_polling()
        .await
        .map_err(|e| eyre!("Failed to start polling: {:?}", e))?;

    Ok(())
}
