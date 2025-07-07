use crate::config::TranslatorConfig;
use alloy::{
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::client::{ClientBuilder, RpcClient},
};
use contract_bindings::synd::{
    arb_chain_config::ArbChainConfig, arb_config_manager::ArbConfigManager,
};
use eyre::Result;
use synd_chain_ingestor::client::{IngestorProvider, Provider as _};
use tracing::{error, info, warn};

async fn rpc_client_from_urls(urls: &[String]) -> RpcClient {
    for url in urls {
        match ClientBuilder::default().connect(url.as_str()).await {
            Ok(client) => {
                return client;
            }
            Err(e) => {
                warn!("Failed to connect to {}: {}", url, e);
            }
        }
    }
    panic!("Failed to connect to any of the provided URLs");
}

/// Overrides the config with onchain values where the config values are None
fn override_with_onchain_config(
    mut config: TranslatorConfig,
    onchain: &ChainConfig,
) -> TranslatorConfig {
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

    if config.sequencing.sequencing_ws_url.is_none() {
        info!(
            "Using the sequencing_ws_rpc_url from on-chain config: {}",
            onchain.default_sequencing_chain_ws_rpc_url
        );
        config.sequencing.sequencing_ws_url =
            Some(onchain.default_sequencing_chain_ws_rpc_url.clone())
    }

    config
}

/// Fetches chain config if it exists
pub async fn with_onchain_config(config: &TranslatorConfig) -> TranslatorConfig {
    let config = config.clone();
    let address = match config.config_manager_address {
        Some(addr) => addr,
        None => {
            warn!("No config_manager_address provided, skipping onchain config fetch");
            return config;
        }
    };

    let ingestor_provider =
        IngestorProvider::new(&config.settlement.settlement_ws_url, config.ws_request_timeout)
            .await;

    let urls = ingestor_provider.get_urls().await.unwrap();
    let client = rpc_client_from_urls(&urls).await;
    let provider = ProviderBuilder::new().connect_client(client);

    let onchain = match get_config(address, U256::from(config.appchain_chain_id), provider).await {
        Ok(c) => c,
        Err(error) => {
            error!(%error, "error obtaining onchain config");
            return config;
        }
    };

    override_with_onchain_config(config, &onchain)
}

async fn get_config<T: Provider + Clone>(
    address: Address,
    chain_id: U256,
    provider: T,
) -> Result<ChainConfig> {
    let config_manager_contract = ArbConfigManager::new(address, provider.clone());
    let config_address = config_manager_contract.getArbChainConfigAddress(chain_id).call().await?;

    let arb_chain_config_contract = ArbChainConfig::new(config_address, provider);

    let arbitrum_bridge_address_call = arb_chain_config_contract.ARBITRUM_BRIDGE_ADDRESS();
    let arbitrum_inbox_address_call = arb_chain_config_contract.ARBITRUM_INBOX_ADDRESS();
    let settlement_delay_call = arb_chain_config_contract.SETTLEMENT_DELAY();
    let settlement_start_block_call = arb_chain_config_contract.SETTLEMENT_START_BLOCK();
    let sequencing_start_block_call = arb_chain_config_contract.SEQUENCING_START_BLOCK();
    let sequencing_contract_address_call = arb_chain_config_contract.SEQUENCING_CONTRACT_ADDRESS();
    let default_sequencing_chain_ws_rpc_url_call =
        arb_chain_config_contract.DEFAULT_SEQUENCING_CHAIN_WS_RPC_URL();

    let (
        arbitrum_bridge_address,
        arbitrum_inbox_address,
        settlement_delay,
        settlement_start_block,
        sequencing_start_block,
        sequencing_contract_address,
        default_sequencing_chain_ws_rpc_url,
    ) = tokio::try_join!(
        arbitrum_bridge_address_call.call(),
        arbitrum_inbox_address_call.call(),
        settlement_delay_call.call(),
        settlement_start_block_call.call(),
        sequencing_start_block_call.call(),
        sequencing_contract_address_call.call(),
        default_sequencing_chain_ws_rpc_url_call.call(),
    )?;

    Ok(ChainConfig {
        arbitrum_bridge_address,
        arbitrum_inbox_address,
        settlement_delay,
        settlement_start_block,
        sequencing_start_block,
        sequencing_contract_address,
        default_sequencing_chain_ws_rpc_url,
    })
}

/// representation of the chain configuration that lives on-chain
#[allow(missing_docs)]
struct ChainConfig {
    arbitrum_bridge_address: Address,
    arbitrum_inbox_address: Address,
    settlement_delay: U256,
    settlement_start_block: U256,
    sequencing_start_block: U256,
    sequencing_contract_address: Address,
    default_sequencing_chain_ws_rpc_url: String,
}

#[cfg(test)]
mod test {
    use super::{override_with_onchain_config, ChainConfig, TranslatorConfig};
    use alloy::primitives::{address, U256};

    #[test]
    fn test_override_with_onchain_config_default() {
        let mut config = TranslatorConfig::default();

        // Create sample onchain config with known values
        let onchain = ChainConfig {
            arbitrum_bridge_address: address!("0x1111111111111111111111111111111111111111"),
            arbitrum_inbox_address: address!("0x2222222222222222222222222222222222222222"),
            settlement_delay: U256::from(30),
            settlement_start_block: U256::from(100),
            sequencing_start_block: U256::from(200),
            sequencing_contract_address: address!("0x3333333333333333333333333333333333333333"),
            default_sequencing_chain_ws_rpc_url: "wss://test-sequencing.example.com".to_string(),
        };

        // Apply overrides
        config = override_with_onchain_config(config, &onchain);

        // Assert all fields that should be overridden are correctly set
        assert_eq!(
            config.block_builder.arbitrum_bridge_address,
            Some(onchain.arbitrum_bridge_address)
        );
        assert_eq!(
            config.block_builder.arbitrum_inbox_address,
            Some(onchain.arbitrum_inbox_address)
        );
        assert_eq!(config.settlement_delay, Some(onchain.settlement_delay.try_into().unwrap()));
        assert_eq!(
            config.settlement.settlement_start_block,
            Some(onchain.settlement_start_block.try_into().unwrap())
        );
        assert_eq!(
            config.sequencing.sequencing_start_block,
            Some(onchain.sequencing_start_block.try_into().unwrap())
        );
        assert_eq!(
            config.block_builder.sequencing_contract_address,
            Some(onchain.sequencing_contract_address)
        );
        assert_eq!(
            config.sequencing.sequencing_ws_url,
            Some(onchain.default_sequencing_chain_ws_rpc_url)
        );
    }

    #[test]
    fn test_override_with_onchain_config_preserves_existing() {
        let mut config = TranslatorConfig::default();

        // Set some existing values that should NOT be overridden
        config.block_builder.arbitrum_bridge_address =
            Some(address!("0x9999999999999999999999999999999999999999"));
        config.settlement_delay = Some(999);

        // Create sample onchain config
        let onchain = ChainConfig {
            arbitrum_bridge_address: address!("0x1111111111111111111111111111111111111111"),
            arbitrum_inbox_address: address!("0x2222222222222222222222222222222222222222"),
            settlement_delay: U256::from(30),
            settlement_start_block: U256::from(100),
            sequencing_start_block: U256::from(200),
            sequencing_contract_address: address!("0x3333333333333333333333333333333333333333"),
            default_sequencing_chain_ws_rpc_url: "wss://test-sequencing.example.com".to_string(),
        };

        // Apply overrides
        config = override_with_onchain_config(config, &onchain);

        // Assert existing values are preserved
        assert_eq!(
            config.block_builder.arbitrum_bridge_address,
            Some(address!("0x9999999999999999999999999999999999999999"))
        );
        assert_eq!(config.settlement_delay, Some(999));

        // Assert None values are overridden
        assert_eq!(
            config.block_builder.arbitrum_inbox_address,
            Some(onchain.arbitrum_inbox_address)
        );
        assert_eq!(
            config.settlement.settlement_start_block,
            Some(onchain.settlement_start_block.try_into().unwrap())
        );
        assert_eq!(
            config.sequencing.sequencing_start_block,
            Some(onchain.sequencing_start_block.try_into().unwrap())
        );
        assert_eq!(
            config.block_builder.sequencing_contract_address,
            Some(onchain.sequencing_contract_address)
        );
        assert_eq!(
            config.sequencing.sequencing_ws_url,
            Some(onchain.default_sequencing_chain_ws_rpc_url)
        );
    }
}
