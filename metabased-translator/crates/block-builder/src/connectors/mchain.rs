//! Connector for the `MetaChain`
use crate::{
    block_builder::BlockBuilderError,
    config::{get_default_private_key_signer, get_rollup_contract_address, BlockBuilderConfig},
    connectors::metrics::MChainMetrics,
};
use alloy::{
    eips::BlockNumberOrTag,
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, U256},
    providers::{
        ext::EngineApi as _,
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, IpcConnect, Provider, ProviderBuilder, RootProvider, WalletProvider,
    },
    rpc::types::{
        engine::{ForkchoiceState, PayloadAttributes, PayloadStatus, PayloadStatusEnum},
        Block, BlockTransactionsKind, TransactionRequest,
    },
};
use contract_bindings::arbitrum::rollup::Rollup::{self, RollupInstance};
use eyre::Result;

#[allow(missing_docs)]
pub type FilledProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;

#[allow(missing_docs)]
pub type HttpProvider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
>;

#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct MetaChainProvider {
    pub mchain_ipc_path: String,
    provider: FilledProvider,
    auth_provider: FilledProvider,
    metrics: MChainMetrics,
}

/// The chain id of the metachain. This is the same for all rollups.
/// TODO(SEQ-652): this should be configurable
pub const MCHAIN_ID: u64 = 84532;

impl MetaChainProvider {
    #[allow(missing_docs)]
    pub async fn get_block_by_number(
        &self,
        number: BlockNumberOrTag,
        kind: BlockTransactionsKind,
    ) -> Result<Option<Block>> {
        self.provider.get_block_by_number(number, kind).await.map_err(|e| e.into())
    }

    /// for testing only - get direct access to the rollup contract
    pub fn get_rollup(&self) -> RollupInstance<(), FilledProvider> {
        Rollup::new(get_rollup_contract_address(), self.provider.clone())
    }

    #[allow(missing_docs)]
    pub async fn get_block_number(&self) -> Result<u64> {
        self.provider.get_block_number().await.map_err(|e| e.into())
    }

    #[allow(missing_docs)]
    pub async fn get_block_receipts(
        &self,
        block: BlockNumberOrTag,
    ) -> Result<Vec<common::types::Receipt>> {
        self.provider
            .raw_request::<_, Vec<common::types::Receipt>>("eth_getBlockReceipts".into(), (block,))
            .await
            .map_err(|e| e.into())
    }

    /// Create a provider for the `MetaChain`
    /// The rollup contract is only deployed to the chain when it is
    /// newly created and on the genesis block.
    /// The genesis block must have a timestamp of 0.
    pub async fn start(config: &BlockBuilderConfig, metrics: &MChainMetrics) -> Result<Self> {
        let provider = ProviderBuilder::new()
            .wallet(EthereumWallet::from(get_default_private_key_signer()))
            .on_ipc(IpcConnect::new(config.mchain_ipc_path.clone()))
            .await?;
        let auth_provider = ProviderBuilder::new()
            .wallet(EthereumWallet::from(get_default_private_key_signer()))
            .on_ipc(IpcConnect::new(config.mchain_auth_ipc_path.clone()))
            .await?;
        let rollup_config = Self::rollup_config(config.target_chain_id);

        let mchain = Self {
            mchain_ipc_path: config.mchain_ipc_path.clone(),
            provider,
            auth_provider,
            metrics: metrics.to_owned(),
        };

        if mchain.get_block_number().await? == 0 {
            _ = Rollup::deploy_builder(
                &mchain.provider,
                U256::from(config.target_chain_id),
                rollup_config.clone(),
            )
            .nonce(0)
            .send()
            .await?;
            mchain.mine_block(0).await?;
        }

        Ok(mchain)
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
    pub async fn mine_block(&self, block_timestamp_secs: u64) -> Result<()> {
        let attr = PayloadAttributes {
            timestamp: block_timestamp_secs,
            prev_randao: Default::default(),
            suggested_fee_recipient: Default::default(),
            withdrawals: Some(Default::default()),
            parent_beacon_block_root: Some(Default::default()),
        };

        #[allow(clippy::expect_used)]
        let block = self
            .get_block_by_number(BlockNumberOrTag::Latest, BlockTransactionsKind::Hashes)
            .await?
            .expect("latest block not found");

        // TODO(SEQ-653): set safe and finalized block hashes properly
        let req = self
            .auth_provider
            .fork_choice_updated_v3(
                ForkchoiceState {
                    head_block_hash: block.header.hash,
                    safe_block_hash: block.header.hash,
                    finalized_block_hash: block.header.hash,
                },
                Some(attr),
            )
            .await?;
        assert_eq!(
            req.payload_status,
            PayloadStatus {
                status: PayloadStatusEnum::Valid,
                latest_valid_hash: Some(block.header.hash)
            }
        );

        #[allow(clippy::expect_used)]
        let payload =
            self.auth_provider.get_payload_v3(req.payload_id.expect("missing payload id")).await?;
        let block_hash = payload.execution_payload.payload_inner.payload_inner.block_hash;
        let status = self
            .auth_provider
            .new_payload_v3(payload.execution_payload, Default::default(), Default::default())
            .await?;
        assert_eq!(
            status,
            PayloadStatus { status: PayloadStatusEnum::Valid, latest_valid_hash: Some(block_hash) }
        );
        let fcu = self
            .auth_provider
            .fork_choice_updated_v3(
                ForkchoiceState {
                    head_block_hash: block_hash,
                    safe_block_hash: block_hash,
                    finalized_block_hash: block_hash,
                },
                None,
            )
            .await?;
        assert_eq!(
            fcu.payload_status,
            PayloadStatus { status: PayloadStatusEnum::Valid, latest_valid_hash: Some(block_hash) },
        );
        self.metrics.record_last_block_mined(block.header.number + 1, block_timestamp_secs);
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
                "InitialArbOSVersion": 32,
                "InitialChainOwner": "{zero}",
                "GenesisBlockNum": 0
              }}
            }}"#
        )
    }

    /// Get the nitro json configuration data for the rollup
    pub fn rollup_info(rollup_config: &str, chain_name: &str) -> String {
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

    struct MetricsState {
        /// Prometheus registry
        pub registry: Registry,
    }

    #[tokio::test]
    #[ignore] // TODO SEQ-528 unskip
    async fn test_anvil_rollback() -> Result<()> {
        let config = BlockBuilderConfig {
            mchain_ipc_path: "http://127.0.0.1:9388".parse()?,
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
            mchain_ipc_path: "http://127.0.0.1:9488".to_string(),
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
