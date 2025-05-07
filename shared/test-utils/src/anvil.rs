//! Anvil components for the integration tests

use crate::port_manager::PortManager;
use alloy::{
    eips::BlockNumberOrTag,
    node_bindings::{Anvil, AnvilInstance},
    providers::{ext::AnvilApi as _, Provider, ProviderBuilder},
    rpc::types::{anvil::MineOptions, Block},
    signers::local::PrivateKeySigner,
};
use eyre::{eyre, Result};
use shared::types::FilledProvider;
use std::str::FromStr as _;

// anvil account 1
pub const PRIVATE_KEY: &str = "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";

pub async fn start_anvil(chain_id: u64) -> Result<(u16, AnvilInstance, FilledProvider)> {
    start_anvil_with_args(chain_id, Default::default()).await
}

pub async fn start_anvil_with_args(
    chain_id: u64,
    args: &[&str],
) -> Result<(u16, AnvilInstance, FilledProvider)> {
    let port = PortManager::instance().next_port().await;
    let mut cmd =
        vec!["--base-fee", "0", "--gas-limit", "30000000", "--timestamp", "0", "--no-mining"];
    cmd.extend_from_slice(args);
    let anvil = Anvil::new().port(port).chain_id(chain_id).args(cmd).try_spawn()?;

    let provider = ProviderBuilder::new()
        .wallet(PrivateKeySigner::from_str(PRIVATE_KEY).unwrap_or_else(|err| {
            panic!("Failed to parse default private key for signer: {}", err)
        }))
        .connect(&anvil.ws_endpoint())
        .await?;
    Ok((port, anvil, provider))
}

/// mine a block with a delay
pub async fn mine_block(provider: &FilledProvider, delay: u64) -> Result<()> {
    let block: Block = provider
        .get_block_by_number(BlockNumberOrTag::Latest)
        .await?
        .ok_or_else(|| eyre!("Block not found"))?;
    provider.evm_mine(Some(MineOptions::Timestamp(Some(block.header.timestamp + delay)))).await?;
    Ok(())
}
