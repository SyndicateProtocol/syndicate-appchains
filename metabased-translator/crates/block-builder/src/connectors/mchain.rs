//! Connector for the `MetaChain`
use crate::{
    block_builder::BlockBuilderError, config::BlockBuilderConfig, rollups::shared::RollupAdapter,
};
use alloy::{
    eips::BlockNumberOrTag,
    network::EthereumWallet,
    primitives::Address,
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, RootProvider,
    },
};
use common::{
    eth_client::RPCClient,
    types::{BlockRef, KnownState},
};
use eyre::Result;
use mchain::mchain::MProvider;
use std::sync::Arc;
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
pub struct MetaChainProvider<R: RollupAdapter> {
    pub provider: MProvider,
    pub rollup_adapter: R,
}

impl<R: RollupAdapter> MetaChainProvider<R> {
    /// Create a provider for the `MetaChain`
    pub async fn start(config: &BlockBuilderConfig, rollup_adapter: R) -> Result<Self> {
        let provider = MProvider::new(config.mchain_rpc_url.parse()?);

        let mchain = Self { provider, rollup_adapter };

        Ok(mchain)
    }

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
        if let Some(slot) = self.provider.slot().await {
            info!("checking latest slot: {:?}", slot);
            let mut seq_block =
                BlockRef { number: slot.seq_block_number, hash: slot.seq_block_hash, timestamp: 0 };
            if validate_block_add_timestamp(sequencing_client, &mut seq_block).await {
                let mut set_block = BlockRef {
                    number: slot.set_block_number,
                    hash: slot.set_block_hash,
                    timestamp: 0,
                };
                if validate_block_add_timestamp(settlement_client, &mut set_block).await {
                    let (state, mut mchain_block_number) = self
                        .provider
                        .get_source_chains_processed_blocks(BlockNumberOrTag::Latest)
                        .await?;
                    if state != slot {
                        // block number is 0 if the slot is empty & a corresponding mchain block
                        // does not exist
                        mchain_block_number = 0;

                        // just in case - make sure the slot comes after the latest mchain block
                        assert!(
                            state.seq_block_number < seq_block.number &&
                                state.set_block_number <= set_block.number
                        );
                    }
                    info!("latest slot is OK - no need to reorg");
                    return Ok(Some(KnownState {
                        mchain_block_number,
                        sequencing_block: seq_block,
                        settlement_block: set_block,
                    }));
                }
            }
        }
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
        sequencing_client: &Arc<dyn RPCClient>,
        settlement_client: &Arc<dyn RPCClient>,
    ) -> Option<KnownState> {
        info!("getting safe state");
        let mut current_block = BlockNumberOrTag::Latest;
        loop {
            #[allow(clippy::unwrap_used)]
            let mut state =
                self.rollup_adapter.get_processed_blocks(self, current_block).await.unwrap()?;
            info!("checking state {:?}", state);
            let seq_valid =
                validate_block_add_timestamp(sequencing_client, &mut state.sequencing_block).await;
            let set_valid =
                validate_block_add_timestamp(settlement_client, &mut state.settlement_block).await;
            info!("seq valid: {}, set valid: {}", seq_valid, set_valid);
            if seq_valid && set_valid {
                info!("found safe state {:?}", state);
                return Some(state)
            }
            assert_ne!(state.mchain_block_number, 0, "cannot reorg genesis block");
            current_block = BlockNumberOrTag::Number(state.mchain_block_number - 1);
        }
    }

    /// Asserts that the mchain rollup owner is the same as the translator's configured owner
    pub async fn assert_rollup_owner(
        &self,
        config_rollup_owner: Option<Address>,
    ) -> Result<(), BlockBuilderError> {
        if let Some(config_rollup_owner) = config_rollup_owner {
            let rollup_owner = self.provider.rollup_owner().await;
            if rollup_owner != config_rollup_owner {
                return Err(BlockBuilderError::RollupOwnerMismatch(
                    config_rollup_owner,
                    rollup_owner,
                ));
            }
        }
        Ok(())
    }
}

async fn validate_block_add_timestamp(
    client: &Arc<dyn RPCClient>,
    expected_block: &mut BlockRef,
) -> bool {
    #[allow(clippy::expect_used)]
    let block = client
        .get_block_by_number(BlockNumberOrTag::Number(expected_block.number))
        .await
        .expect("could not find block");
    assert_eq!(block.number, expected_block.number);
    expected_block.timestamp = block.timestamp;
    block.hash == expected_block.hash
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
}
