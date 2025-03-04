//! Connector for the `MetaChain`
use crate::{
    block_builder::BlockBuilderError,
    config::{get_default_private_key_signer, get_rollup_contract_address, BlockBuilderConfig},
    connectors::metrics::MChainMetrics,
};
use alloy::{
    eips::BlockNumberOrTag,
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, BlockHash, U256},
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
    /// for testing only - get direct access to the rollup contract
    pub fn get_rollup(&self) -> RollupInstance<(), FilledProvider> {
        Rollup::new(get_rollup_contract_address(), self.provider.clone())
    }

    #[allow(missing_docs)]
    pub async fn get_block_by_number(
        &self,
        number: BlockNumberOrTag,
        kind: BlockTransactionsKind,
    ) -> Result<Option<Block>> {
        self.provider.get_block_by_number(number, kind).await.map_err(|e| e.into())
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
                rollup_config,
            )
            .nonce(0)
            .send()
            .await?;
            mchain.mine_block(0).await?;
        }

        // exit with an error message if the rollup contract bytecode does not match
        assert_eq!(
            mchain.provider.get_code_at(get_rollup_contract_address()).await?,
            Rollup::DEPLOYED_BYTECODE
        );

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
    pub async fn mine_block(&self, block_timestamp_secs: u64) -> Result<BlockHash> {
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
        Ok(block_hash)
    }

    /// Rolls back the chain to a specific block hash by performing a reorg
    pub async fn rollback_to_block(&self, block_hash: BlockHash) -> Result<()> {
        // TODO(SEQ-653): set safe and finalized block hashes properly
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
            PayloadStatus { status: PayloadStatusEnum::Valid, latest_valid_hash: Some(block_hash) }
        );
        Ok(())
    }

    /// Return the on-chain config for a rollup with a given chain id
    pub fn rollup_config(chain_id: u64) -> String {
        let zero = Address::ZERO;
        let mut cfg = format!(
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
        );
        cfg.retain(|c| !c.is_whitespace());
        cfg.shrink_to_fit();
        cfg
    }
}
