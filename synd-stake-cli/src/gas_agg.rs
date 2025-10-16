//! The `gas-agg` module contains the functions for aggregating gas usage from appchains.

use alloy::{
    primitives::{Address, U256},
    providers::ProviderBuilder,
};
use clap::Args;
use contract_bindings::synd::gas_aggregator::GasAggregator;
use shared::{
    parse::{parse_address, parse_url},
    types::new_provider,
};
use tracing::{debug, error, info};
use url::Url;

/// Arguments for the `gas-agg` command.
///
/// This struct defines the command-line arguments that can be passed to the gas-agg command.
/// The gas-agg command is used to aggregate gas usage from appchains.
#[derive(Args, Debug)]
pub struct GasAggArgs {
    /// The private key to use for the transaction.
    #[arg(short = 'k', long, env = "PRIVATE_KEY")]
    pub private_key: String,

    /// The address of the gas aggregator contract.
    #[arg(
        short = 'a',
        long,
        env = "GAS_AGGREGATOR_ADDRESS",
        value_parser = parse_address,
    )]
    pub gas_aggregator_address: Address,

    /// The RPC URL to use for the transaction.
    #[arg(short = 'r', long, env = "RPC_URL", default_value = "", value_parser = parse_url)]
    pub rpc_url: Url,
}

/// Aggregates gas usage from appchains.
///
/// This function processes and submits the aggregation transaction for the gas aggregator contract.
/// The gas aggregator contract is used to collect and store gas usage data from appchains.
///
/// # Arguments
///
/// * `args` - The command-line arguments containing configuration options
///
/// # Examples
///
/// ```bash
/// # Run a normal emission
/// synd-stake-cli gas-agg -k <private_key>
///
/// # Run in simulation mode
/// synd-stake-cli gas-agg --sim
/// ```
///
/// # Errors
///
/// This function may return an error if:
/// - The transaction/simulation fails
#[allow(clippy::cognitive_complexity)]
pub async fn gas_agg(args: &GasAggArgs) {
    // TODO (ENG-2111): Use shared provider function
    let provider = ProviderBuilder::new()
        .connect(args.rpc_url.as_str())
        .await
        .unwrap_or_else(|e| panic!("Failed to connect to RPC URL '{}': {}", args.rpc_url, e));

    let gas_aggregator = GasAggregator::new(args.gas_aggregator_address, provider);
    if gas_aggregator.currentEpoch().call().await.unwrap_or_else(|e| {
        panic!(
            "Failed to call currentEpoch on gas aggregator contract: {e}
    "
        )
    }) == gas_aggregator.getCurrentEpoch().call().await.unwrap_or_else(|e| {
        panic!("Failed to call getCurrentEpoch on gas aggregator contract: {e} ")
    }) {
        info!("Epoch not over");
        return;
    }

    let start_index = gas_aggregator
        .currentAggregateIndex()
        .call()
        .await
        .unwrap_or_else(|e| panic!("failed to get current aggregate index: {e}"));

    info!("Calling gas aggregation...");
    let mut index = U256::ZERO;
    let mut chains = Vec::<U256>::new();
    let mut tokens = Vec::<U256>::new();

    loop {
        let (next_index, new_chains, new_tokens): (U256, Vec<U256>, Vec<U256>) = gas_aggregator
            .simulateAggregateTokens(index, chains.clone(), tokens.clone())
            .call()
            .await
            .unwrap_or_else(|e| panic!("failed to simulate aggregate tokens: {e}"))
            .into();
        if index >= start_index {
            let _ = gas_aggregator
                .aggregateTokens(chains, tokens)
                .send()
                .await
                .unwrap_or_else(|e| panic!("failed to aggregate tokens: {e}"));
        }
        index = next_index;
        chains = new_chains;
        tokens = new_tokens;
        debug!("Index: {:?}, Chains: {:?}, Tokens: {:?}", index, chains, tokens);

        if index == U256::ZERO {
            break;
        }
    }

    match GasAggregator::new(
        args.gas_aggregator_address,
        new_provider(&args.rpc_url, &args.private_key).await,
    )
    .aggregateTokens(Vec::<U256>::new(), Vec::<U256>::new())
    .send()
    .await
    {
        Ok(tx) => {
            info!("Gas aggregation succeeded: {}", tx.tx_hash());
        }
        Err(e) => {
            error!("Error aggregating gas. Error: {}", e);
        }
    }
}
