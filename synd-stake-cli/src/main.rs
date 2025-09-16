//! The `synd-stake-cli` is a service for running permissionless actions in the staking system.

use clap::{Parser, Subcommand};
use synd_stake_cli::{
    gas_agg::{gas_agg, GasAggArgs},
    mint::{mint, MintArgs},
    refund_gas::{refund_gas, RefundGasArgs},
};
use tokio;

#[derive(Parser, Debug)]
#[command(version, about)]
#[allow(missing_docs)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Mint rewards to the staking system.
    ///
    /// This command triggers the emission of rewards to stakers based on their
    /// stake and participation in the network. Use the --sim flag to run in
    /// simulation mode without actually submitting transactions.
    Mint(MintArgs),

    /// Refund excess gas from bridging
    ///
    /// This command refunds excess gas from bridging emissions to commons chain.
    RefundGas(RefundGasArgs),

    /// Aggregate gas usage of appchains
    ///
    /// This command aggregates gas usage data from all appchains registered in the factory.
    GasAgg(GasAggArgs),

    /// Ethereum and Base block hashes to Commons Chain
    ///
    /// TODO: Implement
    Relay,

    /// Confirm Sequencing Chain block hash from Ethereum block hash
    ///
    /// TODO: Implement
    ConfirmSeq,

    /// Confirm gas data hash from Sequencing Chain block hash
    ///
    /// TODO: Implement
    ConfirmGas,

    /// Submit gas data to Commons Chain
    ///
    /// TODO: Implement
    SubmitGas,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Mint(mint_args)) => {
            mint(mint_args).await;
        }
        Some(Commands::RefundGas(refund_gas_args)) => {
            refund_gas(refund_gas_args).await;
        }
        Some(Commands::GasAgg(gas_agg_args)) => {
            gas_agg(gas_agg_args).await;
        }
        Some(Commands::Relay) => {
            println!("TODO: Implement relay command");
        }
        Some(Commands::ConfirmSeq) => {
            println!("TODO: Implement confirm sequence command");
        }
        Some(Commands::ConfirmGas) => {
            println!("TODO: Implement confirm gas command");
        }
        Some(Commands::SubmitGas) => {
            println!("TODO: Implement submit gas command");
        }
        None => {
            println!("No command provided. Use --help for usage.");
        }
    }
}
