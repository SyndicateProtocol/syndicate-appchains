//! The `gas-agg` module contains the functions for aggregating gas usage from appchains.

use alloy::{primitives::Address, providers::ProviderBuilder};
use clap::Args;
use contract_bindings::synd::gas_aggregator::GasAggregator;
use shared::{parse::parse_address, types::new_provider};

/// Arguments for the `gas-agg` command.
///
/// This struct defines the command-line arguments that can be passed to the gas-agg command.
/// The gas-agg command is used to aggregate gas usage from appchains.
#[derive(Args, Debug)]
pub struct GasAggArgs {
    /// Run in simulation mode without actually executing the aggregation.
    /// When enabled, the command will perform simulate the transaction.
    #[arg(short = 's', long, default_value_t = false)]
    pub sim: bool,

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
    #[arg(short = 'r', long, env = "RPC_URL", default_value = "")]
    pub rpc_url: String,
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
pub async fn gas_agg(args: &GasAggArgs) {
    let provider = ProviderBuilder::new()
        .connect(args.rpc_url.as_str())
        .await
        .unwrap_or_else(|e| panic!("Failed to connect to RPC URL '{}': {}", args.rpc_url, e));

    let gas_aggregator = GasAggregator::new(args.gas_aggregator_address, provider);
    if gas_aggregator
        .pendingEpoch()
        .call()
        .await
        .unwrap_or_else(|e| panic!("Failed to call pendingEpoch on gas aggregator contract: {}", e)) ==
        gas_aggregator.getCurrentEpoch().call().await.unwrap_or_else(|e| {
            panic!("Failed to call getCurrentEpoch on gas aggregator contract: {}", e)
        })
    {
        println!("Epoch not over");
        return;
    }

    if args.sim {
        println!("Simulating gas aggregation...");
        match gas_aggregator.aggregateTokensUsed().call().await {
            Ok(_) => {
                println!("Simulation succeeded")
            }
            Err(e) => {
                println!("Simulation failed");
                println!("--------------------------------");
                println!("{}", e);
                println!("--------------------------------");
            }
        }
    } else {
        println!("Aggregating gas...");
        match GasAggregator::new(
            args.gas_aggregator_address,
            new_provider(args.rpc_url.as_str(), &args.private_key).await,
        )
        .aggregateTokensUsed()
        .send()
        .await
        {
            Ok(tx) => {
                println!("Gas aggregation succeeded: {}", tx.tx_hash());
            }
            Err(e) => {
                println!("Error aggregating gas");
                println!("--------------------------------");
                println!("{}", e);
                println!("--------------------------------");
            }
        }
    }
}
