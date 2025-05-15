//! Main entrypoint for the synd-verifier
use alloy::primitives::FixedBytes;
use clap::Parser;
use eyre::Result;
use synd_mchain::db::MBlock;
use synd_verifier::{
    config::VerifierConfig, errors::VerifierError, types::ChainVerificationInput,
    verifier::Verifier,
};

#[derive(Parser)]
struct VerifierCliArgs {
    #[command(flatten)]
    config: VerifierConfig,

    /// Sequencing chain input
    #[arg(long)]
    sequencing_chain_input: String,

    /// Settlement chain input
    #[arg(long)]
    settlement_chain_input: String,

    #[arg(long)]
    sequencing_expected_hash: FixedBytes<32>,

    /// Expected hash for settlement chain (as 0x-prefixed hex)
    #[arg(long)]
    settlement_expected_hash: FixedBytes<32>,

    /// Settlement delay in seconds
    #[arg(long)]
    settlement_delay: u64,
}

#[allow(clippy::unwrap_used)]
fn main() {
    println!("Hello, verifier!");
    match run() {
        Ok(mblock) => println!("MBlock created successfully: {:?}", mblock),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<MBlock> {
    let args = VerifierCliArgs::parse();
    let sequencing_chain_input: ChainVerificationInput =
        serde_json::from_str(&args.sequencing_chain_input)?;
    let settlement_chain_input: ChainVerificationInput =
        serde_json::from_str(&args.settlement_chain_input)?;
    let verifier = Verifier::new(&args.config);
    let result = verifier.verify_and_create_batch(
        sequencing_chain_input,
        settlement_chain_input,
        args.sequencing_expected_hash,
        args.settlement_expected_hash,
        args.settlement_delay,
    )?;
    Ok(result)
}
