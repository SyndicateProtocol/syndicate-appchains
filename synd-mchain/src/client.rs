//! The `synd-mchain` client

use crate::db::{MBlock, Slot};
use alloy::eips::BlockNumberOrTag;
pub use jsonrpsee::core::{traits::ToRpcParams, ClientError};
use jsonrpsee::{
    core::{async_trait, client::ClientT as _},
    ws_client::{WsClient, WsClientBuilder},
};
pub use serde::de::DeserializeOwned;
use shared::types::BlockRef;
use tracing::info;

/// Known state of the synd-mchain
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KnownState {
    /// The latest block from the sequencing chain that has been processed
    pub sequencing_block: BlockRef,
    /// The latest block from the settlement chain that has been processed
    pub settlement_block: BlockRef,
}

/// The trait for the provider of the synd-mchain
#[async_trait]
#[allow(clippy::unwrap_used)]
pub trait Provider: Send + Sync {
    /// Sends a request to the provider with the given method and parameters
    /// Returns the deserialized response of type T
    async fn request<Params: ToRpcParams + Send, T: DeserializeOwned + Clone>(
        &self,
        method: &'static str,
        params: Params,
    ) -> Result<T, ClientError>;

    /// Gets the current block number from the chain
    async fn get_block_number(&self) -> u64 {
        let block: alloy::rpc::types::Block =
            self.request("eth_getBlockByNumber", (BlockNumberOrTag::Latest, false)).await.unwrap();
        block.header.number
    }

    /// Adds a new batch to the chain
    async fn add_batch(&self, batch: &MBlock) -> eyre::Result<Option<u64>> {
        Ok(self.request("mchain_addBatch", [batch]).await?)
    }

    /// Gets the processed blocks from source chains for a given block tag
    async fn get_source_chains_processed_blocks(
        &self,
        tag: BlockNumberOrTag,
    ) -> eyre::Result<(Slot, u64)> {
        Ok(self.request("mchain_getSourceChainsProcessedBlocks", [tag]).await?)
    }

    /// Rolls back the chain to a specific block number
    async fn rollback_to_block(&self, block_number: u64) -> eyre::Result<()> {
        Ok(self.request("mchain_rollbackToBlock", [block_number]).await?)
    }

    /// Reconciles the [`MockChain`] state with the source chains (sequencing and settlement)
    ///
    /// This function is used during application startup and when handling reorgs to ensure
    /// the [`MockChain`]'s state is consistent with the source chains. It:
    /// 1. Retrieves the latest valid state from the appchain contract that can be verified against
    ///    both source chains
    /// 2. Rolls back the [`MockChain`] to this validated state if necessary
    /// 3. Returns the established safe state for the translator to resume from
    ///
    /// # Arguments
    /// * `sequencing_client` - Client for the sequencing chain
    /// * `settlement_client` - Client for the settlement chain
    ///
    /// # Returns
    /// * `Ok(Some(KnownState))` - The validated state if one was found
    /// * `Ok(None)` - No validated state was found (starting from genesis)
    /// * `Err` - An error occurred during reconciliation
    async fn reconcile_mchain_with_source_chains(
        &self,
        sequencing_client: &impl synd_chain_ingestor::client::Provider,
        settlement_client: &impl synd_chain_ingestor::client::Provider,
    ) -> eyre::Result<Option<KnownState>> {
        let (safe_state, mchain_block_number) =
            self.get_safe_state(sequencing_client, settlement_client).await;
        if let Some(mchain_block_number) = mchain_block_number {
            let mchain_block_before = self.get_block_number().await;
            self.rollback_to_block(mchain_block_number).await?;
            let mchain_block_after = self.get_block_number().await;
            info!(
                "reconciliation done: mchain_block_before: {}, safe_state: {:?}, mchain_block_after: {}",
                mchain_block_before, safe_state, mchain_block_after
            );
        }
        Ok(safe_state)
    }

    /// `get_safe_state` obtains the processed blocks from the appchain contract and validates them
    /// against the source chain clients.
    /// The safe synd-mchain block number is returned if the chain requires a reorg.
    async fn get_safe_state(
        &self,
        sequencing_client: &impl synd_chain_ingestor::client::Provider,
        settlement_client: &impl synd_chain_ingestor::client::Provider,
    ) -> (Option<KnownState>, Option<u64>) {
        info!("getting safe state");
        let mut current_block = BlockNumberOrTag::Pending;
        loop {
            #[allow(clippy::unwrap_used)]
            let (slot, mchain_block_number) =
                self.get_source_chains_processed_blocks(current_block).await.unwrap();
            assert_ne!(mchain_block_number, 0, "cannot reorg genesis block");

            let not_pending = current_block != BlockNumberOrTag::Pending;

            if slot.seq_block_number == 0 {
                assert_eq!(slot, Default::default());
                assert_eq!(mchain_block_number, if not_pending { 1 } else { 2 });
                return (None, not_pending.then_some(mchain_block_number));
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
                return (Some(state), not_pending.then_some(mchain_block_number));
            }
            current_block = BlockNumberOrTag::Number(mchain_block_number - 1);
        }
    }
}

async fn validate_block_add_timestamp(
    client: &impl synd_chain_ingestor::client::Provider,
    expected_block: &mut BlockRef,
) -> bool {
    #[allow(clippy::unwrap_used)]
    let block = match client.get_block(expected_block.number).await.unwrap() {
        Some(block) => block,
        None => return false,
    };
    assert_eq!(block.number, expected_block.number);
    expected_block.timestamp = block.timestamp;
    block.hash == expected_block.hash
}

/// The provider for the synd-mchain
#[derive(Debug)]
pub struct MProvider(WsClient);

impl MProvider {
    /// Creates a new provider for the synd-mchain
    pub async fn new(url: &str) -> eyre::Result<Self> {
        Ok(Self(WsClientBuilder::new().build(url).await?))
    }
}

#[async_trait]
impl Provider for MProvider {
    async fn request<Params: ToRpcParams + Send, T: DeserializeOwned>(
        &self,
        method: &'static str,
        params: Params,
    ) -> Result<T, ClientError> {
        self.0.request(method, params).await
    }
}

#[cfg(test)]
mod tests {
    use super::Provider;
    use crate::{
        client::{validate_block_add_timestamp, KnownState},
        db::{tests::TestDB, ArbitrumBatch, ArbitrumDB, DelayedMessage, MBlock, Slot},
        methods::common::test_utils::TIME,
        metrics::MchainMetrics,
        server::start_mchain,
    };
    use alloy::{
        eips::BlockNumberOrTag,
        hex,
        primitives::{Address, Bytes, B256, U256},
    };
    use futures_util::{task, Stream};
    use jsonrpsee::{
        core::{async_trait, traits::ToRpcParams, ClientError},
        MethodsError, RpcModule,
    };
    use serde::de::DeserializeOwned;
    use shared::types::BlockRef;
    use std::{collections::HashMap, marker::PhantomData, pin::Pin, sync::Arc, task::Poll};

    #[ctor::ctor]
    fn init() {
        shared::tracing::setup_global_logging();
    }

    #[async_trait]
    impl<X: Send + Sync> Provider for RpcModule<X> {
        async fn request<Params: ToRpcParams + Send, T: DeserializeOwned + Clone>(
            &self,
            method: &'static str,
            params: Params,
        ) -> Result<T, ClientError> {
            self.call(method, params).await.map_err(|e| match e {
                MethodsError::Parse(e) => ClientError::ParseError(e),
                MethodsError::JsonRpc(e) => ClientError::Call(e),
                MethodsError::InvalidSubscriptionId(_) => ClientError::InvalidSubscriptionId,
            })
        }
    }

    impl ArbitrumDB for Arc<TestDB> {
        fn get<K: AsRef<[u8]>>(&self, key: K) -> Option<Bytes> {
            self.as_ref().get(key)
        }
        fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) {
            self.as_ref().put(key, value)
        }
        fn delete<K: AsRef<[u8]>>(&self, key: K) {
            self.as_ref().delete(key)
        }
    }

    async fn setup() -> eyre::Result<(impl Provider, Arc<TestDB>)> {
        let db = Arc::new(TestDB::new());
        let mchain = start_mchain(10, 60, db.clone(), MchainMetrics::default());
        Ok((mchain, db))
    }

    // additional provider functions for testing
    trait TestProvider {
        async fn get_finalized_block(&self) -> u64;
    }

    impl<T: Provider> TestProvider for T {
        async fn get_finalized_block(&self) -> u64 {
            let block: alloy::rpc::types::Block = self
                .request("eth_getBlockByNumber", (BlockNumberOrTag::Finalized, false))
                .await
                .unwrap();
            block.header.number
        }
    }

    #[derive(Debug, Clone)]
    struct MockRPCClient(HashMap<u64, (B256, u64)>);

    struct PanicStream<Notif> {
        phantom: PhantomData<Notif>,
    }

    impl<Notif> Stream for PanicStream<Notif> {
        type Item = Result<Notif, serde_json::Error>;
        fn poll_next(self: Pin<&mut Self>, _: &mut task::Context<'_>) -> Poll<Option<Self::Item>> {
            panic!("unimplemented");
        }
    }

    #[async_trait]
    impl synd_chain_ingestor::client::Provider for MockRPCClient {
        async fn request<Params: ToRpcParams + Send, T: DeserializeOwned + Clone>(
            &self,
            _: &'static str,
            _: Params,
        ) -> Result<T, ClientError> {
            panic!("unimplemented");
        }

        async fn subscribe<
            Params: ToRpcParams + Send,
            Notif: DeserializeOwned + Send + Unpin + 'static,
        >(
            &self,
            _: &'static str,
            _: Params,
            _: &'static str,
        ) -> Result<PanicStream<Notif>, ClientError> {
            panic!("unimplemented");
        }

        async fn get_block(&self, number: u64) -> Result<Option<BlockRef>, ClientError> {
            let block = self.0.get(&number).unwrap();
            Ok(Some(BlockRef { hash: block.0, timestamp: block.1, number }))
        }
    }

    #[tokio::test]
    async fn reconcile_mchain_with_source_chains() -> eyre::Result<()> {
        // pending block is valid

        let (mchain, _) = setup().await?;
        let slot = Slot {
            seq_block_number: 1,
            seq_block_hash: U256::from(2).into(),
            set_block_number: 3,
            set_block_hash: U256::from(4).into(),
        };
        mchain
            .add_batch(&MBlock {
                timestamp: 10,
                slot: slot.clone(),
                payload: Some(Default::default()),
            })
            .await?;
        let mut seq_blocks = HashMap::new();
        seq_blocks.insert(1, (U256::from(2).into(), 5));
        let seq_client = MockRPCClient(seq_blocks);
        let mut set_blocks = HashMap::new();
        set_blocks.insert(3, (U256::from(4).into(), 6));
        let set_client = MockRPCClient(set_blocks);
        let state = Some(KnownState {
            sequencing_block: BlockRef { number: 1, hash: U256::from(2).into(), timestamp: 5 },
            settlement_block: BlockRef { number: 3, hash: U256::from(4).into(), timestamp: 6 },
        });
        // check safe state
        assert_eq!(mchain.get_safe_state(&seq_client, &set_client).await, (state.clone(), None));
        assert_eq!(
            mchain.get_source_chains_processed_blocks(BlockNumberOrTag::Pending).await?,
            (slot.clone(), 3)
        );
        assert_eq!(
            mchain.get_source_chains_processed_blocks(BlockNumberOrTag::Number(2)).await?,
            (slot.clone(), 2)
        );
        // reconcile
        assert_eq!(
            mchain.reconcile_mchain_with_source_chains(&seq_client, &set_client).await?,
            state
        );
        // check safe state again
        assert_eq!(mchain.get_safe_state(&seq_client, &set_client).await, (state.clone(), None));
        assert_eq!(
            mchain.get_source_chains_processed_blocks(BlockNumberOrTag::Pending).await?,
            (slot.clone(), 3)
        );
        assert_eq!(
            mchain.get_source_chains_processed_blocks(BlockNumberOrTag::Number(2)).await?,
            (slot.clone(), 2)
        );

        let new_slot = Slot {
            seq_block_number: 2,
            seq_block_hash: U256::from(20).into(),
            set_block_number: 30,
            set_block_hash: U256::from(40).into(),
        };

        // latest block is valid

        mchain.add_batch(&MBlock { timestamp: 10, slot: new_slot.clone(), payload: None }).await?;
        let mut seq_blocks = HashMap::new();
        seq_blocks.insert(1, (U256::from(2).into(), 5));
        seq_blocks.insert(2, (U256::from(20).into(), 50));
        let seq_client = MockRPCClient(seq_blocks);
        let mut set_blocks = HashMap::new();
        set_blocks.insert(3, (U256::from(4).into(), 6));
        set_blocks.insert(30, (U256::from(400).into(), 60));
        let set_client = MockRPCClient(set_blocks);
        // check safe state
        assert_eq!(mchain.get_safe_state(&seq_client, &set_client).await, (state.clone(), Some(2)));
        assert_eq!(
            mchain.get_source_chains_processed_blocks(BlockNumberOrTag::Pending).await?,
            (new_slot, 3)
        );
        assert_eq!(
            mchain.get_source_chains_processed_blocks(BlockNumberOrTag::Number(2)).await?,
            (slot.clone(), 2)
        );
        // reconcile
        assert_eq!(
            mchain.reconcile_mchain_with_source_chains(&seq_client, &set_client).await?,
            state
        );
        // check safe state again
        assert_eq!(mchain.get_safe_state(&seq_client, &set_client).await, (state.clone(), None));
        assert_eq!(
            mchain.get_source_chains_processed_blocks(BlockNumberOrTag::Pending).await?,
            (slot.clone(), 3)
        );
        assert_eq!(
            mchain.get_source_chains_processed_blocks(BlockNumberOrTag::Number(2)).await?,
            (slot.clone(), 2)
        );

        // no blocks are valid

        let mut seq_blocks = HashMap::new();
        seq_blocks.insert(1, (U256::from(20).into(), 5));
        let seq_client = MockRPCClient(seq_blocks);
        let mut set_blocks = HashMap::new();
        set_blocks.insert(3, (U256::from(4).into(), 6));
        let set_client = MockRPCClient(set_blocks);

        // check safe state
        assert_eq!(mchain.get_safe_state(&seq_client, &set_client).await, (None, Some(1)));
        assert_eq!(
            mchain.get_source_chains_processed_blocks(BlockNumberOrTag::Pending).await?,
            (slot.clone(), 3)
        );
        assert_eq!(
            mchain.get_source_chains_processed_blocks(BlockNumberOrTag::Number(2)).await?,
            (slot, 2)
        );
        // reconcile
        assert_eq!(
            mchain.reconcile_mchain_with_source_chains(&seq_client, &set_client).await?,
            None
        );
        // check safe state again
        assert_eq!(mchain.get_safe_state(&seq_client, &set_client).await, (None, None));
        assert_eq!(
            mchain.get_source_chains_processed_blocks(BlockNumberOrTag::Pending).await?,
            (Slot::default(), 2)
        );
        assert_eq!(
            mchain.get_source_chains_processed_blocks(BlockNumberOrTag::Number(1)).await?,
            (Slot::default(), 1)
        );
        Ok(())
    }

    #[tokio::test]
    async fn validate_block() {
        let expected_hash = B256::from_slice(&hex!(
            "1234567890123456789012345678901234567890123456789012345678901234"
        ));
        let expected_timestamp = 12345;

        let mut test_block = BlockRef {
            hash: expected_hash,
            number: 1,
            timestamp: 0, // Initial timestamp
        };

        let mut blocks = HashMap::new();
        blocks.insert(1, (expected_hash, expected_timestamp));
        let client = MockRPCClient(blocks);

        assert!(validate_block_add_timestamp(&client, &mut test_block).await);
        assert_eq!(test_block.timestamp, expected_timestamp);

        // Test mismatch case
        let mut blocks = HashMap::new();
        blocks.insert(
            1,
            (
                B256::from_slice(&hex!(
                    "4321432143214321432143214321432143214321432143214321432143214321"
                )),
                expected_timestamp,
            ),
        );
        let client_mismatch = MockRPCClient(blocks);

        let mut test_block = BlockRef { hash: expected_hash, number: 1, timestamp: 0 };

        assert!(!validate_block_add_timestamp(&client_mismatch, &mut test_block).await);
    }

    #[tokio::test]
    async fn rollback_to_block() -> eyre::Result<()> {
        let empty = DelayedMessage {
            kind: 0,
            sender: Address::ZERO,
            data: Default::default(),
            base_fee_l1: Default::default(),
        };
        let (mchain, db) = setup().await?;
        assert_eq!(db.0.read().unwrap().keys().len(), 3 + 1); // init msg, block number, batch (1)
        assert_eq!(mchain.get_block_number().await, 1);
        mchain
            .add_batch(&MBlock {
                slot: Slot { seq_block_number: 1, ..Default::default() },
                payload: Some(Default::default()),
                ..Default::default()
            })
            .await?;
        assert_eq!(db.0.read().unwrap().keys().len(), 4 + 1); // block (2)
        assert_eq!(mchain.get_block_number().await, 2);
        mchain
            .add_batch(&MBlock {
                payload: Some(ArbitrumBatch::new(Default::default(), vec![empty.clone()])),
                slot: Slot { seq_block_number: 2, ..Default::default() },
                timestamp: 0,
            })
            .await?; // block + messages (3)
        assert_eq!(db.0.read().unwrap().keys().len(), 6 + 1);
        assert_eq!(mchain.get_block_number().await, 3);
        mchain
            .add_batch(&MBlock {
                payload: Some(ArbitrumBatch::new(Default::default(), vec![empty; 2])),
                slot: Slot { seq_block_number: 3, ..Default::default() },
                timestamp: 0,
            })
            .await?; // block + 2 messagess (4)
        assert_eq!(db.0.read().unwrap().keys().len(), 9 + 1);
        assert_eq!(mchain.get_block_number().await, 4);
        mchain.rollback_to_block(2).await?; // rm 2 blocks + 3 messages
        assert_eq!(db.0.read().unwrap().keys().len(), 4 + 1);
        assert_eq!(mchain.get_block_number().await, 2);
        mchain.rollback_to_block(1).await?; // rm block
        assert_eq!(db.0.read().unwrap().keys().len(), 3 + 1);
        assert_eq!(mchain.get_block_number().await, 1);
        assert!(mchain.rollback_to_block(0).await.is_err());
        assert!(mchain.rollback_to_block(2).await.is_err());
        assert_eq!(db.0.read().unwrap().keys().len(), 3 + 1);
        assert_eq!(mchain.get_block_number().await, 1);
        Ok(())
    }

    #[tokio::test]
    async fn finality() -> eyre::Result<()> {
        let (mchain, _) = setup().await?;
        assert_eq!(mchain.get_block_number().await, 1);
        assert_eq!(mchain.get_finalized_block().await, 1);
        mchain
            .add_batch(&MBlock {
                slot: Slot { seq_block_number: 1, ..Default::default() },
                payload: Some(Default::default()),
                ..Default::default()
            })
            .await?;
        mchain
            .add_batch(&MBlock {
                timestamp: 1,
                slot: Slot { seq_block_number: 2, ..Default::default() },
                payload: Some(Default::default()),
            })
            .await?;
        mchain
            .add_batch(&MBlock {
                timestamp: 1,
                slot: Slot { seq_block_number: 3, ..Default::default() },
                payload: Some(Default::default()),
            })
            .await?;
        assert_eq!(mchain.get_block_number().await, 4);
        assert_eq!(mchain.get_finalized_block().await, 1);
        TIME.set(59);
        assert_eq!(mchain.get_block_number().await, 4);
        assert_eq!(mchain.get_finalized_block().await, 1);
        TIME.set(60);
        assert_eq!(mchain.get_block_number().await, 4);
        assert_eq!(mchain.get_finalized_block().await, 2);
        TIME.set(61);
        assert_eq!(mchain.get_block_number().await, 4);
        assert_eq!(mchain.get_finalized_block().await, 4);
        mchain.rollback_to_block(3).await?;
        assert_eq!(mchain.get_block_number().await, 3);
        assert_eq!(mchain.get_finalized_block().await, 3);
        mchain
            .add_batch(&MBlock {
                timestamp: 1,
                slot: Slot { seq_block_number: 3, ..Default::default() },
                payload: Some(Default::default()),
            })
            .await?;
        assert_eq!(mchain.get_block_number().await, 4);
        assert_eq!(mchain.get_finalized_block().await, 4);
        mchain
            .add_batch(&MBlock {
                timestamp: 2,
                slot: Slot { seq_block_number: 4, ..Default::default() },
                payload: Some(Default::default()),
            })
            .await?;
        assert_eq!(mchain.get_block_number().await, 5);
        assert_eq!(mchain.get_finalized_block().await, 4);
        mchain.rollback_to_block(4).await?;
        TIME.set(62);
        assert_eq!(mchain.get_block_number().await, 4);
        assert_eq!(mchain.get_finalized_block().await, 4);
        mchain
            .add_batch(&MBlock {
                timestamp: 100,
                slot: Slot { seq_block_number: 4, ..Default::default() },
                payload: Some(Default::default()),
            })
            .await?;
        assert_eq!(mchain.get_block_number().await, 5);
        assert_eq!(mchain.get_finalized_block().await, 4);
        TIME.set(0);
        assert_eq!(mchain.get_block_number().await, 5);
        assert_eq!(mchain.get_finalized_block().await, 4);
        mchain.rollback_to_block(2).await?;
        assert_eq!(mchain.get_block_number().await, 2);
        assert_eq!(mchain.get_finalized_block().await, 2);
        TIME.set(1000);
        assert_eq!(mchain.get_block_number().await, 2);
        assert_eq!(mchain.get_finalized_block().await, 2);
        Ok(())
    }
}
