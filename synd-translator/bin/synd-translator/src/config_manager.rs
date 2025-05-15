use crate::config::TranslatorConfig;
use alloy::{
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::client::ClientBuilder,
};
use contract_bindings::synd::{arbchainconfig, arbconfigmanager::ArbConfigManager};
use eyre::Result;
use synd_chain_ingestor::client::{IngestorProvider, Provider as _};
use tracing::{error, info, warn};

pub async fn with_onchain_config(config: &TranslatorConfig) -> TranslatorConfig {
    let address = match config.config_manager_address {
        Some(addr) => addr,
        None => {
            warn!("No config_manager_address provided, skipping on-chain config fetch");
            return config.clone();
        }
    };

    let ingestor_provider =
        IngestorProvider::new(&config.settlement.settlement_rpc_url, config.rpc_timeout).await;

    let provider = ProviderBuilder::new().on_client(
        ClientBuilder::default()
            .connect(ingestor_provider.get_url().await.unwrap().as_ref())
            .await
            .unwrap(),
    );

    let onchain = match get_config(address, U256::from(config.appchain_chain_id), provider).await {
        Ok(c) => c,
        Err(error) => {
            error!(%error, "error obtaining on-chain config");
            return config.clone()
        }
    };

    let mut config = config.clone();

    // Update config with onchain values
    if config.block_builder.arbitrum_bridge_address.is_none() {
        info!(
            "Using the arbitrum_bridge_address from on-chain config: {:?}",
            onchain.arbitrum_bridge_address
        );
        config.block_builder.arbitrum_bridge_address = Some(onchain.arbitrum_bridge_address);
    };

    if config.block_builder.arbitrum_inbox_address.is_none() {
        info!(
            "Using the arbitrum_inbox_address from on-chain config: {:?}",
            onchain.arbitrum_inbox_address
        );
        config.block_builder.arbitrum_inbox_address = Some(onchain.arbitrum_inbox_address);
    };

    if config.block_builder.arbitrum_ignore_delayed_messages.is_none() {
        info!(
            "Using the arbitrum_ignore_delayed_messages from on-chain config: {}",
            onchain.arbitrum_ignore_delayed_messages
        );
        config.block_builder.arbitrum_ignore_delayed_messages =
            Some(onchain.arbitrum_ignore_delayed_messages);
    };

    if config.settlement_delay.is_none() {
        info!("Using the settlement_delay from on-chain config: {}", onchain.settlement_delay);
        config.settlement_delay = Some(onchain.settlement_delay.try_into().unwrap());
    }

    if config.settlement.settlement_start_block.is_none() {
        info!(
            "Using the settlement_start_block from on-chain config: {}",
            onchain.settlement_start_block
        );
        config.settlement.settlement_start_block =
            Some(onchain.settlement_start_block.try_into().unwrap());
    }

    if config.sequencing.sequencing_start_block.is_none() {
        info!(
            "Using the sequencing_start_block from on-chain config: {}",
            onchain.sequencing_start_block
        );
        config.sequencing.sequencing_start_block =
            Some(onchain.sequencing_start_block.try_into().unwrap());
    }

    if config.block_builder.sequencing_contract_address.is_none() {
        info!(
            "Using the sequencing_contract_address from on-chain config: {:?}",
            onchain.sequencing_contract_address
        );
        config.block_builder.sequencing_contract_address =
            Some(onchain.sequencing_contract_address);
    }

    if config.sequencing.sequencing_rpc_url.is_none() {
        info!(
            "Using the sequencing_rpc_url from on-chain config: {}",
            onchain.default_sequencing_chain_rpc_url
        );
        config.sequencing.sequencing_rpc_url = Some(onchain.default_sequencing_chain_rpc_url)
    }

    if config.block_builder.allowed_settlement_addresses.is_empty() {
        info!(
            "Using the allowed_settlement_addresses from on-chain config: {:?}",
            onchain.allowed_settlement_addresses
        );
        config.block_builder.allowed_settlement_addresses = onchain.allowed_settlement_addresses;
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
    let allowed_settlement_addresses_call =
        arb_chain_config_contract.getAllowedSettlementAddresses();

    let (
        arbitrum_bridge_address,
        arbitrum_inbox_address,
        arbitrum_ignore_delayed_messages,
        settlement_delay,
        settlement_start_block,
        sequencing_start_block,
        sequencing_contract_address,
        default_sequencing_chain_rpc_url,
        allowed_settlement_addresses,
    ) = tokio::try_join!(
        arbitrum_bridge_address_call.call(),
        arbitrum_inbox_address_call.call(),
        arbitrum_ignore_delayed_messages_call.call(),
        settlement_delay_call.call(),
        settlement_start_block_call.call(),
        sequencing_start_block_call.call(),
        sequencing_contract_address_call.call(),
        default_sequencing_chain_rpc_url_call.call(),
        allowed_settlement_addresses_call.call(),
    )?;

    Ok(ChainConfig {
        arbitrum_bridge_address: arbitrum_bridge_address._0,
        arbitrum_inbox_address: arbitrum_inbox_address._0,
        arbitrum_ignore_delayed_messages: arbitrum_ignore_delayed_messages._0,
        settlement_delay: settlement_delay._0,
        settlement_start_block: settlement_start_block._0,
        sequencing_start_block: sequencing_start_block._0,
        sequencing_contract_address: sequencing_contract_address._0,
        default_sequencing_chain_rpc_url: default_sequencing_chain_rpc_url._0,
        allowed_settlement_addresses: allowed_settlement_addresses._0,
    })
}

/// representation of the chain configuration that lives on-chain
#[allow(missing_docs)]
struct ChainConfig {
    arbitrum_bridge_address: Address,
    arbitrum_inbox_address: Address,
    arbitrum_ignore_delayed_messages: bool,
    settlement_delay: U256,
    settlement_start_block: U256,
    sequencing_start_block: U256,
    sequencing_contract_address: Address,
    default_sequencing_chain_rpc_url: String,
    allowed_settlement_addresses: Vec<Address>,
}
