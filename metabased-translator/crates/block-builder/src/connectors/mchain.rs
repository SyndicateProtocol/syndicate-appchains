//! Connector for the `MetaChain`
use crate::{
    block_builder::BlockBuilderError,
    config::{get_default_private_key_signer, get_rollup_contract_address, BlockBuilderConfig},
    metrics::BlockBuilderMetrics,
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
use common::{
    eth_client::RPCClient,
    types::{BlockRef, KnownState},
};
use contract_bindings::arbitrum::rollup::Rollup::{self, RollupInstance};
use eyre::Result;
use std::sync::Arc;
use tracing::{debug, error, info};

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
pub struct MetaChainProvider<R: RollupAdapter> {
    pub mchain_ipc_path: String,
    pub provider: FilledProvider,
    auth_provider: FilledProvider,

    pub rollup_adapter: R,
    pub mine_empty_blocks: bool,

    pub metrics: BlockBuilderMetrics,
}

/// The chain id of the metachain. This is the same for all rollups.
/// TODO(SEQ-652): this should be configurable
pub const MCHAIN_ID: u64 = 84532;

impl<R: RollupAdapter> MetaChainProvider<R> {
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
    pub async fn start(
        config: &BlockBuilderConfig,
        metrics: BlockBuilderMetrics,
        rollup_adapter: R,
    ) -> Result<Self> {
        let provider = ProviderBuilder::new()
            .wallet(EthereumWallet::from(get_default_private_key_signer()))
            .on_ipc(IpcConnect::new(config.mchain_ipc_path.clone()))
            .await?;
        let auth_provider = ProviderBuilder::new()
            .wallet(EthereumWallet::from(get_default_private_key_signer()))
            .on_ipc(IpcConnect::new(config.mchain_auth_ipc_path.clone()))
            .await?;
        let rollup_config = rollup_config(config.target_chain_id, config.rollup_owner_address);

        let mchain = Self {
            mchain_ipc_path: config.mchain_ipc_path.clone(),
            provider,
            auth_provider,
            rollup_adapter,
            mine_empty_blocks: config.mine_empty_blocks,
            metrics,
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
                .gas_limit(15000000)
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
        self.metrics
            .mchain_metrics
            .record_last_block_mined(block.header.number + 1, block_timestamp_secs);
        debug!("mined mchain block: #{} hash: {}", block.header.number + 1, block_hash);
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

    // TODO (SEQ-681) - re-use this function in case of reorg
    /// Reconciles the [`MetaChain`] state with the source chains (sequencing and settlement)
    ///
    /// This function is used during application startup and when handling reorgs to ensure
    /// the [`MetaChain`]'s state is consistent with the source chains. It:
    /// 1. Retrieves the latest valid state from the rollup contract that can be verified against
    ///    both source chains
    /// 2. Rolls back the [`MetaChain`] to this validated state if necessary
    /// 3. Returns the established safe state for the translator to resume from
    ///
    /// # Arguments
    /// * `sequencing_client` - Client for the sequencing chain
    /// * `settlement_client` - Client for the settlement chain
    /// * `rollup_adapter` - Adapter for interacting with the rollup contract
    ///
    /// # Returns
    /// * `Ok(Some(KnownState))` - The validated state if one was found
    /// * `Ok(None)` - No validated state was found (starting from genesis)
    /// * `Err` - An error occurred during reconciliation
    pub async fn reconcile_mchain_with_source_chains(
        &self,
        sequencing_client: &Arc<dyn RPCClient>,
        settlement_client: &Arc<dyn RPCClient>,
    ) -> Result<Option<KnownState>> {
        let mchain_block_before = self
            .get_block_by_number(BlockNumberOrTag::Latest, BlockTransactionsKind::Hashes)
            .await?
            .unwrap_or_default();

        let safe_state = self.get_safe_state(sequencing_client, settlement_client).await?;
        self.rollback_to_safe_state(&safe_state).await?;

        let mchain_block_after = self
            .get_block_by_number(BlockNumberOrTag::Latest, BlockTransactionsKind::Hashes)
            .await?
            .unwrap_or_default();
        info!(
            "reconciliation done: mchain_block_before: #{} hash: {}, safe_state: {:?}, mchain_block_after: #{} hash: {}",
            mchain_block_before.header.number, mchain_block_before.header.hash, safe_state, mchain_block_after.header.number, mchain_block_after.header.hash
        );
        Ok(safe_state)
    }

    /// Validates and rolls back to a known block number if necessary
    async fn rollback_to_safe_state(
        &self,
        known_safe_state: &Option<KnownState>,
    ) -> Result<(), BlockBuilderError> {
        let known_block_number = match known_safe_state {
            Some(known_state) => known_state.mchain_block_number,
            None => {
                info!("No known block number to resume from, starting from genesis");
                return Ok(());
            }
        };

        let current_block_number = self
            .provider
            .get_block_number()
            .await
            .map_err(|e| BlockBuilderError::GetCurrentBlockNumber(e.to_string()))?;

        if known_block_number > current_block_number {
            return Err(BlockBuilderError::KnownBlockNumberGreaterThanCurrentBlockNumber(
                known_block_number,
                current_block_number,
            ));
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
        Ok(())
    }

    /// `get_safe_state` obtains the processed blocks from the rollup contract and validates them
    /// against the source chain clients
    pub async fn get_safe_state(
        &self,
        sequencing_client: &Arc<dyn RPCClient>,
        settlement_client: &Arc<dyn RPCClient>,
    ) -> Result<Option<KnownState>> {
        let mut current_block = BlockNumberOrTag::Latest;
        // NOTE: in case there has been a settlement reorg, we need to restart from the [FOUND
        // STATE] that matches the world minus 1 this is because after a reorg, an incoming
        // block might still fit the the latest found state that matches the observable world
        let mut found_block = false;
        loop {
            match self.rollup_adapter.get_processed_blocks(&self.provider, current_block).await? {
                Some((mut state, block_number)) => {
                    let seq_valid = validate_block_add_timestamp(
                        sequencing_client,
                        &mut state.sequencing_block,
                    )
                    .await;
                    let settle_valid = validate_block_add_timestamp(
                        settlement_client,
                        &mut state.settlement_block,
                    )
                    .await;

                    if found_block {
                        return Ok(Some(state));
                    }

                    if seq_valid && settle_valid {
                        found_block = true;
                    }
                    current_block = BlockNumberOrTag::Number(block_number.saturating_sub(1));
                }
                None => {
                    if found_block &&
                        (current_block == BlockNumberOrTag::Number(1) ||
                            current_block == BlockNumberOrTag::Number(0))
                    {
                        error!("reorging to genesis not supported");
                        panic!("reorging to genesis not supported");
                    }
                    return Ok(None);
                }
            };
        }
    }
}

async fn validate_block_add_timestamp(
    client: &Arc<dyn RPCClient>,
    expected_block: &mut BlockRef,
) -> bool {
    match client.get_block_by_number(BlockNumberOrTag::Number(expected_block.number)).await {
        Ok(block) => {
            expected_block.timestamp = block.timestamp;
            block.hash == expected_block.hash
        }
        Err(_) => false,
    }
}

/// Return the on-chain config for a rollup with a given chain id
pub fn rollup_config(chain_id: u64, chain_owner: Address) -> String {
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
            "InitialChainOwner": "{chain_owner}",
            "GenesisBlockNum": 0
            }}
        }}"#
    );
    cfg.retain(|c| !c.is_whitespace());
    cfg.shrink_to_fit();
    cfg
}

/// Calculate the required gas limit for a transaction based on its input data
fn calculate_tx_gas_limit(input_data: &[u8]) -> u64 {
    let data_gas: u64 = input_data.iter().map(|&byte| if byte == 0 { 4 } else { 16 }).sum();
    (BASE_TRANSACTION_GAS + data_gas) * 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{hex, B256};
    use async_trait::async_trait;
    use common::{
        eth_client::{RPCClient, RPCClientError},
        types::{Block, BlockAndReceipts},
    };
    use std::time::Duration;

    #[derive(Debug, Clone)]
    struct MockRPCClient {
        block_hash: B256,
        timestamp: u64,
    }

    #[async_trait]
    impl RPCClient for MockRPCClient {
        async fn get_block_by_number(&self, _: BlockNumberOrTag) -> Result<Block, RPCClientError> {
            Ok(Block {
                hash: self.block_hash,
                timestamp: self.timestamp,
                number: 1,
                ..Default::default()
            })
        }

        async fn batch_get_blocks_and_receipts(
            &self,
            _block_numbers: Vec<u64>,
        ) -> Result<(Vec<BlockAndReceipts>, Duration), RPCClientError> {
            unimplemented!("Not needed for this test")
        }
    }
    #[tokio::test]
    async fn test_validate_block() {
        let expected_hash = B256::from_slice(&hex!(
            "1234567890123456789012345678901234567890123456789012345678901234"
        ));
        let expected_timestamp = 12345;

        let mut test_block = BlockRef {
            hash: expected_hash,
            number: 1,
            timestamp: 0, // Initial timestamp
        };

        let client: Arc<dyn RPCClient> =
            Arc::new(MockRPCClient { block_hash: expected_hash, timestamp: expected_timestamp });

        assert!(validate_block_add_timestamp(&client, &mut test_block).await);
        assert_eq!(test_block.timestamp, expected_timestamp);

        // Test mismatch case
        let client_mismatch: Arc<dyn RPCClient> = Arc::new(MockRPCClient {
            block_hash: B256::from_slice(&hex!(
                "4321432143214321432143214321432143214321432143214321432143214321"
            )),
            timestamp: expected_timestamp,
        });

        let mut test_block = BlockRef { hash: expected_hash, number: 1, timestamp: 0 };

        assert!(!validate_block_add_timestamp(&client_mismatch, &mut test_block).await);
    }

    #[test]
    fn test_calculate_gas_limit() {
        // Test empty input
        assert_eq!(calculate_tx_gas_limit(&[]), BASE_TRANSACTION_GAS * 2);

        // Test with some zero bytes
        assert_eq!(calculate_tx_gas_limit(&[0, 0, 0]), (BASE_TRANSACTION_GAS + (3 * 4)) * 2);

        // Test with some non-zero bytes
        assert_eq!(calculate_tx_gas_limit(&[1, 2, 3]), (BASE_TRANSACTION_GAS + (3 * 16)) * 2);

        // Test with mixed zero and non-zero bytes
        assert_eq!(
            calculate_tx_gas_limit(&[0, 1, 0, 2, 0]),
            (BASE_TRANSACTION_GAS + (3 * 4) + (2 * 16)) * 2
        );
    }
}
