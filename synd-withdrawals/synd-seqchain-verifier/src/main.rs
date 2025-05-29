//! Main entrypoint for the `synd-seqchain-verifier`

use clap::Parser;
use eyre::Result;
use serde::Serialize;
use shared::logger::set_global_default_subscriber;
use synd_seqchain_verifier::{
    config::SeqchainVerifierConfig,
    types::{BlockVerifierInput, L1ChainInput},
    verifier::Verifier,
};
use tracing::debug;

#[derive(Serialize)]
struct OutputWrapper {
    verify_seqchain_output: Vec<BlockVerifierInput>,
}

#[derive(Parser, Debug)]
struct VerifierCliArgs {
    /// Config
    #[arg(long)]
    config: String,

    /// L1 chain input
    #[arg(long)]
    l1_chain_input: String,
}

#[allow(clippy::unwrap_used)]
fn main() {
    match run() {
        Ok(outputs) => {
            // Print raw JSON to stdout
            println!("Outputs created successfully");
            println!(
                "{}",
                serde_json::to_string(&OutputWrapper { verify_seqchain_output: outputs }).unwrap()
            );
        }
        Err(e) => {
            debug!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<Vec<BlockVerifierInput>> {
    set_global_default_subscriber()?;

    let args = VerifierCliArgs::parse();
    debug!("VerifierCliArgs: {:?}", args);

    let l1_chain_input: L1ChainInput = serde_json::from_str(&args.l1_chain_input)?;
    debug!("L1 input: {:?}", l1_chain_input);

    let config: SeqchainVerifierConfig = serde_json::from_str(&args.config)?;

    let verifier = Verifier::new(&config);
    verifier
        .verify_and_create_output(&l1_chain_input)
        .map_err(|e| eyre::eyre!("Error verifying and creating output: {:?}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_input() {
        let config = SeqchainVerifierConfig::default();
        let config_json = serde_json::to_string(&config).unwrap();
        println!("{}", config_json);
        let l1_chain_input = L1ChainInput::default();
        let l1_chain_input_json = serde_json::to_string(&l1_chain_input).unwrap();
        println!("{}", l1_chain_input_json);
    }
}
