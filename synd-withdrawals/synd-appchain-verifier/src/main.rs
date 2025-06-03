//! Main entrypoint for the `synd-appchain-verifier`

use clap::Parser;
use eyre::Result;
use shared::logger::set_global_default_subscriber;
use synd_appchain_verifier::{
    config::AppchainVerifierConfig,
    types::{BlockVerifierInput, SequencingChainInput, SettlementChainInput},
    verifier::Verifier,
};
use tracing::{debug, info};

#[derive(Parser, Debug)]
struct VerifierCliArgs {
    /// Config
    #[arg(long)]
    config: String,

    /// Sequencing chain input
    #[arg(long)]
    sequencing_chain_input: String,

    /// Settlement chain input
    #[arg(long)]
    settlement_chain_input: String,

    /// Config hash
    #[arg(long)]
    appchain_config_hash: String,
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
    info!("Starting Appchain Verifier");

    let args = VerifierCliArgs::parse();

    let config: AppchainVerifierConfig = serde_json::from_str(&args.config)?;

    let settlement_chain_input: SettlementChainInput =
        serde_json::from_str(&args.settlement_chain_input)?;

    let sequencing_chain_input: SequencingChainInput =
        serde_json::from_str(&args.sequencing_chain_input)?;

    // Verify config hash matches config
    if args.appchain_config_hash != config.hash_verifier_config_sha256().to_string() {
        return Err(eyre::eyre!(
            "Config hash mismatch: Got {:?}, Expected {:?}",
            args.appchain_config_hash,
            config.hash_verifier_config_sha256()
        ));
    }

    let verifier = Verifier::new(&config);
    verifier
        .verify_and_create_output(&sequencing_chain_input, &settlement_chain_input)
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
