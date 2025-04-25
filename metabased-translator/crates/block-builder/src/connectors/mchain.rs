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
        let (safe_state, mchain_block_number) =
            self.get_safe_state(sequencing_client, settlement_client).await;
        if let Some(mchain_block_number) = mchain_block_number {
            let mchain_block_before = self.provider.get_block_number().await;
            self.provider.rollback_to_block(mchain_block_number).await.map_err(|e| {
                BlockBuilderError::ResumeFromBlock(format!("Unable to reorg to block: {}", e))
            })?;
            let mchain_block_after = self.provider.get_block_number().await;
            info!(
                "reconciliation done: mchain_block_before: {}, safe_state: {:?}, mchain_block_after: {}",
                mchain_block_before, safe_state, mchain_block_after
            );
        }
        Ok(safe_state)
    }

    /// `get_safe_state` obtains the processed blocks from the rollup contract and validates them
    /// against the source chain clients.
    /// The safe mchain block number is returned if the chain requires a reorg.
    pub async fn get_safe_state(
        &self,
        sequencing_client: &Arc<dyn RPCClient>,
        settlement_client: &Arc<dyn RPCClient>,
    ) -> (Option<KnownState>, Option<u64>) {
        info!("getting safe state");
        let mut current_block = BlockNumberOrTag::Pending;
        loop {
            #[allow(clippy::unwrap_used)]
            let (slot, mchain_block_number) =
                self.provider.get_source_chains_processed_blocks(current_block).await.unwrap();
            assert_ne!(mchain_block_number, 0, "cannot reorg genesis block");

            let not_pending = current_block != BlockNumberOrTag::Pending;

            if slot.seq_block_number == 0 {
                assert_eq!(slot, Default::default());
                assert_eq!(mchain_block_number, if not_pending { 1 } else { 2 });
                return (None, not_pending.then_some(mchain_block_number))
            }

            info!("checking slot {:?}", slot);
            let mut state = KnownState {
                sequencing_block: BlockRef {
                    number: slot.seq_block_number,
                    timestamp: 0,
                    hash: slot.seq_block_hash,
                },
                settlement_block: BlockRef {
                    number: slot.set_block_number,
                    timestamp: 0,
                    hash: slot.set_block_hash,
                },
            };
            let seq_valid =
                validate_block_add_timestamp(sequencing_client, &mut state.sequencing_block).await;
            let set_valid =
                validate_block_add_timestamp(settlement_client, &mut state.settlement_block).await;
            info!("seq valid: {}, set valid: {}", seq_valid, set_valid);
            if seq_valid && set_valid {
                info!("found safe state {:?} current block {:?}", state, current_block);
                return (Some(state), not_pending.then_some(mchain_block_number))
            }
            current_block = BlockNumberOrTag::Number(mchain_block_number - 1);
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
