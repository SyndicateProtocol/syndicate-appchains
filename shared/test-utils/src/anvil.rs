//! Anvil components for the integration tests

use crate::{
    chain_info::{test_account1, ChainInfo, ProcessInstance},
    port_manager::PortManager,
};
use alloy::{
    eips::BlockNumberOrTag,
    node_bindings::Anvil,
    providers::{ext::AnvilApi as _, Provider, ProviderBuilder},
    rpc::types::{anvil::MineOptions, Block},
};
use eyre::{eyre, Result};
use shared::types::FilledProvider;

pub async fn start_anvil(chain_id: u64) -> Result<ChainInfo> {
    start_anvil_with_args(chain_id, Default::default()).await
}

pub async fn start_anvil_with_args(chain_id: u64, args: &[&str]) -> Result<ChainInfo> {
    let port = PortManager::instance().next_port().await;
    let mut cmd =
        vec!["--base-fee", "0", "--gas-limit", "30000000", "--timestamp", "0", "--no-mining"];
    cmd.extend_from_slice(args);
    let anvil = Anvil::new().port(port).chain_id(chain_id).args(cmd).try_spawn()?;

    let provider = ProviderBuilder::new()
        .wallet(test_account1().signer.clone())
        .connect(&anvil.ws_endpoint())
        .await?;
    Ok(ChainInfo {
        ws_url: format!("ws://localhost:{}", port),
        http_url: format!("http://localhost:{}", port),
        instance: ProcessInstance::Anvil(anvil),
        provider,
    })
}

/// mine a block with a delay
pub async fn mine_block(provider: &FilledProvider, delay: u64) -> Result<()> {
    let block: Block = provider
        .get_block_by_number(BlockNumberOrTag::Latest)
        .await?
        .ok_or_else(|| eyre!("Block not found"))?;
    provider
        .evm_mine(Some(MineOptions::Timestamp(Some(block.header.timestamp + delay))))
        .await
        .unwrap();
    Ok(())
}
