//! The `refund-gas` module contains the functions for refunding gas from the refunder contract.

use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
};
use clap::Args;
use contract_bindings::synd::refunder::Refunder;
use std::str::FromStr;

/// Arguments for the `refund-gas` command.
///
/// This struct defines the command-line arguments that can be passed to the refund-gas command.
/// The refund-gas command is used to refund excess gas from the bridging of the emissions
/// to commons chain.
#[derive(Args, Debug)]
pub struct RefundGasArgs {
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
        env = "REFUNDER_ADDRESS",
        default_value = "0x0000000000000000000000000000000000000000"
    )]
    pub refunder_address: String,

    /// The RPC URL to use for the transaction.
    #[arg(short = 'r', long, env = "RPC_URL", default_value = "https://commons.rpc.syndicate.io")]
    pub rpc_url: String,
}

/// Refunds gas from the refunder contract.
///
/// This function processes and submits the refund transaction for the refunder contract.
/// The refunder contract is used to refund excess gas from the bridging of the emissions
/// to commons chain.
///
/// # Arguments
///
/// * `args` - The command-line arguments containing configuration options
///
/// # Examples
///
/// ```bash
/// # Run a normal emission
/// synd-stake-cli refund-gas -k <private_key>
///
/// # Run in simulation mode
/// synd-stake-cli refund-gas --sim
/// ```
///
/// # Errors
///
/// This function may return an error if:
/// - The transaction/simulation fails
pub async fn refund_gas(args: &RefundGasArgs) {
    let refunder_address = Address::from_str(args.refunder_address.as_str()).unwrap();
    let provider = ProviderBuilder::new().connect(args.rpc_url.as_str()).await.unwrap();

    if provider.get_balance(refunder_address).await.unwrap() == U256::from(0) {
        println!("No excess gas to refund");
        return;
    }

    if args.sim {
        println!("Simulating refund gas...");
        match Refunder::new(refunder_address, provider).recover().call().await {
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
        if args.private_key.is_none() {
            println!(
                "Private key is required for refunding gas. Use the -k flag to provide the private key."
            );
            return;
        }
        println!("Refunding gas...");
        match Refunder::new(
            refunder_address,
            ProviderBuilder::new()
                .wallet(EthereumWallet::from(
                    PrivateKeySigner::from_str(args.private_key.as_ref().unwrap()).unwrap(),
                ))
                .connect(args.rpc_url.as_str())
                .await
                .unwrap(),
        )
        .recover()
        .send()
        .await
        {
            Ok(tx) => {
                println!("Refund succeeded: {}", tx.tx_hash());
            }
            Err(e) => {
                println!("Error refunding gas");
                println!("--------------------------------");
                println!("{}", e);
                println!("--------------------------------");
            }
        }
    }
}
