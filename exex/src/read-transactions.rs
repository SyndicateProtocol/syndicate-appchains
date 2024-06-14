//! Example of using the HTTP provider with the `reqwest` crate to get the latest block number.

use alloy::core::sol;
use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder};
use eyre::Result;

// https://github.com/alloy-rs/examples/blob/main/examples/providers/examples/http.rs
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

// https://github.com/alloy-rs/alloy/tree/main/crates/contract
async fn interactWithContract() -> Result<()> {
    sol! {
        #[sol(rpc)]
        "../contracts/src/BasedSequencerChain.sol"
    }
    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = "https://sepolia.optimism.io/".parse()?;
    let provider = ProviderBuilder::new().with_recommended_fillers().on_builtin("https://sepolia.optimism.io/").await?;


    // // Create a provider with the HTTP transport using the `reqwest` crate.
    // let provider = ProviderBuilder::new().on_http(rpc_url);

    // Get latest block number.
    let latest_block = provider.get_block_number().await?;

    println!("Latest block number: {:?}", latest_block);


    // Create a contract instance.
    let addr_str = "0x66F9664f97f2B50F62d13EA064982F936de76657";
    let addr: Address = Address::parse_checksummed(addr_str, Some(30)).unwrap();

    let contract = BasedSequencerChain::new(addr, &provider);
    // contract.do

    // Call the contract
    contract.calculateCurrentEpochNumber();

    // println!("WETH total supply is {_0}");

    Ok(())
}



fn main() -> Result<()> {
    // Create a new Tokio runtime - for async
    let rt = tokio::runtime::Runtime::new()?;

    // Use the runtime to block on the async function
    rt.block_on(async {
        get_latest_block().await
    })?;

    rt.block_on(async {
      interactWithContract().await
    })?;

    Ok(())

    // next step - use Alloy contract to interact with on-chain contract
    // https://github.com/alloy-rs/alloy/tree/main/crates/contract

    // https://alloy.rs/examples/sol-macro/contract.html
}