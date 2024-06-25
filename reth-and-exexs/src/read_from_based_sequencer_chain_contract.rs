use alloy::{primitives::U256, providers::ProviderBuilder, sol};
use dotenv::dotenv;
use eyre::{Context, Result};
use std::env;

sol!(
    #[sol(rpc)]
    BasedSequencerChain,
    "../contracts/out/BasedSequencerChain.sol/BasedSequencerChain.json"
);

pub struct Config {
    pub contract_address: String,
    pub rpc_url: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let contract_address =
            env::var("CONTRACT_ADDRESS").context("CONTRACT_ADDRESS env var is not set")?;
        let rpc_url = env::var("RPC_URL").context("RPC_URL env var is not set")?;
        Ok(Config {
            contract_address,
            rpc_url,
        })
    }
}

pub async fn run() -> Result<()> {
    dotenv().ok();

    // contract address and RPC URL from environment variables
    let settings = Config::new()?;

    let provider = ProviderBuilder::new().on_http(settings.rpc_url.parse()?);

    // contract instance
    let contract = BasedSequencerChain::new(settings.contract_address.parse()?, provider);

    // read data from the contract
    let builder_epoch_length = contract.EPOCH_LENGTH();
    let epoch_length = builder_epoch_length.call().await?._0;

    let builder_initial_epoch = contract.INITIAL_EPOCH_NUMBER();
    let initial_epoch = builder_initial_epoch.call().await?._0;

    let BasedSequencerChain::MAX_BID_LIST_SIZEReturn {
        _0: max_bid_list_size,
    } = contract.MAX_BID_LIST_SIZE().call().await?;
    let BasedSequencerChain::lastNonEmptyEpochNumberReturn {
        _0: last_non_empty_epoch_number,
    } = contract.lastNonEmptyEpochNumber().call().await?;
    let BasedSequencerChain::batchesReturn {
        parent_hash,
        parent_epoch_number,
        epoch_number,
        epoch_hash,
    } = contract.batches(U256::from(0)).call().await?;
    let BasedSequencerChain::calculateEpochNumberReturn {
        _0: epoch_number_timestamp,
    } = contract
        .calculateEpochNumber(U256::from_str_radix("1686331200", 10)?)
        .call()
        .await?;
    let BasedSequencerChain::calculateCurrentEpochNumberReturn {
        _0: current_epoch_number,
    } = contract.calculateCurrentEpochNumber().call().await?;
    let BasedSequencerChain::checkParentHashReturn {
        _0: is_parent_hash_valid,
    } = contract.checkParentHash(epoch_hash).call().await?;

    println!("EPOCH_LENGTH: {}", epoch_length);
    println!("INITIAL_EPOCH_NUMBER: {}", initial_epoch);
    println!("MAX_BID_LIST_SIZE: {}", max_bid_list_size);
    println!("lastNonEmptyEpochNumber: {}", last_non_empty_epoch_number);
    println!(
        "Batch: parent_hash={:?}, parent_epoch_number={}, epoch_number={}, epoch_hash={:?}",
        parent_hash, parent_epoch_number, epoch_number, epoch_hash
    );
    println!(
        "Epoch number for timestamp 1686331200: {}",
        epoch_number_timestamp
    );
    println!("Current epoch number: {}", current_epoch_number);
    println!("Is parent hash valid? {}", is_parent_hash_valid);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{network::EthereumWallet, node_bindings::Anvil, signers::local::PrivateKeySigner};

    #[tokio::test]
    async fn test_reading_data() -> Result<()> {
        let anvil = Anvil::new().try_spawn()?;

        // Set up signer from the first default Anvil account (Alice).
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = EthereumWallet::from(signer);

        // create a provider with the wallet.
        let rpc_url = anvil.endpoint().parse()?;
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url);

        println!("Anvil running at `{}`", anvil.endpoint());

        // Deploy the contract.
        let contract = BasedSequencerChain::deploy(&provider).await?;

        println!("Deployed contract at address: {}", contract.address());

        let epoch_length = contract.EPOCH_LENGTH().call().await?._0;

        println!("EPOCH_LENGTH: {}", epoch_length);

        assert!(
            epoch_length > U256::from(0),
            " EPOCH_LENGTH should have been 0."
        );

        Ok(())
    }
}
