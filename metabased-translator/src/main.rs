use alloy_primitives::{Address, B256, U256};
use eyre::Result;
use metabased_translator::config::cli;
use metabased_translator::connectors::anvil;
use metabased_translator::rollups::optimism::batch::{new_batcher_tx, Batch};
use metabased_translator::rollups::optimism::frame::to_data;
use std::str::FromStr;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    cli::init_tracing_subscriber();

    // Initialize logging (optional, depending on your setup)

    // Define the batch
    let batch = Batch {
        parent_hash: B256::from_str(
            "0xe009262cd1adf34cfaf845fd1c17a6ddb7f97c67b2992cd9f286ff4e1c6ad233",
        )
        .unwrap(),
        epoch_num: 0,
        epoch_hash: B256::from_str(
            "0x103ac73bf5b87545625259521c3c53c9f51f08c782831a5eb216c6383ddb201d",
        )
        .unwrap(),
        timestamp: 1712500002,
        transactions: vec![],
    };

    // Generate frames and convert to data
    let frames = batch.get_frames(1000000).unwrap();
    info!("frames : {:?}", frames);
    let data = to_data(&frames).unwrap();

    // Expected data
    let expected_data = hex::decode("0081c27ac9f21c4f1abd1679d7bddcd69300000000005e7801004e00b1ffb84c00f849a0e009262cd1adf34cfaf845fd1c17a6ddb7f97c67b2992cd9f286ff4e1c6ad23380a0103ac73bf5b87545625259521c3c53c9f51f08c782831a5eb216c6383ddb201d846612ad22c0010000ffff381525a701")?;

    // Log the data
    info!("Data (hex): {:x?}", data);
    info!("Expected Data (hex): {:x?}", expected_data);

    Ok(())
}
