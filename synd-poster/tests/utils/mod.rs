//! Utils for the poster integration tests

use alloy::{
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    providers::ProviderBuilder,
    signers::{
        k256::ecdsa::SigningKey,
        local::{LocalSigner, PrivateKeySigner},
    },
};
use eyre::Result;
use shared::types::FilledProvider;
use std::str::FromStr;

pub(crate) const DEFAULT_PRIVATE_KEY_SIGNER: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"; // address = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266

/// Starts local anvil node for testing
pub(crate) async fn start_anvil(
    chain_id: u64,
    port: u16,
) -> Result<(AnvilInstance, FilledProvider)> {
    let cmd = vec!["--base-fee", "0", "--gas-limit", "30000000", "--timestamp", "0"];
    let anvil = Anvil::new().port(port).chain_id(chain_id).args(cmd).try_spawn()?;

    let provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(get_default_private_key_signer()))
        .connect(&anvil.ws_endpoint())
        .await?;
    Ok((anvil, provider))
}

/// Parse default string into a `PrivateKeySigner`.
fn get_default_private_key_signer() -> LocalSigner<SigningKey> {
    PrivateKeySigner::from_str(DEFAULT_PRIVATE_KEY_SIGNER)
        .unwrap_or_else(|err| panic!("Failed to parse default private key for signer: {}", err))
}
