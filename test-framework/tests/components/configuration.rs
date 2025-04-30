use alloy::{primitives::Address, providers::WalletProvider};
use contract_bindings::metabased::arbconfigmanager::ArbConfigManager;
use eyre::Result;
use test_utils::anvil::{mine_block, FilledProvider};
/// nitro contract version on the settlement chain used for testing
#[derive(Debug, Clone)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) enum ContractVersion {
    V213,
    V300,
}

#[derive(Debug, Clone)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) struct ConfigurationOptions {
    pub(crate) pre_loaded: Option<ContractVersion>,
    pub(crate) sequencing_start_block: u64,
    pub(crate) settlement_start_block: u64,
    pub(crate) settlement_delay: u64,
    pub(crate) rollup_owner: Address,
    pub(crate) appchain_chain_id: u64,
    pub(crate) finality_delay: u64,
}

impl Default for ConfigurationOptions {
    fn default() -> Self {
        Self {
            pre_loaded: None,
            // skip the genesis block
            sequencing_start_block: 1,
            // skip the genesis block and the 2 blocks used to deploy the config manager
            settlement_start_block: 1 + 2,
            settlement_delay: 0,
            rollup_owner: Address::ZERO,
            appchain_chain_id: 13331370,
            finality_delay: 60,
        }
    }
}

/// Sets up the config manager and creates the chain configuration
#[allow(clippy::unwrap_used)]
pub(super) async fn setup_config_manager(
    set_provider: &FilledProvider,
    options: &ConfigurationOptions,
    sequencing_contract_address: Address,
    arbitrum_bridge_address: Address,
    arbitrum_inbox_address: Address,
    sequencing_rpc_url: &str,
    appchain_block_explorer_url: &str,
) -> Result<Address> {
    // Deploy config manager
    let config_manager_owner = set_provider.default_signer_address();
    let config_manager_tx =
        ArbConfigManager::deploy_builder(set_provider, config_manager_owner).send().await?;
    mine_block(set_provider, 0).await?;
    let config_manager_address = config_manager_tx.get_receipt().await?.contract_address.unwrap();
    let config_manager = ArbConfigManager::new(config_manager_address, set_provider.clone());

    let options_clone = options.clone();
    let sequencing_rpc_url_clone = sequencing_rpc_url.to_string();
    let appchain_block_explorer_url_clone = appchain_block_explorer_url.to_string();

    let create_chain_config_tx = config_manager
        .createArbChainConfig(
            config_manager_owner,
            options_clone.appchain_chain_id.try_into().unwrap(),
            options_clone.appchain_chain_id.try_into().unwrap(),
            arbitrum_bridge_address,
            arbitrum_inbox_address,
            false,
            options_clone.settlement_delay.try_into().unwrap(),
            options_clone.settlement_start_block.try_into().unwrap(),
            sequencing_contract_address,
            options_clone.sequencing_start_block.try_into().unwrap(),
            options_clone.rollup_owner,
            sequencing_rpc_url_clone,
            appchain_block_explorer_url_clone,
            vec![],
        )
        .send()
        .await?;
    mine_block(set_provider, 0).await?;

    assert!(create_chain_config_tx.get_receipt().await?.status());

    Ok(config_manager.address().to_owned())
}
