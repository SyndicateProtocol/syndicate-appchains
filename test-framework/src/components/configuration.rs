#![allow(missing_docs)]

use crate::components::test_components::SEQUENCING_CHAIN_ID;
use alloy::{
    primitives::{Address, U256},
    providers::WalletProvider,
};
use contract_bindings::synd::{
    arb_chain_config::ArbChainConfig::{self, ArbChainConfigInstance},
    arb_config_manager::ArbConfigManager,
};
use eyre::Result;
use shared::types::FilledProvider;
use std::time::Duration;
use synd_migration_cli::migration::RollupState;
use test_utils::{anvil::mine_block, preloaded_config::ContractVersion};
use tracing::info;

/// The base chains type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseChainsType {
    Anvil,
    PreLoaded(ContractVersion),
    Nitro,            // auto-mine enabled
    NitroWithEigenda, // auto-mine enabled
}

/// Arbitrum Nitro contract version on the settlement chain used for testing
#[derive(Debug, Clone)]
#[allow(clippy::redundant_pub_crate)]
pub struct ConfigurationOptions {
    pub base_chains_type: BaseChainsType,
    pub use_write_loop: bool,
    pub sequencing_start_block: u64,
    pub settlement_start_block: u64,
    pub settlement_delay: u64,
    pub rollup_owner: Address,
    pub appchain_chain_id: u64,
    pub finality_delay: u64,
    pub maestro_finalization_duration: Option<Duration>,
    pub maestro_finalization_checker_interval: Option<Duration>,
    pub close_challenge_interval: Duration,
}

impl Default for ConfigurationOptions {
    fn default() -> Self {
        Self {
            base_chains_type: BaseChainsType::Anvil,
            // Spins up write loop components
            use_write_loop: false,
            // skip the genesis block
            sequencing_start_block: 1,
            // skip the genesis block and the 2 blocks used to deploy the config manager
            settlement_start_block: 1 + 2,
            settlement_delay: 0,
            rollup_owner: Address::ZERO,
            appchain_chain_id: 13331370,
            finality_delay: 60,
            maestro_finalization_duration: None,
            maestro_finalization_checker_interval: None,
            close_challenge_interval: Duration::from_secs(1),
        }
    }
}

/// Sets up the config manager and creates the chain configuration
#[allow(clippy::unwrap_used, clippy::too_many_arguments)]
pub async fn setup_config_manager(
    set_provider: &FilledProvider,
    options: &ConfigurationOptions,
    sequencing_contract_address: Address,
    arbitrum_bridge_address: Address,
    arbitrum_inbox_address: Address,
    sequencing_rpc_url: String,
    appchain_block_explorer_url: String,
    migration_data: Option<RollupState>,
) -> Result<Address> {
    // Deploy config manager
    let config_manager_owner = set_provider.default_signer_address();
    let config_manager_tx =
        ArbConfigManager::deploy_builder(set_provider, config_manager_owner).send().await?;

    match options.base_chains_type {
        BaseChainsType::Anvil | BaseChainsType::PreLoaded(_) => {
            mine_block(set_provider, 0).await?;
        }
        _ => {}
    };

    let config_manager_address = config_manager_tx.get_receipt().await?.contract_address.unwrap();
    let config_manager = ArbConfigManager::new(config_manager_address, set_provider.clone());

    let create_chain_config_tx = config_manager
        .createArbChainConfig(
            config_manager_owner,
            options.appchain_chain_id.try_into().unwrap(),
            U256::from(SEQUENCING_CHAIN_ID),
            arbitrum_bridge_address,
            arbitrum_inbox_address,
            options.settlement_delay.try_into().unwrap(),
            options.settlement_start_block.try_into().unwrap(),
            sequencing_contract_address,
            options.sequencing_start_block.try_into().unwrap(),
            options.rollup_owner,
            sequencing_rpc_url,
            appchain_block_explorer_url,
        )
        .send()
        .await?;

    let config_address = config_manager
        .getArbChainConfigAddress(options.appchain_chain_id.try_into().unwrap())
        .call()
        .await?;
    let config = ArbChainConfig::new(config_address, set_provider.clone());

    if let Some(migration_data) = migration_data {
        chain_config_migration(&config, options, &migration_data).await?;
    }

    match options.base_chains_type {
        BaseChainsType::Anvil | BaseChainsType::PreLoaded(_) => {
            mine_block(set_provider, 0).await?;
        }
        _ => {}
    };

    assert!(create_chain_config_tx.get_receipt().await?.status());
    Ok(config_manager.address().to_owned())
}

async fn chain_config_migration(
    config: &ArbChainConfigInstance<FilledProvider>,
    options: &ConfigurationOptions,
    migration_data: &RollupState,
) -> Result<()> {
    info!("Migrating chain config with data: {:#?}", migration_data);
    let receipt = config
        .migration(
            migration_data.parent_chain_block.try_into()?,
            options.sequencing_start_block.try_into()?,
            migration_data.batch_acc.into(),
            migration_data.batch_count.try_into()?,
            migration_data.delayed_msgs_acc.into(),
            migration_data.delayed_msgs_count.try_into()?,
            migration_data.last_block_hash.into(),
        )
        .send()
        .await?
        .get_receipt()
        .await?;
    assert!(receipt.status());
    info!("Migration transaction receipt: {:#?}", receipt);
    Ok(())
}
