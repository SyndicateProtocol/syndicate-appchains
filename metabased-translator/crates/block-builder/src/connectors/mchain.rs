//! Connector for the `MetaChain`
use crate::{
    block_builder::BlockBuilderError, config::BlockBuilderConfig, metrics::BlockBuilderMetrics,
    rollups::shared::RollupAdapter,
};
use alloy::{
    eips::BlockNumberOrTag,
    network::EthereumWallet,
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, RootProvider,
    },
};
use common::{
    eth_client::Client,
    types::{BlockRef, KnownState},
};
use eyre::Result;
use mchain::mchain::MProvider;
use tracing::{error, info};

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
    pub provider: MProvider,

    pub rollup_adapter: R,

    pub metrics: BlockBuilderMetrics,
}

impl<R: RollupAdapter> MetaChainProvider<R> {
    /// Create a provider for the `MetaChain`
    pub async fn start(
        config: &BlockBuilderConfig,
        metrics: BlockBuilderMetrics,
        rollup_adapter: R,
    ) -> Result<Self> {
        let provider = MProvider::new(config.mchain_rpc_url.parse()?);

        let mchain = Self { provider, rollup_adapter, metrics };

        Ok(mchain)
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
        sequencing_client: &Client,
        settlement_client: &Client,
    ) -> Result<Option<KnownState>> {
        let mchain_block_before = self.provider.get_block_number().await;

        let safe_state = self.get_safe_state(sequencing_client, settlement_client).await;
        self.rollback_to_safe_state(&safe_state).await?;

        let mchain_block_after = self.provider.get_block_number().await;
        info!(
            "reconciliation done: mchain_block_before: {}, safe_state: {:?}, mchain_block_after: {}",
            mchain_block_before, safe_state, mchain_block_after
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

        let current_block_number = self.provider.get_block_number().await;

        if known_block_number > current_block_number {
            return Err(BlockBuilderError::KnownBlockNumberGreaterThanCurrentBlockNumber(
                known_block_number,
                current_block_number,
            ));
        }

        // rollback to block if necessary
        if known_block_number < current_block_number {
            self.provider.rollback_to_block(known_block_number).await.map_err(|e| {
                BlockBuilderError::ResumeFromBlock(format!("Unable to reorg to block: {}", e))
            })?;
        }
        Ok(())
    }

    /// `get_safe_state` obtains the processed blocks from the rollup contract and validates them
    /// against the source chain clients
    pub async fn get_safe_state(
        &self,
        sequencing_client: &Client,
        settlement_client: &Client,
    ) -> Option<KnownState> {
        let mut current_block = BlockNumberOrTag::Latest;
        // NOTE: in case there has been a settlement reorg, we need to restart from the [FOUND
        // STATE] that matches the world minus 1 this is because after a reorg, an incoming
        // block might still fit the the latest found state that matches the observable world
        let mut found_block = false;
        loop {
            #[allow(clippy::unwrap_used)]
            match self.rollup_adapter.get_processed_blocks(self, current_block).await.unwrap() {
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
                        return Some(state);
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
                    return None;
                }
            };
        }
    }
}

async fn validate_block_add_timestamp(client: &Client, expected_block: &mut BlockRef) -> bool {
    match client.get_block_by_number(BlockNumberOrTag::Number(expected_block.number)).await {
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
    use async_trait::async_trait;
    use common::{
        eth_client::{RPCClient, RPCClientError},
        types::{Block, Receipt},
    };
    use std::sync::Arc;

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

        async fn get_block_receipts(
            &self,
            _block_number_hex: &str,
        ) -> Result<Vec<Receipt>, RPCClientError> {
            panic!("Not implemented");
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
        let client: Client = Client::Http(client);

        assert!(validate_block_add_timestamp(&client, &mut test_block).await);
        assert_eq!(test_block.timestamp, expected_timestamp);

        // Test mismatch case
        let client_mismatch: Arc<dyn RPCClient> = Arc::new(MockRPCClient {
            block_hash: B256::from_slice(&hex!(
                "4321432143214321432143214321432143214321432143214321432143214321"
            )),
            timestamp: expected_timestamp,
        });
        let client_mismatch: Client = Client::Http(client_mismatch);

        let mut test_block = BlockRef { hash: expected_hash, number: 1, timestamp: 0 };

        assert!(!validate_block_add_timestamp(&client_mismatch, &mut test_block).await);
    }
}
