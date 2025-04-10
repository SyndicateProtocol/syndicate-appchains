use crate::{
    db::{to_err, ArbitrumDB as _, DelayedMessage, MBlock, KVDB},
    metrics::MchainMetrics,
};
use alloy::{
    eips::BlockNumberOrTag,
    primitives::{address, keccak256, Address, Bytes, FixedBytes, U256},
    providers::{Provider as _, ProviderBuilder, RootProvider},
    rpc::types::{FilterBlockOption, TransactionRequest},
    sol_types::{SolCall, SolEvent as _, SolValue as _},
};
use contract_bindings::arbitrum::{
    ibridge::IBridge,
    iinbox::IInbox,
    isequencerinbox::{self, ISequencerInbox},
};
use jsonrpsee::{
    types::{error::INTERNAL_ERROR_CODE, ErrorObjectOwned},
    RpcModule,
};
#[cfg(not(test))]
use std::time::SystemTime;
use std::{collections::VecDeque, sync::Mutex, time::UNIX_EPOCH};
#[cfg(test)]
use tests::SystemTime;
use tracing::error;

/// The chain id of the metachain. This is the same for all rollups.
/// TODO(SEQ-652): this should be configurable
pub const MCHAIN_ID: u64 = 84532;

pub const ROLLUP: Address = address!("0x5FbDB2315678afecb367f032d93F642f64180aa3");

fn err(message: &'static str) -> ErrorObjectOwned {
    ErrorObjectOwned::borrowed(INTERNAL_ERROR_CODE, message, None)
}

fn create_log(block_num: u64, data: alloy::primitives::LogData) -> alloy::rpc::types::Log {
    alloy::rpc::types::Log {
        inner: alloy::primitives::Log { address: ROLLUP, data },
        transaction_hash: Some(FixedBytes::ZERO),
        block_number: Some(block_num),
        block_hash: Some(U256::from(block_num).into()),
        ..Default::default()
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

#[derive(Debug, Clone)]
pub struct MProvider(RootProvider);

impl MProvider {
    pub fn new(url: alloy::transports::http::reqwest::Url) -> Self {
        Self(ProviderBuilder::default().on_http(url))
    }
    pub async fn get_block_number(&self) -> u64 {
        #[allow(clippy::expect_used)]
        self.0
            .get_block_by_number(BlockNumberOrTag::Latest)
            .await
            .expect("get_block_number failed")
            .expect("latest block not found")
            .header
            .number
    }
    pub async fn add_batch(&self, batch: MBlock) -> eyre::Result<u64> {
        Ok(self.0.raw_request("mchain_addBatch".into(), (batch,)).await?)
    }
    pub async fn get_source_chains_processed_blocks(
        &self,
        tag: BlockNumberOrTag,
    ) -> eyre::Result<(u64, FixedBytes<32>, u64, FixedBytes<32>, u64)> {
        Ok(self.0.raw_request("mchain_getSourceChainsProcessedBlocks".into(), tag).await?)
    }
    pub async fn rollback_to_block(&self, block_number: u64) -> eyre::Result<()> {
        Ok(self.0.raw_request("mchain_rollbackToBlock".into(), (block_number,)).await?)
    }
}

#[allow(clippy::unwrap_used)]
pub async fn start_mchain<T: KVDB + Send + Sync + 'static>(
    chain_id: u64,
    chain_owner: Address,
    finality_delay: u64,
    db: T,
    metrics: MchainMetrics,
) -> RpcModule<(T, MchainMetrics, Mutex<(u64, VecDeque<u64>)>)> {
    let init_msg = DelayedMessage {
        kind: 11, // L1MessageType::Initialize
        sender: Address::ZERO,
        data: (
            U256::from(chain_id),
            [1u8],      // initMsgVersion
            U256::ZERO, // currentDataCost
            rollup_config(chain_id, chain_owner),
        )
            .abi_encode_packed()
            .into(),
        base_fee_l1: U256::ZERO,
    };
    let mut pending_ts: VecDeque<u64> = Default::default();
    let mut finalized_block = 1u64;
    if db.get_block_number() == 0 {
        // 000b00800203 corresponds to a batch containing a single delayed message
        db.add_batch(MBlock {
            timestamp: 0,
            messages: vec![init_msg],
            batch: alloy::hex!("000b00800203").into(),
            seq_block_hash: FixedBytes::ZERO,
            seq_block_number: 0,
            set_block_hash: FixedBytes::ZERO,
            set_block_number: 0,
        })
        .unwrap();
    } else {
        let db_init = &db.get_block(1).unwrap().messages[0].0;
        if db_init != &init_msg {
            error!("init message does not match - are the cli arguments correct?");
            assert_eq!(db_init, &init_msg);
        }
        // search for the finalized head. store unfinalized timestamps in a queue.
        finalized_block = db.get_block_number();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        while finalized_block > 1 {
            let block_ts = db.get_block(finalized_block).unwrap().timestamp;
            if block_ts + finality_delay <= now {
                break;
            }
            finalized_block -= 1;
            pending_ts.push_front(block_ts);
        }
    }
    assert_eq!(finalized_block + pending_ts.len() as u64, db.get_block_number());
    let mut module = RpcModule::new((db, metrics, Mutex::new((finalized_block, pending_ts))));

    // -------------------------------------------------
    // mchain methods
    // -------------------------------------------------
    module
        .register_method(
            "mchain_addBatch",
            move |p, (db, metrics, mutex), _| -> Result<u64, ErrorObjectOwned> {
                let (batch,): (MBlock,) = p.parse()?;
                let timestamp = batch.timestamp;
                let block = db.add_batch(batch)?;
                metrics.record_last_block(block, timestamp);
                let mut data = mutex.lock().unwrap();
                data.1.push_back(timestamp);
                assert_eq!(data.0 + data.1.len() as u64, block);
                drop(data);
                Ok(block)
            },
        )
        .unwrap();
    module
        .register_method(
            "mchain_rollbackToBlock",
            move |p, (db, metrics, mutex), _| -> Result<(), ErrorObjectOwned> {
                let (block,): (u64,) = p.parse()?;
                let latest = db.get_block_number();
                if block == latest {
                    return Ok(());
                }
                if block > latest {
                    return Err(err("cannot set head past the last block"));
                }
                if block == 0 {
                    return Err(err("cannot set head before the first block"));
                }
                // remove stale ts data & delete blocks, messages
                db.put_block_number(block);
                let start_block = db.get_block(block)?;
                metrics.record_reorg(block, latest, start_block.timestamp);
                let end_block = db.get_block(latest)?;
                for i in block..latest {
                    db.delete_block(i + 1);
                }
                for i in start_block.after_message_count()..end_block.after_message_count() {
                    db.delete_message_acc(i);
                }
                let mut data = mutex.lock().unwrap();
                if block < data.0 {
                    metrics.record_finalized_block(block, start_block.timestamp);
                    data.0 = block;
                    data.1.clear();
                } else {
                    let removed = (latest - block) as usize;
                    let data_len = data.1.len();
                    assert!(data_len >= removed);
                    data.1.truncate(data_len - removed);
                }
                assert_eq!(data.0 + data.1.len() as u64, block);
                drop(data);
                Ok(())
            },
        )
        .unwrap();
    module.register_method(
        "mchain_getSourceChainsProcessedBlocks",
        move |p,
              (db, _, _),
              _|
              -> Result<(u64, FixedBytes<32>, u64, FixedBytes<32>, u64), ErrorObjectOwned> {
            let tag: BlockNumberOrTag = p.parse()?;
            let block_num = match tag {
                BlockNumberOrTag::Latest => db.get_block_number(),
                BlockNumberOrTag::Number(i) => i,
                _ => return Err(to_err(format!("unexpected block tag: {}", tag))),
            };
            let block = db.get_block(block_num)?;
            Ok((
                block.seq_block_number,
                block.seq_block_hash,
                block.set_block_number,
                block.set_block_hash,
                block_num,
            ))
        },
    ).unwrap();

    // -------------------------------------------------
    // eth methods
    // -------------------------------------------------
    module.register_method("eth_chainId", move |_, _, _| format!("{:#x}", MCHAIN_ID)).unwrap();
    module.register_method("eth_getCode", move |_, _, _| "0x").unwrap();
    module
        .register_method(
            "eth_getLogs",
            move |p, (db, _, _), _| -> Result<Vec<alloy::rpc::types::Log>, ErrorObjectOwned> {
                let (f,): (alloy::rpc::types::Filter,) = p.parse()?;
                // these are unique
                if f.address != ROLLUP.into() {
                    return Ok(Default::default());
                }
                if f.topics[0].matches(&ISequencerInbox::SequencerBatchData::SIGNATURE_HASH) {
                    let index = match f.topics[1].to_value_or_array() {
                        Some(alloy::rpc::types::ValueOrArray::Value(i)) => i,
                        _ => return Err(err("missing topic1")),
                    };
                    let ind = u64::from_be_bytes(index[24..32].try_into().map_err(to_err)?);
                    if f.block_option != FilterBlockOption::AtBlockHash(U256::from(ind + 1).into())
                    {
                        return Err(err("block hash and batch index mismatch"));
                    }
                    let block = db.get_block(ind + 1)?;
                    return Ok(vec![create_log(
                        ind + 1,
                        ISequencerInbox::SequencerBatchData {
                            batchSequenceNumber: U256::from(ind),
                            data: block.batch,
                        }
                        .encode_log_data(),
                    )]);
                }
                let mut events = vec![];
                let (from_block, to_block) = match f.block_option {
                    FilterBlockOption::Range {
                        from_block: Some(BlockNumberOrTag::Number(from_block)),
                        to_block: Some(BlockNumberOrTag::Number(to_block)),
                    } => (from_block, to_block),
                    _ => return Err(err("block range not found")),
                };
                if from_block == 0 || to_block < from_block {
                    return Err(err("invalid block range"));
                }
                if f.topics[0].matches(&IBridge::MessageDelivered::SIGNATURE_HASH) {
                    for i in from_block..to_block + 1 {
                        let block = db.get_block(i)?;
                        let mut before_acc = block.before_message_acc;
                        for (j, (msg, acc)) in block.messages.iter().enumerate() {
                            events.push(create_log(
                                i,
                                IBridge::MessageDelivered {
                                    messageIndex: U256::from(block.before_message_count + j as u64),
                                    beforeInboxAcc: before_acc,
                                    inbox: ROLLUP,
                                    kind: msg.kind,
                                    sender: msg.sender,
                                    messageDataHash: keccak256(&msg.data),
                                    baseFeeL1: msg.base_fee_l1,
                                    timestamp: block.timestamp,
                                }
                                .encode_log_data(),
                            ));
                            before_acc = *acc;
                        }
                    }
                }
                if f.topics[0].matches(&ISequencerInbox::SequencerBatchDelivered::SIGNATURE_HASH) {
                    for i in from_block..to_block + 1 {
                        let block = db.get_block(i)?;
                        events.push(create_log(
                            i,
                            ISequencerInbox::SequencerBatchDelivered {
                                batchSequenceNumber: U256::from(i - 1),
                                beforeAcc: block.before_batch_acc,
                                afterAcc: block.after_batch_acc,
                                delayedAcc: block.after_message_acc(),
                                afterDelayedMessagesRead: U256::from(block.after_message_count()),
                                timeBounds: isequencerinbox::IBridge::TimeBounds {
                                    minTimestamp: 0,
                                    maxTimestamp: u64::MAX,
                                    minBlockNumber: 0,
                                    maxBlockNumber: u64::MAX,
                                },
                                dataLocation: 1,
                            }
                            .encode_log_data(),
                        ));
                    }
                }
                if f.topics[0].matches(&IInbox::InboxMessageDelivered::SIGNATURE_HASH) {
                    for i in from_block..to_block + 1 {
                        let block = db.get_block(i)?;
                        for (j, (msg, _)) in block.messages.iter().enumerate() {
                            events.push(create_log(
                                i,
                                IInbox::InboxMessageDelivered {
                                    messageNum: U256::from(block.before_message_count + j as u64),
                                    data: msg.data.clone(),
                                }
                                .encode_log_data(),
                            ));
                        }
                    }
                }
                Ok(events)
            },
        )
        .unwrap();
    module
        .register_method(
            "eth_getBlockByHash",
            move |p, _, _| -> Result<alloy::rpc::types::Block, ErrorObjectOwned> {
                let (hash, _): (FixedBytes<32>, bool) = p.parse()?;
                Ok(alloy::rpc::types::Block {
                    header: alloy::rpc::types::Header {
                        inner: alloy::consensus::Header {
                            number: u64::from_be_bytes(hash[24..32].try_into().map_err(to_err)?),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
            },
        )
        .unwrap();
    module
        .register_method(
            "eth_getBlockByNumber",
            move |p,
                  (db, metrics, mutex),
                  _|
                  -> Result<alloy::rpc::types::Block, ErrorObjectOwned> {
                let (tag, _): (BlockNumberOrTag, bool) = p.parse()?;
                let number = match tag {
                    BlockNumberOrTag::Latest => db.get_block_number(),
                    BlockNumberOrTag::Finalized => {
                        let mut data = mutex.lock().unwrap();
                        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                        let mut ts = 0u64;
                        while let Some(block_ts) = data.1.front() {
                            if block_ts + finality_delay > now {
                                break;
                            }
                            ts = *block_ts;
                            data.0 += 1;
                            data.1.pop_front();
                        }
                        if ts > 0 {
                            metrics.record_finalized_block(data.0, ts);
                        }

                        data.0
                    }
                    _ => return Err(format!("invalid tag: {}", tag)).map_err(to_err),
                };
                Ok(alloy::rpc::types::Block {
                    header: alloy::rpc::types::Header {
                        inner: alloy::consensus::Header { number, ..Default::default() },
                        ..Default::default()
                    },
                    ..Default::default()
                })
            },
        )
        .unwrap();
    module
        .register_method("eth_call", move |p, (db, _, _), _| -> Result<Bytes, ErrorObjectOwned> {
            let (input, _): (TransactionRequest, BlockNumberOrTag) = p.parse()?;
            if input.to != Some(alloy::primitives::TxKind::Call(ROLLUP)) {
                return Ok(Default::default());
            }
            let input = input.input.input.ok_or_else(|| err("missing calldata"))?;
            let selector = input.get(0..4).ok_or_else(|| err("missing selector"))?;
            match TryInto::<[u8; 4]>::try_into(selector).map_err(to_err)? {
                // TODO(SEQ-767): make sure the max data size property is set properly
                IInbox::maxDataSizeCall::SELECTOR => Ok(117964.abi_encode().into()),
                IBridge::delayedMessageCountCall::SELECTOR => {
                    Ok((db.get_block(db.get_block_number())?.after_message_count())
                        .abi_encode()
                        .into())
                }
                ISequencerInbox::batchCountCall::SELECTOR => {
                    Ok((db.get_block_number()).abi_encode().into())
                }
                IBridge::delayedInboxAccsCall::SELECTOR => {
                    let data = IBridge::delayedInboxAccsCall::abi_decode(input.as_ref(), false)
                        .map_err(to_err)?;
                    let index = data._0.try_into().map_err(to_err)?;
                    Ok(db.get_message_acc(index)?.abi_encode().into())
                }
                ISequencerInbox::inboxAccsCall::SELECTOR => {
                    let data = ISequencerInbox::inboxAccsCall::abi_decode(input.as_ref(), false)
                        .map_err(to_err)?;
                    let index: u64 = data.index.try_into().map_err(to_err)?;
                    Ok(db.get_block(index + 1)?.after_batch_acc.abi_encode().into())
                }
                _ => Err(err("unknown selector")),
            }
        })
        .unwrap();

    module
}

#[cfg(test)]
mod tests {
    use super::start_mchain;
    use crate::{
        db::{tests::TestDB, DelayedMessage, MBlock, KVDB},
        mchain::MProvider,
        metrics::MchainMetrics,
    };
    use alloy::{
        eips::BlockNumberOrTag,
        primitives::{Address, Bytes},
        providers::Provider as _,
    };
    use jsonrpsee::server::{Server, ServerHandle};
    use std::{
        sync::{Arc, RwLock},
        time::UNIX_EPOCH,
    };
    use test_utils::port_manager::PortManager;

    #[ctor::ctor]
    fn init() {
        shared::logger::set_global_default_subscriber();
    }

    #[allow(clippy::redundant_pub_crate)]
    pub(crate) struct SystemTime(RwLock<u64>);
    static TIME: SystemTime = SystemTime(RwLock::new(0));
    impl SystemTime {
        pub(crate) fn now() -> &'static Self {
            &TIME
        }
        pub(crate) fn duration_since(&self, from: std::time::SystemTime) -> Option<&Self> {
            assert_eq!(from, UNIX_EPOCH);
            Some(&TIME)
        }
        pub(crate) fn as_secs(&self) -> u64 {
            *self.0.read().unwrap()
        }
        fn set(&self, secs: u64) {
            *self.0.write().unwrap() = secs;
        }
    }

    impl KVDB for Arc<TestDB> {
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

    impl MProvider {
        async fn get_finalized_block(&self) -> u64 {
            self.0
                .get_block_by_number(BlockNumberOrTag::Finalized)
                .await
                .expect("get_finalized_block failed")
                .expect("finalized block not found")
                .header
                .number
        }
    }

    async fn setup() -> eyre::Result<(MProvider, Arc<TestDB>, ServerHandle)> {
        let db = Arc::new(TestDB::new());
        let mchain =
            start_mchain(10, Address::ZERO, 60, db.clone(), MchainMetrics::default()).await;
        let port = PortManager::instance().next_port();
        let handle = Server::builder().build(format!("0.0.0.0:{}", port)).await?.start(mchain);
        let provider = MProvider::new(format!("http://localhost:{}", port).parse()?);
        Ok((provider, db, handle))
    }

    #[tokio::test]
    async fn rollback_to_block() -> eyre::Result<()> {
        let empty = DelayedMessage {
            kind: 0,
            sender: Address::ZERO,
            data: Default::default(),
            base_fee_l1: Default::default(),
        };
        let (mchain, db, _handle) = setup().await?;
        assert_eq!(db.0.read().unwrap().keys().len(), 3); // init msg, block number, batch (1)
        assert_eq!(mchain.get_block_number().await, 1);
        mchain.add_batch(MBlock::default()).await?;
        assert_eq!(db.0.read().unwrap().keys().len(), 4); // block (2)
        assert_eq!(mchain.get_block_number().await, 2);
        mchain.add_batch(MBlock { messages: vec![empty.clone()], ..Default::default() }).await?; // block + messages (3)
        assert_eq!(db.0.read().unwrap().keys().len(), 6);
        assert_eq!(mchain.get_block_number().await, 3);
        mchain.add_batch(MBlock { messages: vec![empty; 2], ..Default::default() }).await?; // block + 2 messagess (4)
        assert_eq!(db.0.read().unwrap().keys().len(), 9);
        assert_eq!(mchain.get_block_number().await, 4);
        mchain.rollback_to_block(2).await?; // rm 2 blocks + 3 messages
        assert_eq!(db.0.read().unwrap().keys().len(), 4);
        assert_eq!(mchain.get_block_number().await, 2);
        mchain.rollback_to_block(1).await?; // rm block
        assert_eq!(db.0.read().unwrap().keys().len(), 3);
        assert_eq!(mchain.get_block_number().await, 1);
        assert!(mchain.rollback_to_block(0).await.is_err());
        assert!(mchain.rollback_to_block(2).await.is_err());
        assert_eq!(db.0.read().unwrap().keys().len(), 3);
        assert_eq!(mchain.get_block_number().await, 1);
        Ok(())
    }

    #[tokio::test]
    async fn finality() -> eyre::Result<()> {
        let (mchain, _, _handle) = setup().await?;
        assert_eq!(mchain.get_block_number().await, 1);
        assert_eq!(mchain.get_finalized_block().await, 1);
        mchain.add_batch(MBlock::default()).await?;
        mchain.add_batch(MBlock { timestamp: 1, ..Default::default() }).await?;
        mchain.add_batch(MBlock { timestamp: 1, ..Default::default() }).await?;
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
        mchain.add_batch(MBlock { timestamp: 1, ..Default::default() }).await?;
        assert_eq!(mchain.get_block_number().await, 4);
        assert_eq!(mchain.get_finalized_block().await, 4);
        mchain.add_batch(MBlock { timestamp: 2, ..Default::default() }).await?;
        assert_eq!(mchain.get_block_number().await, 5);
        assert_eq!(mchain.get_finalized_block().await, 4);
        mchain.rollback_to_block(4).await?;
        TIME.set(62);
        assert_eq!(mchain.get_block_number().await, 4);
        assert_eq!(mchain.get_finalized_block().await, 4);
        mchain.add_batch(MBlock { timestamp: 100, ..Default::default() }).await?;
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
