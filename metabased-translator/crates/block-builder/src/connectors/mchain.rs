//! Connector for the `MetaChain`
use crate::{
    block_builder::BlockBuilderError,
    config::{get_default_private_key_signer, get_rollup_contract_address, BlockBuilderConfig},
    connectors::metrics::MChainMetrics,
    rollups::shared::RollupAdapter,
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
        BlockTransactionsKind, TransactionRequest,
    },
};
use common::types::{Block, BlockRef, KnownState};
use contract_bindings::arbitrum::rollup::Rollup::{self, RollupInstance};
use eyre::{eyre, Result};
use std::future::Future;
use tracing::info;

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
    pub provider: FilledProvider,
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
    ) -> Result<Option<alloy::rpc::types::Block>> {
        self.provider.get_block_by_number(number, kind).await.map_err(|e| e.into())
    }

    #[allow(missing_docs)]
    pub async fn get_block_by_hash(
        &self,
        hash: BlockHash,
        kind: BlockTransactionsKind,
    ) -> Result<Option<alloy::rpc::types::Block>> {
        self.provider.get_block_by_hash(hash, kind).await.map_err(|e| e.into())
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
                    safe_block_hash: Default::default(),
                    finalized_block_hash: Default::default(),
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
                    safe_block_hash: Default::default(),
                    finalized_block_hash: Default::default(),
                },
                None,
            )
            .await?;
        assert_eq!(
            fcu.payload_status,
            PayloadStatus { status: PayloadStatusEnum::Valid, latest_valid_hash: Some(block_hash) }
        );
        self.metrics.record_last_block_mined(block.header.number + 1, block_timestamp_secs);
        Ok(block_hash)
    }

    /*
    fn into_payload(b: Block) -> ExecutionPayloadV3 {
        ExecutionPayloadV3::from_block_unchecked(
            b.header.hash,
            &alloy::consensus::BlockBody {
                transactions: b
                    .transactions
                    .into_transactions_vec()
                    .into_iter()
                    .map(|t| t.inner)
                    .collect(),
                ommers: vec![],
                withdrawals: b.withdrawals,
            }
            .into_block(b.header),
        )
    }
    */

    /// Rolls back the chain to a block by performing a reorg
    pub async fn rollback_to_block(&self, block_hash: BlockHash) -> Result<()> {
        // TODO(SEQ-653): set safe and finalized block hashes
        let fcu = self
            .auth_provider
            .fork_choice_updated_v3(
                ForkchoiceState {
                    head_block_hash: block_hash,
                    safe_block_hash: Default::default(),
                    finalized_block_hash: Default::default(),
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

    /// TODO write me and find a better name for this func
    // TODO (SEQ-651) - re-use this function in case of reorg
    pub async fn start_from_safe_state<F1, F2, Fut1, Fut2>(
        &self,
        sequencing_client: F1,
        settlement_client: F2,
        rollup_adapter: &impl RollupAdapter,
    ) -> Result<Option<KnownState>>
    where
        F1: Fn(u64) -> Fut1 + Send + Sync,
        F2: Fn(u64) -> Fut2 + Send + Sync,
        Fut1: Future<Output = Result<Block>> + Send,
        Fut2: Future<Output = Result<Block>> + Send,
    {
        let safe_state =
            self.get_safe_state(sequencing_client, settlement_client, rollup_adapter).await?;
        self.rollback_to_safe_state(&safe_state).await?;
        Ok(safe_state)
    }

    // TODO find a better name for this func
    /// Validates and rolls back to a known block number if necessary
    async fn rollback_to_safe_state(&self, known_safe_state: &Option<KnownState>) -> Result<()> {
        let known_block_number = match known_safe_state {
            Some(known_state) => known_state.mchain_block_number,
            None => {
                info!("No known block number to resume from, starting from genesis");
                return Ok(())
            }
        };

        let current_block_number = self
            .provider
            .get_block_number()
            .await
            .map_err(|e| eyre!(format!("Error getting current block number: {}", e)))?;

        if known_block_number > current_block_number {
            return Err(eyre!(format!(
                "Known block(slot) number {} is greater than the current mchain block number {}",
                known_block_number, current_block_number
            )));
        }

        // rollback to block if necessary
        if known_block_number < current_block_number {
            let block = self
                .provider
                .get_block_by_number(
                    BlockNumberOrTag::Number(known_block_number),
                    BlockTransactionsKind::Hashes,
                )
                .await
                .map_err(|e| BlockBuilderError::ResumeFromBlock(e.to_string()))?
                .ok_or(BlockBuilderError::ResumeFromBlock(format!(
                    "Could not find block: {}",
                    known_block_number
                )))?;
            self.rollback_to_block(block.header.hash).await.map_err(|e| {
                BlockBuilderError::ResumeFromBlock(format!("Unable to reorg to block: {}", e))
            })?;
        }
        info!("Resumed from block: {}", known_block_number);
        Ok(())
    }

    /// Get obtains the processed blocks from the rollup contract and validates them against the
    /// source chain clients
    pub async fn get_safe_state<F1, F2, Fut1, Fut2>(
        &self,
        sequencing_client: F1,
        settlement_client: F2,
        rollup_adapter: &impl RollupAdapter,
    ) -> Result<Option<KnownState>>
    where
        F1: Fn(u64) -> Fut1 + Send + Sync,
        F2: Fn(u64) -> Fut2 + Send + Sync,
        Fut1: Future<Output = Result<Block>> + Send,
        Fut2: Future<Output = Result<Block>> + Send,
    {
        let mut current_block = BlockNumberOrTag::Latest;
        loop {
            match rollup_adapter.get_processed_blocks(&self.provider, current_block).await? {
                Some((mut state, block_number)) => {
                    let seq_valid = validate_block_add_timestamp(
                        &sequencing_client,
                        &mut state.sequencing_block,
                    )
                    .await;
                    let settle_valid = validate_block_add_timestamp(
                        &settlement_client,
                        &mut state.settlement_block,
                    )
                    .await;

                    if seq_valid && settle_valid {
                        return Ok(Some(state));
                    }
                    current_block = BlockNumberOrTag::Number(block_number.saturating_sub(1));
                }
                None => return Ok(None),
            };
        }
    }
}

async fn validate_block_add_timestamp<F, Fut>(get_block: &F, expected_block: &mut BlockRef) -> bool
where
    F: Fn(u64) -> Fut + Send + Sync,
    Fut: Future<Output = Result<Block>> + Send,
{
    match get_block(expected_block.number).await {
        Ok(block) => {
            expected_block.timestamp = block.timestamp;
            block.hash == expected_block.hash
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{hex, B256};

    #[tokio::test]
    async fn test_validate_block() {
        let expected_hash = B256::from_slice(&hex!(
            "1234567890123456789012345678901234567890123456789012345678901234"
        ));
        let expected_timestamp = 12345;

        let mut test_block = BlockRef { hash: expected_hash, number: 1, timestamp: 0 };

        let get_block = |_: u64| async move {
            Ok(Block { hash: expected_hash, timestamp: expected_timestamp, ..Default::default() })
        };

        assert!(validate_block_add_timestamp(&get_block, &mut test_block).await);
        assert_eq!(test_block.timestamp, expected_timestamp);

        // Test mismatch case
        let get_block_mismatch = |_: u64| async move {
            Ok(Block {
                hash: B256::from_slice(&hex!(
                    "4321432143214321432143214321432143214321432143214321432143214321"
                )),
                timestamp: expected_timestamp,
                ..Default::default()
            })
        };

        let mut test_block = BlockRef { hash: expected_hash, number: 1, timestamp: 0 };
        assert!(!validate_block_add_timestamp(&get_block_mismatch, &mut test_block).await);
    }
}
