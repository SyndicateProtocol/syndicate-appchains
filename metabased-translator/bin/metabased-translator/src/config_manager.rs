use crate::config::MetabasedConfig;
use alloy::{
    primitives::{Address, U256},
    providers::Provider,
};
use contract_bindings::metabased::{arbchainconfig, arbconfigmanager::ArbConfigManager};
use eyre::Result;
use tracing::error;

pub async fn with_onchain_config<T: Provider + Clone>(
    config: &MetabasedConfig,
    provider: T,
) -> MetabasedConfig {
    let address = match config.config_manager_address {
        Some(addr) => addr,
        None => return config.clone(),
    };
    let chain_id = match config.appchain_chain_id {
        Some(chain_id) => chain_id,
        None => return config.clone(),
    };

    let onchain = match get_config(address, U256::from(chain_id), provider).await {
        Ok(c) => c,
        Err(error) => {
            error!(%error, "error obtaining on-chain config");
            return config.clone()
        }
    };

    let mut config = config.clone();

    // Update config with onchain values
    if config.block_builder.arbitrum_bridge_address.is_none() {
        config.block_builder.arbitrum_bridge_address = Some(onchain.arbitrum_bridge_address);
    };

    if config.block_builder.arbitrum_inbox_address.is_none() {
        config.block_builder.arbitrum_inbox_address = Some(onchain.arbitrum_inbox_address);
    };

    if config.block_builder.arbitrum_ignore_delayed_messages.is_none() {
        config.block_builder.arbitrum_ignore_delayed_messages =
            Some(onchain.arbitrum_ignore_delayed_messages);
    };

    if config.settlement_delay.is_none() {
        config.settlement_delay = Some(onchain.settlement_delay.try_into().unwrap());
    }

    if config.settlement.settlement_start_block.is_none() {
        config.settlement.settlement_start_block =
            Some(onchain.settlement_start_block.try_into().unwrap());
    }

    if config.sequencing.sequencing_start_block.is_none() {
        config.sequencing.sequencing_start_block =
            Some(onchain.sequencing_start_block.try_into().unwrap());
    }

    if config.block_builder.sequencing_contract_address.is_none() {
        config.block_builder.sequencing_contract_address =
            Some(onchain.sequencing_contract_address);
    }

    if config.sequencing.sequencing_rpc_url.is_none() {
        config.sequencing.sequencing_rpc_url = Some(onchain.default_sequencing_chain_rpc_url)
    }

    if config.rollup_owner.is_none() {
        config.rollup_owner = Some(onchain.rollup_owner);
    }

    config
}

async fn get_config<T: Provider + Clone>(
    address: Address,
    chain_id: U256,
    provider: T,
) -> Result<ChainConfig> {
    let config_manager_contract = ArbConfigManager::new(address, provider.clone());
    let config_address = config_manager_contract.getArbChainConfigAddress(chain_id).call().await?;
    let arb_chain_config_contract =
        arbchainconfig::ArbChainConfig::new(config_address._0, provider);

    let mine_empty_blocks_call = arb_chain_config_contract.MINE_EMPTY_BLOCKS();
    let arbitrum_bridge_address_call = arb_chain_config_contract.ARBITRUM_BRIDGE_ADDRESS();
    let arbitrum_inbox_address_call = arb_chain_config_contract.ARBITRUM_INBOX_ADDRESS();
    let arbitrum_ignore_delayed_messages_call =
        arb_chain_config_contract.ARBITRUM_IGNORE_DELAYED_MESSAGES();
    let settlement_delay_call = arb_chain_config_contract.SETTLEMENT_DELAY();
    let settlement_start_block_call = arb_chain_config_contract.SETTLEMENT_START_BLOCK();
    let sequencing_start_block_call = arb_chain_config_contract.SEQUENCING_START_BLOCK();
    let sequencing_contract_address_call = arb_chain_config_contract.SEQUENCING_CONTRACT_ADDRESS();
    let default_sequencing_chain_rpc_url_call =
        arb_chain_config_contract.DEFAULT_SEQUENCING_CHAIN_RPC_URL();
    let rollup_owner_call = arb_chain_config_contract.ROLLUP_OWNER();

    let (
        mine_empty_blocks,
        arbitrum_bridge_address,
        arbitrum_inbox_address,
        arbitrum_ignore_delayed_messages,
        settlement_delay,
        settlement_start_block,
        sequencing_start_block,
        sequencing_contract_address,
        default_sequencing_chain_rpc_url,
        rollup_owner,
    ) = tokio::try_join!(
        mine_empty_blocks_call.call(),
        arbitrum_bridge_address_call.call(),
        arbitrum_inbox_address_call.call(),
        arbitrum_ignore_delayed_messages_call.call(),
        settlement_delay_call.call(),
        settlement_start_block_call.call(),
        sequencing_start_block_call.call(),
        sequencing_contract_address_call.call(),
        default_sequencing_chain_rpc_url_call.call(),
        rollup_owner_call.call(),
    )?;

    Ok(ChainConfig {
        mine_empty_blocks: mine_empty_blocks._0,
        arbitrum_bridge_address: arbitrum_bridge_address._0,
        arbitrum_inbox_address: arbitrum_inbox_address._0,
        arbitrum_ignore_delayed_messages: arbitrum_ignore_delayed_messages._0,
        settlement_delay: settlement_delay._0,
        settlement_start_block: settlement_start_block._0,
        sequencing_start_block: sequencing_start_block._0,
        sequencing_contract_address: sequencing_contract_address._0,
        default_sequencing_chain_rpc_url: default_sequencing_chain_rpc_url._0,
        rollup_owner: rollup_owner._0,
    })
}

/// representation of the chain configuration that lives on-chain
#[allow(missing_docs)]
struct ChainConfig {
    mine_empty_blocks: bool, // TODO remove this from the contract
    arbitrum_bridge_address: Address,
    arbitrum_inbox_address: Address,
    arbitrum_ignore_delayed_messages: bool,
    settlement_delay: U256,
    settlement_start_block: U256,
    sequencing_start_block: U256,
    sequencing_contract_address: Address,
    default_sequencing_chain_rpc_url: String,
    rollup_owner: Address,
}
