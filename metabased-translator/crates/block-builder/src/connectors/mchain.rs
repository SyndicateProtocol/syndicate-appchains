//! Connector for the `MetaChain`
use crate::{
    block_builder::BlockBuilderError,
    config::{get_default_private_key_signer, get_rollup_contract_address, BlockBuilderConfig},
    connectors::metrics::MChainMetrics,
};
use alloy::{
    network::{Ethereum, EthereumWallet, TransactionBuilder},
    primitives::{Address, U256},
    providers::{
        ext::AnvilApi as _,
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
use eyre::{Error, Result};
use reqwest::Client;
use thiserror::Error;
use tracing::{debug, error};

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
    pub mchain_url: url::Url,
    pub provider: FilledProvider,
    pub rollup: Rollup::RollupInstance<Http<Client>, FilledProvider>,
    pub target_chain_id: u64,
    pub metrics: MChainMetrics,
}

/// The chain id of the metachain. This is the same for all rollups.
pub const MCHAIN_ID: u64 = 84532;

impl MetaChainProvider {
    /// Create a provider for the `MetaChain`
    /// The rollup contract is only deployed to the chain when it is
    /// newly created and on the genesis block.
    /// The genesis block must have a timestamp of 0.
    pub async fn start(config: &BlockBuilderConfig, metrics: &MChainMetrics) -> Result<Self> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(EthereumWallet::from(get_default_private_key_signer()))
            .on_http(config.mchain_url.clone());
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
            provider.evm_mine(Some(MineOptions::Timestamp(Some(0)))).await?;
        }

        let rollup = Rollup::new(get_rollup_contract_address(), provider.clone());

        Ok(Self {
            mchain_url: config.mchain_url.clone(),
            provider,
            rollup,
            target_chain_id: config.target_chain_id,
            metrics: metrics.to_owned(),
        })
    }

    /// Submits a list of transactions to the `MetaChain`
    pub async fn submit_txns(&self, txns: Vec<TransactionRequest>) -> Result<()> {
        let mut nonce =
            self.provider.get_transaction_count(self.provider.default_signer_address()).await?;
        for txn in txns {
            let tx = txn
                .with_chain_id(MCHAIN_ID)
                .gas_limit(1000000)
                .max_fee_per_gas(1000000000)
                .max_priority_fee_per_gas(0)
                .nonce(nonce)
                .build(self.provider.wallet())
                .await?;
            let _ = self
                .provider
                .send_tx_envelope(tx)
                .await
                .map_err(BlockBuilderError::SubmitTxnError)?;
            nonce += 1;
        }

        Ok(())
    }

    /// Mines a block on the `MetaChain`
    pub async fn mine_block(&self, block_timestamp_secs: u64) -> Result<(), MineBlockError> {
        let mut result = self
            .provider
            .anvil_mine_detailed(Some(MineOptions::Timestamp(Some(block_timestamp_secs))))
            .await;
        debug!("{}", format!("Mined block on MetaChain {:?}", result));
        // TODO: remove this hack once anvil_mine_detailed() returns the blocks
        result = self
            .provider
            .raw_request("eth_getBlockByNumber".into(), ("latest", false))
            .await
            .map(|x| vec![x]);
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{eips::BlockId, rpc::types::BlockTransactionsKind};
    use prometheus_client::registry::Registry;
    use url::Url;

    struct MetricsState {
        /// Prometheus registry
        pub registry: Registry,
    }

    #[tokio::test]
    #[ignore] // TODO SEQ-528 unskip
    async fn test_anvil_rollback() -> Result<()> {
        let config = BlockBuilderConfig {
            mchain_url: "http://127.0.0.1:9388".parse()?,
            ..Default::default()
        };
        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = MChainMetrics::new(&mut metrics_state.registry);
        let chain = MetaChainProvider::start(&config, &metrics).await?;
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
        let genesis_ts = 0;
        let config = BlockBuilderConfig {
            mchain_url: Url::parse("http://127.0.0.1:9488").expect("Invalid URL"),
            ..Default::default()
        };

        let mut metrics_state = MetricsState { registry: Registry::default() };
        let metrics = MChainMetrics::new(&mut metrics_state.registry);
        let provider = MetaChainProvider::start(&config, &metrics).await?;

        // Mine some blocks with increasing timestamps
        for i in 1..100_000 {
            provider.mine_block(genesis_ts + (i * 1000)).await?;
        }
        let original_block = provider.provider.get_block_number().await?;

        drop(provider);

        // Second instance: restore state
        let metrics = MChainMetrics::new(&mut metrics_state.registry);
        let provider = MetaChainProvider::start(&config, &metrics).await?;

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
