//! Main entrypoint for the `synd-appchain-verifier`

use alloy::primitives::B256;
use clap::Parser;
use eyre::Result;
use serde::{Deserialize, Serialize};
use shared::logger::set_global_default_subscriber;
use synd_appchain_verifier::{
    config::AppchainVerifierConfig,
    types::{parse_json, BlockVerifierInput, SequencingChainInput, SettlementChainInput},
    verifier::Verifier,
};
use tracing::{debug, error, info};

#[derive(Parser, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct VerifierCliArgs {
    /// Config
    #[arg(long, value_parser = |s: &str| parse_json::<AppchainVerifierConfig>(s))]
    config: AppchainVerifierConfig,

    /// Sequencing chain input
    #[arg(long, value_parser = |s: &str| parse_json::<SequencingChainInput>(s))]
    sequencing_chain_input: SequencingChainInput,

    /// Settlement chain input
    #[arg(long, value_parser = |s: &str| parse_json::<SettlementChainInput>(s))]
    settlement_chain_input: SettlementChainInput,

    /// Config hash
    #[arg(long)]
    appchain_config_hash: B256,
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
    info!("Starting Appchain Verifier. Parsing inputs...");

    let args = VerifierCliArgs::parse();
    info!("Verifier CLI Args: {:?}", args);

    // Verify config hash matches config
    if args.appchain_config_hash != args.config.hash_verifier_config_sha256() {
        let err_msg = format!(
            "Config hash mismatch: Got {:?}, Expected {:?}",
            args.appchain_config_hash,
            args.config.hash_verifier_config_sha256()
        );
        error!("{}", err_msg);
        return Err(eyre::eyre!(err_msg));
    }

    let verifier = Verifier::new(&args.config);
    verifier
        .verify_and_create_output(&args.sequencing_chain_input, &args.settlement_chain_input)
        .map_err(|e| eyre::eyre!("Error verifying and creating output: {:?}", e))
    // Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_input() {
        let config = AppchainVerifierConfig::default();
        let config_json = serde_json::to_string(&config).unwrap();
        println!("{}", config_json);
        let sequencing_chain_input = SequencingChainInput::default();
        let sequencing_chain_input_json = serde_json::to_string(&sequencing_chain_input).unwrap();
        println!("{}", sequencing_chain_input_json);
        let settlement_chain_input = SettlementChainInput::default();
        let settlement_chain_input_json = serde_json::to_string(&settlement_chain_input).unwrap();
        println!("{}", settlement_chain_input_json);
        let appchain_config_hash = config.hash_verifier_config_sha256();
        println!("{}", appchain_config_hash);
    }
}
