//! Main entrypoint for the `synd-seqchain-verifier`

use alloy::primitives::B256;
use clap::Parser;
use eyre::Result;
use serde::{Deserialize, Serialize};
use shared::logger::set_global_default_subscriber;
use synd_seqchain_verifier::{
    config::SeqchainVerifierConfig,
    types::{parse_json, BlockVerifierInput, L1ChainInput},
    verifier::Verifier,
};
use tracing::{debug, error};

#[derive(Parser, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct VerifierCliArgs {
    /// Config
    #[arg(long, value_parser = |s: &str| parse_json::<SeqchainVerifierConfig>(s))]
    config: SeqchainVerifierConfig,

    /// L1 chain input
    #[arg(long, value_parser = |s: &str| parse_json::<L1ChainInput>(s))]
    l1_chain_input: L1ChainInput,

    /// Config hash
    #[arg(long)]
    seq_config_hash: B256,
}

#[allow(clippy::unwrap_used)]
fn main() {
    match run() {
        Ok(outputs) => {
            // Print raw JSON to stdout
            println!("Outputs created successfully");
            println!("{}", serde_json::to_string(&outputs).unwrap());
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

    // Verify config hash matches config
    let expected_config_hash = args.config.hash_verifier_config_sha256();
    if args.seq_config_hash != expected_config_hash {
        let err_msg = format!(
            "Config hash mismatch: Got {:?}, Expected {:?}",
            args.seq_config_hash, expected_config_hash
        );
        error!("{}", err_msg);
        return Err(eyre::eyre!(err_msg));
    }

    let verifier = Verifier::new(&args.config);
    verifier
        .verify_and_create_output(&args.l1_chain_input)
        .map_err(|e| eyre::eyre!("Error verifying and creating output: {:?}", e))
    // Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Address;

    #[test]
    fn test_generate_input() {
        let config = SeqchainVerifierConfig { arbitrum_bridge_address: Address::ZERO };
        let config_json = serde_json::to_string(&config).unwrap();
        println!("{}", config_json);
        let l1_chain_input = L1ChainInput::default();
        let l1_chain_input_json = serde_json::to_string(&l1_chain_input).unwrap();
        println!("{}", l1_chain_input_json);
    }
}
