//! The `mint` module contains the functions for minting emissions.

use alloy::{
    primitives::{utils::format_ether, Address},
    providers::ProviderBuilder,
};
use clap::Args;
use contract_bindings::synd::{
    emissions_calculator::EmissionsCalculator, emissions_scheduler::EmissionsScheduler,
};
use shared::{
    parse::{parse_address, parse_url},
    types::new_provider,
};
use tracing::{error, info};

/// Arguments for the `mint` command.
///
/// This struct defines the command-line arguments that can be passed to the mint command.
/// The mint command is used to trigger emissions in the staking system.
#[derive(Args, Debug)]
pub struct MintArgs {
    /// Run in simulation mode without actually executing the emission.
    /// When enabled, the command will perform simulate the transaction.
    #[arg(short = 's', long, default_value_t = false)]
    pub sim: bool,

    /// The private key of the account to mint the emissions.
    #[arg(short = 'k', long, env = "PRIVATE_KEY")]
    pub private_key: String,

    /// The address to mint the emissions to.
    #[arg(
        short = 'a',
        long,
        env = "EMISSIONS_ADDRESS",
        value_parser = parse_address,
    )]
    pub emissions_address: Address,

    /// The RPC URL to use for the transaction.
    #[arg(short = 'r', long, env = "RPC_URL", default_value = "https://eth.drpc.org", value_parser = parse_url)]
    pub rpc_url: String,
}

/// Mints emissions to the staking system.
///
/// This function processes and submits the mint transaction for an epoch's rewards.
/// Emissions are used to distribute rewards to stakers based on their stake and
/// participation in the network.
///
/// # Arguments
///
/// * `args` - The command-line arguments containing configuration options
///
/// # Examples
///
/// ```bash
/// # Run a normal emission
/// synd-stake-cli mint
///
/// # Run in simulation mode
/// synd-stake-cli mint --sim
/// ```
///
/// # Errors
///
/// This function may return an error if:
/// - The transaction/simulation fails
#[allow(clippy::cognitive_complexity)]
pub async fn mint(args: &MintArgs) {
    if args.sim {
        info!("Simulating mint...");
        // TODO (ENG-2111): Use shared provider function
        let provider = ProviderBuilder::new()
            .connect(args.rpc_url.as_str())
            .await
            .unwrap_or_else(|e| panic!("Failed to connect to RPC URL '{}': {}", args.rpc_url, e));
        let emissions_scheduler = EmissionsScheduler::new(args.emissions_address, &provider);

        match emissions_scheduler.mintEmission().call().await {
            Ok(_) => {
                info!("Simulation succeeded!");

                if let Ok(next_emission) = EmissionsCalculator::new(
                    emissions_scheduler.emissionsCalculator().call().await.unwrap_or_else(|e| {
                        panic!("Failed to call emissionsCalculator on emissions scheduler contract: {e} ")
                    }),
                    &provider,
                )
                .getNextEmission()
                .call()
                .await
                {
                    info!(
                        "Transaction would mint: ${:.2} SYND",
                        format_ether(next_emission).parse::<f64>().unwrap_or_else(|e| {
                            panic!("Failed to parse emission amount as f64: {e} ")
                        })
                    );
                }
            }
            Err(e) => {
                error!("Simulation failed. Error: {}", e);
            }
        }
    } else {
        info!("Minting emissions...");
        match EmissionsScheduler::new(
            args.emissions_address,
            new_provider(&args.rpc_url, &args.private_key).await,
        )
        .mintEmission()
        .send()
        .await
        {
            Ok(tx) => {
                info!("Minting succeeded: {}", tx.tx_hash());
            }
            Err(e) => {
                error!("Error minting. Error: {}", e);
            }
        }
    }
}
