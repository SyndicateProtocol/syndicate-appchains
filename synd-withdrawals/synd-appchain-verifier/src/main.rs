//! Main entrypoint for the `synd-appchain-verifier`

use alloy::primitives::B256;
use clap::Parser;
use eyre::Result;
use shared::logger::set_global_default_subscriber;
use synd_appchain_verifier::{
    config::AppchainVerifierConfig,
    types::{SequencingChainInput, SettlementChainInput, VerifierOutput},
    verifier::Verifier,
};
use tracing::debug;

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
        Ok((outputs, unused_delayed_messages_hash)) => {
            debug!("Outputs created successfully:\n {}", serde_json::to_string(&outputs).unwrap());
            debug!("Unused delayed messages hash: {}", unused_delayed_messages_hash);
        }
        Err(e) => {
            debug!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<(Vec<VerifierOutput>, B256)> {
    // TODO (SEQ-769): Implement Appchain Verifier Component

    set_global_default_subscriber()?;

    let args = VerifierCliArgs::parse();
    debug!("VerifierCliArgs: {:?}", args);

    let sequencing_chain_input: SequencingChainInput =
        serde_json::from_str(&args.sequencing_chain_input)?;
    let settlement_chain_input: SettlementChainInput =
        serde_json::from_str(&args.settlement_chain_input)?;

    debug!("Sequencing chain input: {:?}", sequencing_chain_input);
    debug!("Settlement chain input: {:?}", settlement_chain_input);

    let config: AppchainVerifierConfig = serde_json::from_str(&args.config)?;

    // Verify config hash matches config
    if args.appchain_config_hash != config.hash_verifier_config_sha256().to_string() {
        return Err(eyre::eyre!("Config hash mismatch"));
    }

    let _verifier = Verifier::new(&config);
    // Ok(verifier.verify_and_create_output(&sequencing_chain_input, &settlement_chain_input)?)
    Ok((vec![], B256::ZERO))
}
