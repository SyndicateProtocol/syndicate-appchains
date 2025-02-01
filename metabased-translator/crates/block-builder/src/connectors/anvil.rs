//! Anvil connector for the `MetaChain`
use crate::{
    block_builder::{
        AnvilStartError::{InvalidHost, NoPort, PortUnavailable},
        BlockBuilderError,
    },
    config::{get_default_private_key_signer, get_rollup_contract_address, BlockBuilderConfig},
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
use eyre::Result;
use reqwest::Client;
use std::{net::TcpListener, time::Duration};
use tracing::debug;
use url::Host;

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
}

/// The chain id of the metachain. This is the same for all rollups.
pub const MCHAIN_ID: u64 = 84532;

impl MetaChainProvider {
    /// Starts the Anvil instance and creates a provider for the `MetaChain`
    /// If file in `BlockBuilderConfig` is set to a non-empty string, the anvil node stores and
    /// loads state from the file. The rollup contract is only deployed to the chain when it is
    /// newly created and on the genesis block.
    pub async fn start(config: &BlockBuilderConfig) -> Result<Self> {
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
        let prune_history = config.anvil_prune_history.to_string();

        if !config.anvil_state_path.is_empty() {
            args.push("--state");
            args.push(&config.anvil_state_path);
            args.push("--state-interval");
            args.push(&state_interval);
            args.push("--prune-history");
            args.push(&prune_history);
        }

        let anvil = Anvil::new().port(port).chain_id(MCHAIN_ID).args(args).try_spawn()?;
        debug!("Anvil started at {}:{}", host_str, port);

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

        Ok(Self { anvil, provider, rollup, target_chain_id: config.target_chain_id })
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
    pub async fn mine_block(&self, block_timestamp_secs: u64) -> Result<()> {
        let opts = MineOptions::Options {
            timestamp: (block_timestamp_secs > 0).then_some(block_timestamp_secs),
            blocks: Some(1),
        };
        let result = self.provider.anvil_mine_detailed(Some(opts)).await;
        debug!("{}", format!("Mined block on MetaChain {:?}", result));
        result?;

        Ok(())
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
                "InitialArbOSVersion": 10,
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
        panic!("not implemented"); // TODO SEQ-528

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

/// Custom [`Drop`] to make sure the Anvil process is terminated and the port is released.
impl Drop for MetaChainProvider {
    fn drop(&mut self) {
        // Ensure anvil process is terminated
        let id = self.anvil.child().id();
        let _ = std::process::Command::new("kill").arg(id.to_string()).output();
        // Give the port time to be released
        std::thread::sleep(Duration::from_millis(500));
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{eips::BlockId, rpc::types::BlockTransactionsKind};
    use serial_test::serial;
    use std::{env::temp_dir, fs, path::PathBuf};
    use tracing::info;
    use url::Url;

    #[serial]
    #[tokio::test]
    async fn test_anvil_resume() -> Result<()> {
        let file = temp_dir().join("dump.json");
        let cfg = BlockBuilderConfig {
            mchain_url: Url::parse("http://127.0.0.1:9888").expect("Invalid URL"),
            anvil_state_path: file.to_str().unwrap().to_string(),
            ..Default::default()
        };
        let _ = fs::remove_file(file);
        let mut provider = MetaChainProvider::start(&cfg).await?;
        provider.mine_block(0).await?;
        let old_count = provider.provider.get_block_number().await?;
        drop(provider); // To explicitly release the port

        provider = MetaChainProvider::start(&cfg).await?;
        let new_count = provider.provider.get_block_number().await?;
        assert_eq!(old_count, new_count);
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_block_persistence() -> Result<()> {
        let state_path = PathBuf::from("test-state.json");

        // check is_port_available and wait 2 sec for anvil to start
        if !is_port_available(9888) {
            info!("Waiting 2sec for anvil to start... at port 9888");
            tokio::time::sleep(Duration::from_secs(2)).await;
        }

        let config = BlockBuilderConfig {
            anvil_state_path: state_path.to_str().unwrap().to_string(),
            genesis_timestamp: 999,
            mchain_url: Url::parse("http://127.0.0.1:9888").expect("Invalid URL"),
            ..Default::default()
        };

        // First instance: create blocks
        {
            let chain = MetaChainProvider::start(&config).await?;

            // Mine 1000 blocks
            for i in 1000..2000 {
                chain.mine_block(i as u64).await?;
            }

            // Let anvil write state before dropping
            tokio::time::sleep(Duration::from_secs(1)).await;
        } // First instance is dropped here

        // Second instance: verify blocks
        let chain = MetaChainProvider::start(&config).await?;

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

        // Drop chain before cleanup
        drop(chain);

        // Cleanup
        fs::remove_file(state_path)?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    #[ignore] // TODO SEQ-528 unskip
    async fn test_anvil_rollback() -> Result<()> {
        // TODO SEQ-528 refactor these test-state-paths
        let state_path = PathBuf::from("test-state1.json");
        let config = BlockBuilderConfig {
            anvil_state_path: state_path.to_str().unwrap().to_string(),
            genesis_timestamp: 999,
            ..Default::default()
        };

        let chain = MetaChainProvider::start(&config).await?;
        // Mine 10 blocks
        for i in 1000..1010 {
            chain.mine_block(i as u64).await?;
        }

        chain.rollback_to_block(5).await?;
        let block_num = chain.provider.get_block_number().await?;
        assert_eq!(block_num, 5, "Chain should be at block 5");

        Ok(())
    }
}
