use crate::db::{MBlock, Slot};
use alloy::{
    eips::BlockNumberOrTag,
    primitives::Address,
    providers::RootProvider,
    rpc::json_rpc::{RpcRecv, RpcSend},
    transports::{RpcError, TransportErrorKind},
};
use jsonrpsee::core::async_trait;
use shared::{eth_client::RPCClient, types::BlockRef};
use std::sync::Arc;
use tracing::info;

/// Known state of the mchain
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KnownState {
    /// The latest block from the sequencing chain that has been processed
    pub sequencing_block: BlockRef,
    /// The latest block from the settlement chain that has been processed
    pub settlement_block: BlockRef,
}

#[async_trait]
#[allow(clippy::unwrap_used)]
pub trait Provider: Send + Sync {
    async fn raw_request<Params: RpcSend, T: RpcRecv + Clone>(
        &self,
        method: &'static str,
        params: Params,
    ) -> Result<T, RpcError<TransportErrorKind>>;

    async fn get_block_number(&self) -> u64 {
        let block: alloy::rpc::types::Block = self
            .raw_request("eth_getBlockByNumber", (BlockNumberOrTag::Latest, false))
            .await
            .unwrap();
        block.header.number
    }
    async fn add_batch(&self, batch: &MBlock) -> eyre::Result<Option<u64>> {
        Ok(self.raw_request("mchain_addBatch", batch).await?)
    }
    async fn get_source_chains_processed_blocks(
        &self,
        tag: BlockNumberOrTag,
    ) -> eyre::Result<(Slot, u64)> {
        Ok(self.raw_request("mchain_getSourceChainsProcessedBlocks", tag).await?)
    }
    async fn rollback_to_block(&self, block_number: u64) -> eyre::Result<()> {
        Ok(self.raw_request("mchain_rollbackToBlock", block_number).await?)
    }
    async fn rollup_owner(&self) -> Address {
        self.raw_request("mchain_rollupOwner", ()).await.unwrap()
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
    async fn reconcile_mchain_with_source_chains(
        &self,
        sequencing_client: &Arc<dyn RPCClient>,
        settlement_client: &Arc<dyn RPCClient>,
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

    /// `get_safe_state` obtains the processed blocks from the rollup contract and validates them
    /// against the source chain clients.
    /// The safe mchain block number is returned if the chain requires a reorg.
    async fn get_safe_state(
        &self,
        sequencing_client: &Arc<dyn RPCClient>,
        settlement_client: &Arc<dyn RPCClient>,
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

#[derive(Debug, Clone)]
pub struct MProvider(RootProvider);

impl MProvider {
    pub fn new(url: &str) -> Result<Self, url::ParseError> {
        Ok(Self(RootProvider::new_http(url.parse()?)))
    }
}

#[async_trait]
impl Provider for MProvider {
    async fn raw_request<Params: RpcSend, T: RpcRecv>(
        &self,
        method: &'static str,
        params: Params,
    ) -> Result<T, RpcError<TransportErrorKind>> {
        alloy::providers::Provider::raw_request(&self.0, method.into(), params).await
    }
}

#[cfg(test)]
mod tests {
    use super::Provider;
    use crate::{
        client::{validate_block_add_timestamp, KnownState},
        db::{tests::TestDB, ArbitrumDB, DelayedMessage, MBlock, Slot},
        metrics::MchainMetrics,
        server::{start_mchain, tests::TIME},
    };
    use alloy::{
        eips::BlockNumberOrTag,
        hex,
        primitives::{Address, Bytes, B256, U256},
        rpc::json_rpc::{self, RpcRecv, RpcSend},
        transports::{RpcError, TransportErrorKind},
    };
    use jsonrpsee::{core::async_trait, RpcModule};
    use shared::{
        eth_client::{RPCClient, RPCClientError},
        types::{Block, BlockRef, Receipt},
    };
    use std::{collections::HashMap, sync::Arc};

    #[ctor::ctor]
    fn init() {
        shared::logger::set_global_default_subscriber();
    }

    #[async_trait]
    impl<X: Send + Sync> Provider for RpcModule<X> {
        async fn raw_request<Params: RpcSend, T: RpcRecv + Clone>(
            &self,
            method: &'static str,
            params: Params,
        ) -> Result<T, RpcError<TransportErrorKind>> {
            let payload = json_rpc::Request::new(method, json_rpc::Id::None, params)
                .serialize()
                .map_err(RpcError::SerError)?
                .take_request();
            let (rp, _) = self
                .raw_json_request(payload.get(), 1)
                .await
                .map_err(|e| RpcError::DeserError { err: e, text: payload.get().to_string() })?;
            match serde_json::from_str::<json_rpc::Response<T>>(&rp)
                .map_err(|e| RpcError::DeserError { err: e, text: rp })?
                .payload
            {
                json_rpc::ResponsePayload::Success(x) => Ok(x),
                json_rpc::ResponsePayload::Failure(x) => Err(RpcError::ErrorResp(x)),
            }
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
        let mchain =
            start_mchain(10, Address::ZERO, 60, db.clone(), MchainMetrics::default()).await;
        Ok((mchain, db))
    }

    // additional provider functions for testing
    trait TestProvider {
        async fn get_finalized_block(&self) -> u64;
    }

    impl<T: Provider> TestProvider for T {
        async fn get_finalized_block(&self) -> u64 {
            let block: alloy::rpc::types::Block = self
                .raw_request("eth_getBlockByNumber", (BlockNumberOrTag::Finalized, false))
                .await
                .unwrap();
            block.header.number
        }
    }

    #[derive(Debug, Clone)]
    struct MockRPCClient(HashMap<u64, (B256, u64)>);

    #[async_trait]
    impl RPCClient for MockRPCClient {
        async fn get_block_by_number(
            &self,
            tag: BlockNumberOrTag,
        ) -> Result<Block, RPCClientError> {
            let number = match tag {
                BlockNumberOrTag::Number(x) => x,
                e => panic!("invalid tag: {}", e),
            };
            let block = self.0.get(&number).unwrap();
            Ok(Block { hash: block.0, timestamp: block.1, number, ..Default::default() })
        }

        async fn get_block_receipts(
            &self,
            _block_number_hex: &str,
        ) -> Result<Vec<Receipt>, RPCClientError> {
            panic!("Not implemented");
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
        let seq_client: Arc<dyn RPCClient> = Arc::new(MockRPCClient(seq_blocks));
        let mut set_blocks = HashMap::new();
        set_blocks.insert(3, (U256::from(4).into(), 6));
        let set_client: Arc<dyn RPCClient> = Arc::new(MockRPCClient(set_blocks));
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
        let seq_client: Arc<dyn RPCClient> = Arc::new(MockRPCClient(seq_blocks));
        let mut set_blocks = HashMap::new();
        set_blocks.insert(3, (U256::from(4).into(), 6));
        set_blocks.insert(30, (U256::from(400).into(), 60));
        let set_client: Arc<dyn RPCClient> = Arc::new(MockRPCClient(set_blocks));
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
        let seq_client: Arc<dyn RPCClient> = Arc::new(MockRPCClient(seq_blocks));
        let mut set_blocks = HashMap::new();
        set_blocks.insert(3, (U256::from(4).into(), 6));
        let set_client: Arc<dyn RPCClient> = Arc::new(MockRPCClient(set_blocks));

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
        let client: Arc<dyn RPCClient> = Arc::new(MockRPCClient(blocks));

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
        let client_mismatch: Arc<dyn RPCClient> = Arc::new(MockRPCClient(blocks));

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
                payload: Some((Default::default(), vec![empty.clone()])),
                slot: Slot { seq_block_number: 2, ..Default::default() },
                timestamp: 0,
            })
            .await?; // block + messages (3)
        assert_eq!(db.0.read().unwrap().keys().len(), 6 + 1);
        assert_eq!(mchain.get_block_number().await, 3);
        mchain
            .add_batch(&MBlock {
                payload: Some((Default::default(), vec![empty; 2])),
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
