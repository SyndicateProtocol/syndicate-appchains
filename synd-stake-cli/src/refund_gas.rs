//! The `refund-gas` module contains the functions for refunding gas from the refunder contract.

use alloy::{
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
};
use clap::Args;
use contract_bindings::synd::refunder::Refunder;
use shared::{parse::parse_address, types::new_provider};

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

    /// The private key to use for the transaction.
    #[arg(short = 'k', long, env = "PRIVATE_KEY")]
    pub private_key: String,

    /// The address of the refunder contract.
    #[arg(
        short = 'a',
        long,
        env = "REFUNDER_ADDRESS",
        value_parser = parse_address,
    )]
    pub refunder_address: Address,

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
    let provider = ProviderBuilder::new()
        .connect(args.rpc_url.as_str())
        .await
        .unwrap_or_else(|e| panic!("Failed to connect to RPC URL '{}': {}", args.rpc_url, e));

    if provider.get_balance(args.refunder_address).await.unwrap_or_else(|e| {
        panic!("Failed to get balance for refunder address '{}': {}", args.refunder_address, e)
    }) == U256::from(0)
    {
        println!("No excess gas to refund");
        return;
    }

    if args.sim {
        println!("Simulating refund gas...");
        match Refunder::new(args.refunder_address, provider).recover().call().await {
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
        println!("Refunding gas...");
        match Refunder::new(
            args.refunder_address,
            new_provider(args.rpc_url.as_str(), &args.private_key).await,
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
