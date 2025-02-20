//! Anvil connector for the `MetaChain`
use crate::{
    block_builder::{
        AnvilStartError::{InvalidHost, NoPort, PortUnavailable},
        BlockBuilderError,
    },
    config::{get_default_private_key_signer, get_rollup_contract_address, BlockBuilderConfig},
    connectors::metrics::MChainMetrics,
};
use alloy::{
    network::{Ethereum, EthereumWallet},
    node_bindings::{Anvil, AnvilInstance},
    primitives::{Address, U256},
    providers::{
        ext::AnvilApi,
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, Provider, ProviderBuilder, RootProvider,
    },
    rpc::types::{anvil::MineOptions, TransactionRequest},
    transports::http::Http,
};
use contract_bindings::arbitrum::rollup::Rollup;
use eyre::{Error, Result};
use reqwest::Client;
use std::net::TcpListener;
use thiserror::Error;
use tracing::{debug, error, info};
use url::Host;

/// Possible errors when mining a block
#[derive(Debug, Error)]
pub enum MineBlockError {
    /// Block list is empty
    #[error("Mining operation returned an empty block list")]
    EmptyBlockList,

    /// Failed to mine block
    #[error("Failed to mine block: {0}")]
    MiningFailed(#[from] Error),
}

#[allow(missing_docs)]
pub type FilledProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider<Http<Client>>,
    Http<Client>,
    Ethereum,
>;

/// `MetaChainProvider` starts the anvil chain when `start` is called and stops the chain when it
/// is dropped.
#[derive(Debug)]
#[allow(missing_docs)]
pub struct MetaChainProvider {
    pub anvil: AnvilInstance,
    pub provider: FilledProvider,
    pub rollup: Rollup::RollupInstance<Http<Client>, FilledProvider>,
    pub target_chain_id: u64,
    pub metrics: MChainMetrics,
}

/// The chain id of the metachain. This is the same for all rollups.
pub const MCHAIN_ID: u64 = 84532;

impl MetaChainProvider {
    /// Starts the Anvil instance and creates a provider for the `MetaChain`
    /// If file in `BlockBuilderConfig` is set to a non-empty string, the anvil node stores and
    /// loads state from the file. The rollup contract is only deployed to the chain when it is
    /// newly created and on the genesis block.
    pub async fn start(
        config: &BlockBuilderConfig,
        datadir: &str,
        metrics: &MChainMetrics,
    ) -> Result<Self> {
        let port = config.mchain_url.port().ok_or_else(|| BlockBuilderError::AnvilStart(NoPort))?;

        if !is_port_available(port) {
            return Err(BlockBuilderError::AnvilStart(PortUnavailable {
                mchain_url: config.mchain_url.clone(),
                port,
            })
            .into());
        }

        let host_string = match config.mchain_url.host() {
            Some(Host::Domain(domain)) => domain.to_string(),
            Some(Host::Ipv4(ip)) => ip.to_string(),
            Some(Host::Ipv6(ip)) => ip.to_string(),
            _ => return Err(BlockBuilderError::AnvilStart(InvalidHost).into()),
        };
        let host_str = host_string.as_str();
        let ts = config.genesis_timestamp.to_string();

        let mut args = vec![
            "--host",
            host_str,
            "--base-fee",
            "0",
            "--gas-limit",
            "30000000",
            "--timestamp",
            ts.as_str(),
            "--no-mining",
        ];

        let state_interval = config.anvil_state_interval.to_string();
        let max_persisted_states = config.max_persisted_states.to_string();
        let prune_history = config.prune_history.to_string();
        let anvil_state_path = format!("{}/anvil_state.json", datadir);

        if !datadir.is_empty() {
            args.push("--state");
            args.push(&anvil_state_path);
            args.push("--state-interval");
            args.push(&state_interval);
        }

        if config.max_persisted_states > 0 {
            args.push("--max-persisted-states");
            args.push(&max_persisted_states);
        } else {
            args.push("--prune-history");
            args.push(&prune_history);
        }

        debug!("Anvil args: {:?}", args);
        let anvil = Anvil::new().port(port).chain_id(MCHAIN_ID).args(args).try_spawn()?;
        info!("Anvil started at {}:{}", host_str, port);

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(EthereumWallet::from(get_default_private_key_signer()))
            .on_http(anvil.endpoint_url());
        provider.anvil_set_block_timestamp_interval(0).await?;

        let rollup_config = Self::rollup_config(config.target_chain_id);

        if provider.get_block_number().await? == 0 {
            let _ = Rollup::deploy_builder(
                &provider,
                U256::from(config.target_chain_id),
                rollup_config.clone(),
            )
            .nonce(0)
            .send()
            .await?;
            provider.evm_mine(None).await?;
        }

        let rollup = Rollup::new(get_rollup_contract_address(), provider.clone());

        Ok(Self {
            anvil,
            provider,
            rollup,
            target_chain_id: config.target_chain_id,
            metrics: metrics.to_owned(),
        })
    }

    /// Submits a list of transactions to the `MetaChain`
    pub async fn submit_txns(&self, txns: Vec<TransactionRequest>) -> Result<()> {
        for txn in txns {
            let _ = self
                .provider
                .send_transaction(txn)
                .await
                .map_err(BlockBuilderError::SubmitTxnError)?;
        }

        Ok(())
    }

    /// Mines a block on the `MetaChain`
    pub async fn mine_block(&self, block_timestamp_secs: u64) -> Result<(), MineBlockError> {
        let opts = MineOptions::Options {
            timestamp: (block_timestamp_secs > 0).then_some(block_timestamp_secs),
            blocks: Some(1),
        };
        let result = self.provider.anvil_mine_detailed(Some(opts)).await;
        debug!("{}", format!("Mined block on MetaChain {:?}", result));
        match result {
            Ok(mut mined_blocks) if !mined_blocks.is_empty() => {
                let first_block = mined_blocks.remove(0); // Extract the first block
                debug!("Mined block on MetaChain {:?}", first_block);
                self.metrics.record_last_block_mined(&first_block);
                Ok(())
            }
            Ok(_) => {
                error!("Mining succeeded but returned an empty block list");
                Err(MineBlockError::EmptyBlockList)
            }
            Err(e) => {
                error!("Failed to mine block: {:?}", e);
                Err(MineBlockError::MiningFailed(e.into()))
            }
        }
    }

    /// Return the on-chain config for a rollup with a given chain id
    pub fn rollup_config(chain_id: u64) -> String {
        let zero = Address::ZERO;
        format!(
            r#"{{
              "chainId": {chain_id},
              "homesteadBlock": 0,
              "daoForkBlock": null,
              "daoForkSupport": true,
              "eip150Block": 0,
              "eip150Hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
              "eip155Block": 0,
              "eip158Block": 0,
              "byzantiumBlock": 0,
              "constantinopleBlock": 0,
              "petersburgBlock": 0,
              "istanbulBlock": 0,
              "muirGlacierBlock": 0,
              "berlinBlock": 0,
              "londonBlock": 0,
              "clique": {{
                "period": 0,
                "epoch": 0
              }},
              "arbitrum": {{
                "EnableArbOS": true,
                "AllowDebugPrecompiles": false,
                "DataAvailabilityCommittee": false,
                "InitialArbOSVersion": 32,
                "InitialChainOwner": "{zero}",
                "GenesisBlockNum": 0
              }}
            }}"#
        )
    }

    /// Get the nitro json configuration data for the rollup
    pub fn rollup_info(&self, chain_name: &str) -> String {
        let rollup_config = Self::rollup_config(self.target_chain_id);
        let rollup = get_rollup_contract_address();
        let deployed_at: u64 = 1;
        let zero = Address::ZERO;
        format!(
            r#"[{{
              "chain-name": "{chain_name}",
              "parent-chain-id": {MCHAIN_ID},
              "parent-chain-is-arbitrum": false,
              "sequencer-url": "",
              "secondary-forwarding-target": "",
              "feed-url": "",
              "secondary-feed-url": "",
              "das-index-url": "",
              "has-genesis-state": false,
              "chain-config": {rollup_config},
              "rollup": {{
                "bridge": "{rollup}",
                "inbox": "{zero}",
                "sequencer-inbox": "{rollup}",
                "deployed-at": {deployed_at},
                "rollup": "{zero}",
                "native-token": "{zero}",
                "upgrade-executor": "{zero}",
                "validator-wallet-creator": "{zero}"
              }}
            }}]"#
        )
    }

    /// Rolls back the chain to a specific block number by performing a reorg
    pub async fn rollback_to_block(&self, _block_number: u64) -> Result<()> {
        panic!("rollback not implemented"); // TODO SEQ-528

        // let current_block = self.provider.get_block_number().await?;
        // let depth = current_block - block_number;
        // self.provider.anvil_reorg(ReorgOptions { depth, tx_block_pairs: vec![] }).await?;

        // // Verify we're at the correct block
        // let new_block = self.provider.get_block_number().await?;
        // if new_block != block_number {
        //     return Err(eyre::eyre!(
        //         "Failed to rollback: expected block {}, got block {}",
        //         block_number,
        //         new_block
        //     ));
        // }

        // Ok(())
    }
}

/// Check if a port is available by attempting to bind to it
///
/// The port will be used for both HTTP and WebSocket connections, a feature provided by Anvil.
/// See: <https://book.getfoundry.sh/reference/anvil/#supported-transport-layers>
pub fn is_port_available(port: u16) -> bool {
    let addr = format!("127.0.0.1:{}", port);
    TcpListener::bind(addr).is_ok()
}

/// Custom [`Drop`] to make sure the Anvil process is terminated and the port is released.
impl Drop for MetaChainProvider {
    fn drop(&mut self) {
        // Ensure anvil process is terminated
        info!("Terminating anvil");
        let id = self.anvil.child().id();
        _ = std::process::Command::new("kill").arg(id.to_string()).output();
        _ = self.anvil.child_mut().wait();
        info!("Terminated anvil");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{eips::BlockId, rpc::types::BlockTransactionsKind};
    use prometheus_client::registry::Registry;
    use std::time::Duration;
    use test_utils::test_path;
    use url::Url;

    struct MetricsState {
        /// Prometheus registry
        pub registry: Registry,
    }

    #[tokio::test]
    async fn test_anvil_resume() -> Result<()> {
        let datadir = test_path("datadir");
        let cfg = BlockBuilderConfig {
            mchain_url: Url::parse("http://127.0.0.1:9188").expect("Invalid URL"),
            ..Default::default()
        };
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = MChainMetrics::new(&mut metrics_state.registry);
        let mut provider = MetaChainProvider::start(&cfg, &datadir, &metrics).await?;
        provider.mine_block(0).await?;
        let old_count = provider.provider.get_block_number().await?;
        drop(provider); // To explicitly release the port

        let metrics = MChainMetrics::new(&mut metrics_state.registry);
        provider = MetaChainProvider::start(&cfg, &datadir, &metrics).await?;
        let new_count = provider.provider.get_block_number().await?;
        assert_eq!(old_count, new_count);
        Ok(())
    }

    #[tokio::test]
    async fn test_block_persistence() -> Result<()> {
        let datadir = test_path("datadir");

        let config = BlockBuilderConfig {
            genesis_timestamp: 999,
            mchain_url: Url::parse("http://127.0.0.1:9288").expect("Invalid URL"),
            ..Default::default()
        };
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = MChainMetrics::new(&mut metrics_state.registry);

        // First instance: create blocks
        {
            let chain = MetaChainProvider::start(&config, &datadir, &metrics).await?;

            // Mine 1000 blocks
            for i in 1000..2000 {
                chain.mine_block(i as u64).await?;
            }

            // Let anvil write state before dropping
            tokio::time::sleep(Duration::from_secs(1)).await;
        } // First instance is dropped here

        // Second instance: verify blocks
        let metrics = MChainMetrics::new(&mut metrics_state.registry);
        let chain = MetaChainProvider::start(&config, &datadir, &metrics).await?;

        // Check a few random blocks are accessible
        for block_num in [0, 42, 567, 999, 1000] {
            let block = chain
                .provider
                .get_block(BlockId::Number(block_num.into()), BlockTransactionsKind::Full)
                .await?;
            assert!(block.is_some(), "Block {} should be available", block_num);
            assert_eq!(
                block.unwrap().header.number,
                block_num,
                "Block number mismatch for block {}",
                block_num
            );
        }

        Ok(())
    }

    #[tokio::test]
    #[ignore] // TODO SEQ-528 unskip
    async fn test_anvil_rollback() -> Result<()> {
        let config = BlockBuilderConfig {
            genesis_timestamp: 999,
            mchain_url: "http://127.0.0.1:9388".parse()?,
            ..Default::default()
        };
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = MChainMetrics::new(&mut metrics_state.registry);
        let datadir = test_path("datadir");
        let chain = MetaChainProvider::start(&config, &datadir, &metrics).await?;
        // Mine 10 blocks
        for i in 1000..1010 {
            chain.mine_block(i as u64).await?;
        }

        chain.rollback_to_block(5).await?;
        let block_num = chain.provider.get_block_number().await?;
        assert_eq!(block_num, 5, "Chain should be at block 5");

        Ok(())
    }

    #[tokio::test]
    #[ignore] // just for debugging
    async fn test_anvil_stop_resume() -> Result<()> {
        let datadir = test_path("datadir_dump_test");
        let genesis_ts = 1000;
        let config = BlockBuilderConfig {
            mchain_url: Url::parse("http://127.0.0.1:9488").expect("Invalid URL"),
            genesis_timestamp: genesis_ts,
            ..Default::default()
        };

        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = MChainMetrics::new(&mut metrics_state.registry);
        let provider = MetaChainProvider::start(&config, &datadir, &metrics).await?;

        // Mine some blocks with increasing timestamps
        for i in 1..100_000 {
            provider.mine_block(genesis_ts + (i * 1000)).await?;
        }
        let original_block = provider.provider.get_block_number().await?;

        drop(provider);

        // Second instance: restore state
        let metrics = MChainMetrics::new(&mut metrics_state.registry);
        let provider = MetaChainProvider::start(&config, &datadir, &metrics).await?;

        // Verify state was restored correctly
        let restored_block = provider.provider.get_block_number().await?;
        assert_eq!(original_block, restored_block, "Block number should match after restore");

        // Check random historical blocks are accessible
        for block_num in [42, 567, 12345, 50000, 100_000] {
            let block = provider
                .provider
                .get_block(BlockId::Number(block_num.into()), BlockTransactionsKind::Full)
                .await?;
            assert!(block.is_some(), "Block {} should be available", block_num);
            assert_eq!(
                block.unwrap().header.number,
                block_num,
                "Block number mismatch for block {}",
                block_num
            );
        }

        Ok(())
    }
}
