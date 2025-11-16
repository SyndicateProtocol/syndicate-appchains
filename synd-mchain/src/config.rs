//! The `MockChain` is used for appchain block derivation.

use crate::db::MigrationParams;
use alloy::{
    primitives::{Address, B256, U256},
    providers::{Provider, ProviderBuilder},
};
use clap::Parser;
use contract_bindings::synd::{
    arb_chain_config::ArbChainConfig, arb_config_manager::ArbConfigManager,
};
use eyre::Result;
use shared::parse::{parse_address, parse_hash};
use tracing::{info, warn};

/// CLI args for the `synd-mchain` executable
#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct MchainConfig {
    /// time delay until a block is considered finalized
    #[arg(long, env = "FINALITY_DELAY", default_value_t = 60)]
    pub finality_delay: u64,

    /// data directory where mchain will store it's state
    #[arg(long, env = "DATADIR", default_value = "./datadir")]
    pub datadir: String,

    /// port to bind the jsonrpc API server to
    #[arg(long, env = "PORT", default_value_t = 8545)]
    pub port: u64,

    /// port to bind the metrics API to
    #[arg(long, env = "METRICS_PORT", default_value_t = 8546)]
    pub metrics_port: u16,

    /// chain id of the appchain
    #[arg(long, env = "APPCHAIN_CHAIN_ID")]
    pub appchain_chain_id: u64,

    /// The genesis config for a migrated appchain
    #[arg(long, env = "GENESIS_CONFIG")]
    pub genesis_config: Option<String>,

    /// The block number at which the migration took place (last known L1 block from the rollup
    /// point-of-view)
    #[arg(long = "settlement-start-block", env = "SETTLEMENT_START_BLOCK")]
    pub settlement_start_block: Option<u64>,

    /// The batch accumulator at the point of migration
    #[arg(long, env = "MIGRATED_BATCH_ACC", value_parser = parse_hash)]
    pub migrated_batch_acc: Option<B256>,

    /// The batch accumulator at the point of migration
    #[arg(long, env = "MIGRATED_BATCH_COUNT")]
    pub migrated_batch_count: Option<u64>,

    /// The delayed messages accumulator at the point of migration
    #[arg(long, env = "MIGRATED_DELAYED_MSGS_ACC", value_parser = parse_hash)]
    pub migrated_delayed_msgs_acc: Option<B256>,

    /// The delayed messages count at the point of migration
    #[arg(long, env = "MIGRATED_DELAYED_MSGS_COUNT")]
    pub migrated_delayed_msgs_count: Option<u64>,

    /// The WS URL of the sequencing chain ingestor
    #[arg(long = "config_manager-rpc-url", env = "CONFIG_MANAGER_RPC_URL")]
    pub config_manager_rpc_url: Option<String>,

    /// The address of the `ConfigManager` contract on the settlement chain
    #[arg(
        long = "config-manager-address",
        env = "CONFIG_MANAGER_ADDRESS",
        value_parser = parse_address
    )]
    pub config_manager_address: Option<Address>,
}

impl MchainConfig {
    /// obtains the optional migration params in the format the db function expects
    pub fn migration_config(&self) -> Option<MigrationParams> {
        self.migrated_batch_acc?;

        let settlement_start_block = self
            .settlement_start_block
            .unwrap_or_else(|| panic!("migration initial settlement block is none"));
        let batch_acc = self.migrated_batch_acc.unwrap_or_else(|| panic!("batch acc is none"));
        let batch_count =
            self.migrated_batch_count.unwrap_or_else(|| panic!("batch count is none"));
        let delayed_msgs_acc =
            self.migrated_delayed_msgs_acc.unwrap_or_else(|| panic!("delayed msgs acc is none"));
        let delayed_msgs_count = self
            .migrated_delayed_msgs_count
            .unwrap_or_else(|| panic!("delayed msgs count is none"));

        Some(MigrationParams {
            settlement_start_block,
            batch_acc,
            batch_count,
            delayed_msgs_acc,
            delayed_msgs_count,
        })
    }
}

// ------------------- on chain config -------------------

/// representation of the chain configuration that lives on-chain
#[allow(missing_docs)]
#[derive(Debug)]
struct ChainConfig {
    settlement_start_block: U256,
    migrated_batch_acc: U256,
    migrated_batch_count: U256,
    migrated_delayed_msgs_acc: U256,
    migrated_delayed_msgs_count: U256,
}

/// Fetches chain config if it exists and extends the passed config with any missing values
pub async fn with_onchain_config(config: MchainConfig) -> MchainConfig {
    let address = match config.config_manager_address {
        Some(addr) => addr,
        None => {
            warn!("No config_manager_address provided, skipping onchain config fetch");
            return config;
        }
    };
    let url = match &config.config_manager_rpc_url {
        Some(url) => url,
        None => {
            warn!("No config_manager_rpc_url provided, skipping onchain config fetch");
            return config;
        }
    };

    let provider = ProviderBuilder::new()
        .connect(url.as_str())
        .await
        .unwrap_or_else(|error| panic!("error connecting to RPC URL: {}", error));

    let onchain = get_config(address, U256::from(config.appchain_chain_id), provider)
        .await
        .unwrap_or_else(|error| panic!("error obtaining onchain config: {}", error));

    info!("got onchain config: {:?}", onchain);

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

    let settlement_start_block_call = arb_chain_config_contract.SETTLEMENT_START_BLOCK();
    let migrated_batch_acc_call = arb_chain_config_contract.MIGRATED_BATCH_ACC();
    let migrated_batch_count_call = arb_chain_config_contract.MIGRATED_BATCH_COUNT();
    let migrated_delayed_msgs_acc_call = arb_chain_config_contract.MIGRATED_DELAYED_MSGS_ACC();
    let migrated_delayed_msgs_count_call = arb_chain_config_contract.MIGRATED_DELAYED_MSGS_COUNT();

    let (
        settlement_start_block,
        migrated_batch_acc,
        migrated_batch_count,
        migrated_delayed_msgs_acc,
        migrated_delayed_msgs_count,
    ) = tokio::try_join!(
        settlement_start_block_call.call(),
        migrated_batch_acc_call.call(),
        migrated_batch_count_call.call(),
        migrated_delayed_msgs_acc_call.call(),
        migrated_delayed_msgs_count_call.call(),
    )?;

    Ok(ChainConfig {
        settlement_start_block,
        migrated_batch_acc,
        migrated_batch_count,
        migrated_delayed_msgs_acc,
        migrated_delayed_msgs_count,
    })
}

/// Overrides the config with onchain values where the config values are None
#[allow(clippy::cognitive_complexity)]
fn override_with_onchain_config(mut config: MchainConfig, onchain: &ChainConfig) -> MchainConfig {
    if config.settlement_start_block.is_none() {
        info!(
            "Using the settlement_start_block from on-chain config: {}",
            onchain.settlement_start_block
        );
        config.settlement_start_block = Some(
            onchain
                .settlement_start_block
                .try_into()
                .unwrap_or_else(|_| panic!("settlement_start_block should fit into u64")),
        );
    }

    if config.migrated_batch_acc.is_none() {
        info!(
            "Using the migrated_batch_acc from on-chain config: {:?}",
            onchain.migrated_batch_acc
        );
        config.migrated_batch_acc = Some(B256::from(onchain.migrated_batch_acc));
    }

    if config.migrated_batch_count.is_none() {
        info!(
            "Using the migrated_batch_count from on-chain config: {}",
            onchain.migrated_batch_count
        );
        config.migrated_batch_count = Some(
            onchain
                .migrated_batch_count
                .try_into()
                .unwrap_or_else(|_| panic!("migrated_batch_count should fit into u64")),
        );
    }

    if config.migrated_delayed_msgs_acc.is_none() {
        info!(
            "Using the migrated_delayed_msgs_acc from on-chain config: {:?}",
            onchain.migrated_delayed_msgs_acc
        );
        config.migrated_delayed_msgs_acc = Some(B256::from(onchain.migrated_delayed_msgs_acc));
    }

    if config.migrated_delayed_msgs_count.is_none() {
        info!(
            "Using the migrated_delayed_msgs_count from on-chain config: {}",
            onchain.migrated_delayed_msgs_count
        );
        config.migrated_delayed_msgs_count = Some(
            onchain
                .migrated_delayed_msgs_count
                .try_into()
                .unwrap_or_else(|_| panic!("migrated_delayed_msgs_count should fit into u64")),
        );
    }

    config
}

#[cfg(test)]
mod test {
    use super::*;
    use alloy::primitives::{b256, U256};

    fn create_test_config() -> MchainConfig {
        MchainConfig {
            finality_delay: 60,
            datadir: "./datadir".to_string(),
            port: 8545,
            metrics_port: 8546,
            appchain_chain_id: 1,
            genesis_config: None,
            settlement_start_block: None,
            migrated_batch_acc: None,
            migrated_batch_count: None,
            migrated_delayed_msgs_acc: None,
            migrated_delayed_msgs_count: None,
            config_manager_rpc_url: None,
            config_manager_address: None,
        }
    }

    #[test]
    fn test_override_with_onchain_config_default() {
        let config = create_test_config();

        // Create sample onchain config with known values
        let onchain = ChainConfig {
            settlement_start_block: U256::from(100),
            migrated_batch_acc: U256::from(0x1111_u64),
            migrated_batch_count: U256::from(50),
            migrated_delayed_msgs_acc: U256::from(0x2222_u64),
            migrated_delayed_msgs_count: U256::from(75),
        };

        // Apply overrides
        let config = override_with_onchain_config(config, &onchain);

        // Assert all fields that should be overridden are correctly set
        assert_eq!(
            config.settlement_start_block,
            Some(onchain.settlement_start_block.try_into().unwrap())
        );
        assert_eq!(config.migrated_batch_acc, Some(B256::from(onchain.migrated_batch_acc)));
        assert_eq!(
            config.migrated_batch_count,
            Some(onchain.migrated_batch_count.try_into().unwrap())
        );
        assert_eq!(
            config.migrated_delayed_msgs_acc,
            Some(B256::from(onchain.migrated_delayed_msgs_acc))
        );
        assert_eq!(
            config.migrated_delayed_msgs_count,
            Some(onchain.migrated_delayed_msgs_count.try_into().unwrap())
        );
    }

    #[test]
    fn test_override_with_onchain_config_preserves_existing() {
        let mut config = create_test_config();

        // Set some existing values that should NOT be overridden
        config.settlement_start_block = Some(999);
        config.migrated_batch_acc =
            Some(b256!("9999999999999999999999999999999999999999999999999999999999999999"));

        // Create sample onchain config
        let onchain = ChainConfig {
            settlement_start_block: U256::from(100),
            migrated_batch_acc: U256::from(0x1111_u64),
            migrated_batch_count: U256::from(50),
            migrated_delayed_msgs_acc: U256::from(0x2222_u64),
            migrated_delayed_msgs_count: U256::from(75),
        };

        // Apply overrides
        let config = override_with_onchain_config(config, &onchain);

        // Assert existing values are preserved
        assert_eq!(config.settlement_start_block, Some(999));
        assert_eq!(
            config.migrated_batch_acc,
            Some(b256!("9999999999999999999999999999999999999999999999999999999999999999"))
        );

        // Assert None values are overridden
        assert_eq!(
            config.migrated_batch_count,
            Some(onchain.migrated_batch_count.try_into().unwrap())
        );
        assert_eq!(
            config.migrated_delayed_msgs_acc,
            Some(B256::from(onchain.migrated_delayed_msgs_acc))
        );
        assert_eq!(
            config.migrated_delayed_msgs_count,
            Some(onchain.migrated_delayed_msgs_count.try_into().unwrap())
        );
    }
}
