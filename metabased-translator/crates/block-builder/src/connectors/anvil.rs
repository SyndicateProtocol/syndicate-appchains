//! Anvil connector for the `MetaChain`
use crate::{
    block_builder::BlockBuilderError,
    config::{get_default_private_key_signer, BlockBuilderConfig},
    rollups::arbitrum,
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
        Identity, Provider, ProviderBuilder, RootProvider, WalletProvider,
    },
    rpc::types::{anvil::MineOptions, TransactionRequest},
    transports::http::Http,
};
use contract_bindings::arbitrum::rollup::Rollup;
use eyre::Result;
use reqwest::Client;
use std::net::TcpListener;
use tracing::info;

type FilledProvider = FillProvider<
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
    pub rollup_info: String,
}

/// The chain id of the metachain. This is the same for all rollups.
pub const MCHAIN_ID: u64 = 84532;

impl MetaChainProvider {
    /// Starts the Anvil instance and creates a provider for the `MetaChain`
    /// If file in `BlockBuilderConfig` is set to a non-empty string, the anvil node stores and
    /// loads state from the file. The rollup contract is only deployed to the chain when it is
    /// newly created and on the genesis block.
    pub async fn start(config: BlockBuilderConfig) -> Result<Self> {
        let port = find_available_port(config.port, 10).ok_or_else(|| {
            BlockBuilderError::AnvilStartError("No available ports found after 10 attempts")
        })?;

        if port != config.port {
            info!("Port {} is in use, switching to port {}", config.port, port);
        }

        let ts = config.genesis_timestamp.to_string();

        let mut args = vec![
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

        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(EthereumWallet::from(get_default_private_key_signer()))
            .on_http(anvil.endpoint_url());

        let rollup_config = Self::rollup_config(U256::from(config.target_chain_id));

        let deploy_tx = Rollup::deploy_builder(
            &provider,
            U256::from(config.target_chain_id),
            rollup_config.clone(),
        )
        .nonce(0)
        .from(provider.default_signer_address());
        let contract_addr =
            deploy_tx.calculate_create_address().ok_or(BlockBuilderError::NoContractAddress())?;
        provider.anvil_set_block_timestamp_interval(1).await?;

        if provider.get_block_number().await? == 0 {
            _ = deploy_tx.send().await?;
            provider.anvil_mine(Some(U256::from(1)), None::<U256>).await?;
        }

        let rollup = Rollup::new(contract_addr, provider.clone());

        let rollup_info = Self::rollup_info(
            "test",
            U256::from(MCHAIN_ID),
            &rollup_config,
            rollup.address(),
            U256::from(1),
        );

        Ok(Self { anvil, provider, rollup, rollup_info })
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
        info!("{}", format!("Mined block on MetaChain {:?}", result));
        result?;

        Ok(())
    }

    fn rollup_config(chain_id: U256) -> String {
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

    fn rollup_info(
        chain_name: &str,
        chain_id: U256,
        rollup_config: &str,
        rollup: &Address,
        deployed_at: U256,
    ) -> String {
        let zero = Address::ZERO;
        format!(
            r#"[{{
              "chain-name": "{chain_name}",
              "parent-chain-id": {chain_id},
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

    /// post a rollup batch to the mchain and mine the next block
    pub async fn send_batch(&self, batch: &arbitrum::batch::Batch) -> Result<()> {
        let tx = self
            .rollup
            .postBatch(
                U256::from(
                    batch
                        .0
                        .iter()
                        .filter(|x| matches!(x, arbitrum::batch::BatchMessage::Delayed))
                        .count(),
                ),
                batch.encode()?,
            )
            .send()
            .await?
            .watch();
        self.mine_block(0).await?;
        tx.await?;
        Ok(())
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

/// Check if a port is available by attempting to bind to it
///
/// The port will be used for both HTTP and WebSocket connections, a feature provided by Anvil.
/// See: <https://book.getfoundry.sh/reference/anvil/#supported-transport-layers>
pub fn is_port_available(port: u16) -> bool {
    let addr = format!("127.0.0.1:{}", port);
    TcpListener::bind(addr).is_ok()
}

/// Try to find an available port starting from `base_port`
/// Increments by 100 each try, up to `max_attempts`
pub fn find_available_port(base_port: u16, max_attempts: u16) -> Option<u16> {
    for attempt in 0..max_attempts {
        let port = base_port.saturating_add(attempt * 100);
        if is_port_available(port) {
            return Some(port);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{eips::BlockId, rpc::types::BlockTransactionsKind};
    use std::{env::temp_dir, fs, path::PathBuf};

    #[tokio::test]
    async fn test_port_availability_checking() -> Result<()> {
        // Initial port should be available
        let base_port = 1111;
        assert!(is_port_available(base_port), "Base port should be available initially");

        // Bind to the port to make it unavailable
        let _listener = TcpListener::bind(format!("127.0.0.1:{}", base_port))?;
        assert!(!is_port_available(base_port), "Base port should be unavailable after binding");

        // Should find next available port
        let port = find_available_port(base_port, 10)
            .ok_or_else(|| eyre::eyre!("Failed to find available port"))?;

        // Port should be base_port + N*100 where N is 1..10
        assert!(port > base_port, "New port should be higher than base port");
        assert_eq!((port - base_port) % 100, 0, "Port increment should be multiple of 100");
        assert!(port <= base_port + 900, "Port should not exceed max attempts range");

        // New port should be available
        assert!(is_port_available(port), "New port should be available");

        Ok(())
    }

    #[tokio::test]
    async fn test_anvil_resume() -> Result<()> {
        let file = temp_dir().join("dump.json");
        let cfg = BlockBuilderConfig {
            port: 9888,
            anvil_state_path: file.to_str().unwrap().to_string(),
            ..Default::default()
        };
        _ = fs::remove_file(file);
        let mut provider = MetaChainProvider::start(cfg.clone()).await?;
        provider.mine_block(0).await?;
        let old_count = provider.provider.get_block_number().await?;
        std::process::Command::new("kill").arg(provider.anvil.child().id().to_string()).output()?;
        provider.anvil.child_mut().wait()?;
        provider = MetaChainProvider::start(cfg).await?;
        let new_count = provider.provider.get_block_number().await?;
        assert_eq!(old_count, new_count);
        Ok(())
    }

    #[tokio::test]
    async fn test_block_persistence() -> Result<()> {
        let state_path = PathBuf::from("test-state.json");

        let config = BlockBuilderConfig {
            anvil_state_path: state_path.to_str().unwrap().to_string(),
            genesis_timestamp: 999,
            port: 9889,
            ..Default::default()
        };

        // First instance: create blocks
        {
            let chain = MetaChainProvider::start(config.clone()).await?;

            // Mine 1000 blocks
            for i in 1000..2000 {
                chain.mine_block(i as u64).await?;
            }

            // Let anvil write state before dropping
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        } // First instance is dropped here

        // Second instance: verify blocks
        let chain = MetaChainProvider::start(config).await?;

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

        // Cleanup
        fs::remove_file(state_path)?;

        Ok(())
    }

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

        let chain = MetaChainProvider::start(config).await?;
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
