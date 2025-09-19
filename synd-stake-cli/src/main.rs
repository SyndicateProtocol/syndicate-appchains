//! The `synd-stake-cli` is a service for running permissionless actions in the staking system.

use clap::{Parser, Subcommand};
use synd_stake_cli::{
    gas_agg::{gas_agg, GasAggArgs},
    mint::{mint, MintArgs},
    proofs::{
        aggregate_gas_data_top_n_chains, submit_gas_proofs, update_base_and_ethereum_block_hashes,
        AggregateGasDataTopNChainsArgs, SubmitGasProofsArgs, UpdateBaseAndEthereumBlockHashesArgs,
    },
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

    /// call the BlockHashRelayer to update the base/ethereum block hashes on the staking appchain
    UpdateBaseAndEthereumBlockHashes(UpdateBaseAndEthereumBlockHashesArgs),

    /// call eth_getProof for the sequencing chain(s) and submit it to the staking appchain (twice
    /// 1 proof for block hash, another for the gasAggregation data)
    SubmitGasProofs(SubmitGasProofsArgs),

    /// submit the gas data for the top N chains, if we are over the "offchain aggregation"
    /// threshold
    AggregateGasDataTopNChains(AggregateGasDataTopNChainsArgs),
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match &args.command.unwrap_or_else(|| panic!("No command provided. use --help for more info")) {
        Commands::Mint(mint_args) => mint(mint_args).await,
        Commands::RefundGas(refund_gas_args) => refund_gas(refund_gas_args).await,
        Commands::GasAgg(gas_agg_args) => gas_agg(gas_agg_args).await,
        Commands::Relay => todo!(),
        Commands::ConfirmSeq => todo!(),
        Commands::ConfirmGas => todo!(),
        Commands::SubmitGas => todo!(),
        Commands::UpdateBaseAndEthereumBlockHashes(args) => {
            update_base_and_ethereum_block_hashes(args).await
        }
        Commands::SubmitGasProofs(args) => submit_gas_proofs(args).await,
        Commands::AggregateGasDataTopNChains(args) => aggregate_gas_data_top_n_chains(args).await,
    };
}
