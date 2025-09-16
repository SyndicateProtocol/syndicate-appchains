//! The `mint` module contains the functions for minting emissions.

use alloy::{
    network::EthereumWallet,
    primitives::{utils::format_ether, Address},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use clap::Args;
use contract_bindings::synd::{
    emissions_calculator::EmissionsCalculator, emissions_scheduler::EmissionsScheduler,
};
use std::str::FromStr;

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
    pub private_key: Option<String>,

    /// The address to mint the emissions to.
    #[arg(
        short = 'a',
        long,
        env = "EMISSIONS_ADDRESS",
        default_value = "0x0000000000000000000000000000000000000000"
    )]
    pub emissions_address: String,

    /// The RPC URL to use for the transaction.
    #[arg(short = 'r', long, env = "RPC_URL", default_value = "https://eth.drpc.org")]
    pub rpc_url: String,
}

/// Mints emissions to the staking system.
///
/// This function processes and submits the mint transaction for an epochs rewards.
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
pub async fn mint(args: &MintArgs) {
    let emissions_address = Address::from_str(args.emissions_address.as_str()).unwrap();

    if args.sim {
        println!("Simulating mint...");
        let provider = ProviderBuilder::new().connect(args.rpc_url.as_str()).await.unwrap();
        let emissions_scheduler = EmissionsScheduler::new(emissions_address, &provider);

        match emissions_scheduler.mintEmission().call().await {
            Ok(_) => {
                println!("Simulation succeeded!");

                if let Ok(next_emission) = EmissionsCalculator::new(
                    emissions_scheduler.emissionsCalculator().call().await.unwrap(),
                    &provider,
                )
                .getNextEmission()
                .call()
                .await
                {
                    println!(
                        "Transaction would mint: ${:.2} SYND",
                        format_ether(next_emission).parse::<f64>().unwrap()
                    );
                }
            }
            Err(e) => {
                println!("Simulation failed");
                println!("--------------------------------");
                println!("{}", e);
                println!("--------------------------------");
            }
        }
    } else {
        if args.private_key.is_none() {
            println!(
                "Private key is required for minting. Use the -k flag to provide the private key."
            );
            return;
        }
        println!("Minting emissions...");
        match EmissionsScheduler::new(
            emissions_address,
            ProviderBuilder::new()
                .wallet(EthereumWallet::from(
                    PrivateKeySigner::from_str(args.private_key.as_ref().unwrap()).unwrap(),
                ))
                .connect(args.rpc_url.as_str())
                .await
                .unwrap(),
        )
        .mintEmission()
        .send()
        .await
        {
            Ok(tx) => {
                println!("Minting succeeded: {}", tx.tx_hash());
            }
            Err(e) => {
                println!("Error minting");
                println!("--------------------------------");
                println!("{}", e);
                println!("--------------------------------");
            }
        }
    }
}
