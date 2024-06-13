//! Example of using the HTTP provider with the `reqwest` crate to get the latest block number.

use alloy::providers::{Provider, ProviderBuilder};
use eyre::Result;

async fn get_latest_block() -> Result<()> {
    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = "https://sepolia.optimism.io/".parse()?;

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url);

    // Get latest block number.
    let latest_block = provider.get_block_number().await?;

    println!("Latest block number: {:?}", latest_block);

    Ok(())
}



fn main() -> Result<()> {
    // Create a new Tokio runtime
    let rt = tokio::runtime::Runtime::new()?;

    // Use the runtime to block on the async function
    rt.block_on(async {
        get_latest_block().await
    })?;

    Ok(())

    // next step - use Alloy contract to interact with on-chain contract
    // https://github.com/alloy-rs/alloy/tree/main/crates/contract
}