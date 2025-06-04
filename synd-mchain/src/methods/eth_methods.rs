#[cfg(test)]
use crate::methods::common::test_utils::SystemTime;
use crate::{
    db::{to_err, ArbitrumDB},
    methods::common::{create_header, create_log, err, Context, APPCHAIN_CONTRACT, MCHAIN_ID},
    metrics::MchainMetrics,
};
use alloy::{
    eips::BlockNumberOrTag,
    primitives::{keccak256, Bytes, FixedBytes, U256},
    rpc::types::{FilterBlockOption, TransactionRequest},
    sol_types::{SolCall, SolEvent as _, SolValue as _},
};
use contract_bindings::synd::{
    ibridge::IBridge,
    iinbox::IInbox,
    iinboxbase::IInboxBase,
    isequencerinbox::{self, ISequencerInbox},
};
use jsonrpsee::{
    core::error::StringError,
    types::{ErrorObjectOwned, Params},
    Extensions, PendingSubscriptionSink,
};
#[cfg(not(test))]
use std::time::SystemTime;
use std::{
    sync::{Arc, Mutex},
    time::UNIX_EPOCH,
};

/// `eth_subscribe`
#[allow(clippy::unwrap_used)]
pub async fn eth_subscribe(
    p: Params<'_>,
    pending: PendingSubscriptionSink,
    ctx: Arc<(impl ArbitrumDB + Send + Sync, MchainMetrics, Mutex<Context>)>,
    _: Extensions,
) -> Result<(), StringError> {
    let (param,): (&str,) = p.parse()?;
    if param != "newHeads" {
        return Err(format!("unknown subscription event: {}", param).into());
    }
    let sink = pending.accept().await.map_err(to_err)?;
    ctx.2.lock().unwrap().subs.push(sink.clone());
    sink.closed().await;
    drop(sink);
    Ok(())
}

/// `eth_chainId`
pub fn eth_chain_id(
    _: Params<'_>,
    _: &(impl ArbitrumDB + Send + Sync, MchainMetrics, Mutex<Context>),
    _: &Extensions,
) -> String {
    format!("{:#x}", MCHAIN_ID)
}

/// `eth_getCode`
pub fn eth_get_code(
    _: Params<'_>,
    _: &(impl ArbitrumDB + Send + Sync, MchainMetrics, Mutex<Context>),
    _: &Extensions,
) -> String {
    "0x".to_string()
}

/// `eth_getLogs`
#[allow(clippy::unwrap_used)]
pub fn eth_get_logs(
    p: Params<'_>,
    (db, _, _): &(impl ArbitrumDB + Send + Sync, MchainMetrics, Mutex<Context>),
    _: &Extensions,
) -> Result<Vec<alloy::rpc::types::Log>, ErrorObjectOwned> {
    let (f,): (alloy::rpc::types::Filter,) = p.parse()?;
    // these are unique
    if f.address != APPCHAIN_CONTRACT.into() {
        return Ok(Default::default());
    }
    if f.topics[0].matches(&ISequencerInbox::SequencerBatchData::SIGNATURE_HASH) {
        let index = match f.topics[1].to_value_or_array() {
            Some(alloy::rpc::types::ValueOrArray::Value(i)) => i,
            _ => return Err(err("missing topic1")),
        };
        let ind = u64::from_be_bytes(index[index.len() - 8..].try_into().map_err(to_err)?);
        if f.block_option != FilterBlockOption::AtBlockHash(U256::from(ind + 1).into()) {
            return Err(err("block hash and batch index mismatch"));
        }
        let block = db.get_block(ind + 1)?;
        if block.batch.is_empty() {
            return Err(err("batch is empty - SequencerBatchData event does not exist"));
        }
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
                        inbox: APPCHAIN_CONTRACT,
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
                    dataLocation: if block.batch.is_empty() {
                        2 // NoData
                    } else {
                        1 // SeparateBatchEvent
                    },
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
}

/// `eth_getBlockByHash`
pub fn eth_get_block_by_hash(
    p: Params<'_>,
    (db, _, _): &(impl ArbitrumDB + Send + Sync, MchainMetrics, Mutex<Context>),
    _: &Extensions,
) -> Result<alloy::rpc::types::Block, ErrorObjectOwned> {
    let (hash, _): (FixedBytes<32>, bool) = p.parse()?;
    let number = u64::from_be_bytes(hash[hash.len() - 8..].try_into().map_err(to_err)?);
    let block = db.get_block(number)?;
    Ok(alloy::rpc::types::Block {
        header: create_header(number, block.slot.seq_block_number, block.timestamp),
        ..Default::default()
    })
}

/// `eth_getBlockByNumber`
#[allow(clippy::unwrap_used)]
pub fn eth_get_block_by_number(
    p: Params<'_>,
    (db, metrics, mutex): &(impl ArbitrumDB + Send + Sync, MchainMetrics, Mutex<Context>),
    _: &Extensions,
    finality_delay: u64,
) -> Result<alloy::rpc::types::Block, ErrorObjectOwned> {
    let (tag, _): (BlockNumberOrTag, bool) = p.parse()?;
    let number = match tag {
        BlockNumberOrTag::Latest => db.get_state().batch_count,
        BlockNumberOrTag::Safe | BlockNumberOrTag::Finalized => {
            let mut data = mutex.lock().unwrap();
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let mut ts = 0u64;
            while let Some(block_ts) = data.pending_ts.front() {
                if block_ts + finality_delay > now {
                    break;
                }
                ts = *block_ts;
                data.finalized_block += 1;
                data.pending_ts.pop_front();
            }
            if ts > 0 {
                metrics.record_finalized_block(data.finalized_block, ts);
            }

            data.finalized_block
        }
        _ => return Err(format!("invalid tag: {}", tag)).map_err(to_err),
    };
    let block = db.get_block(number).unwrap();
    Ok(alloy::rpc::types::Block {
        header: create_header(number, block.slot.seq_block_number, block.timestamp),
        ..Default::default()
    })
}

/// `eth_call`
pub fn eth_call(
    p: Params<'_>,
    (db, _, _): &(impl ArbitrumDB + Send + Sync, MchainMetrics, Mutex<Context>),
    _: &Extensions,
) -> Result<Bytes, ErrorObjectOwned> {
    let (input, _): (TransactionRequest, BlockNumberOrTag) = p.parse()?;
    if input.to != Some(alloy::primitives::TxKind::Call(APPCHAIN_CONTRACT)) {
        return Ok(Default::default());
    }
    let input = input.input.input.ok_or_else(|| err("missing calldata"))?;
    let selector = input.get(0..4).ok_or_else(|| err("missing selector"))?;
    match TryInto::<[u8; 4]>::try_into(selector).map_err(to_err)? {
        // TODO(SEQ-767): make sure the max data size property is set properly
        IInboxBase::maxDataSizeCall::SELECTOR => Ok(117964.abi_encode().into()),
        IBridge::delayedMessageCountCall::SELECTOR => {
            Ok(db.get_state().message_count.abi_encode().into())
        }
        ISequencerInbox::batchCountCall::SELECTOR => {
            Ok(db.get_state().batch_count.abi_encode().into())
        }
        IBridge::delayedInboxAccsCall::SELECTOR => {
            let data =
                IBridge::delayedInboxAccsCall::abi_decode(input.as_ref(), false).map_err(to_err)?;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{DelayedMessage, MBlock, Slot};
    use alloy::{primitives::Bytes, rpc::types::TransactionInput};
    use shared::service_start_utils::MetricsState;
    use std::{
        collections::{HashMap, VecDeque},
        sync::{Mutex, RwLock},
    };

    #[allow(clippy::redundant_pub_crate)]
    #[derive(Debug)]
    pub(crate) struct TestDB(pub(crate) RwLock<HashMap<Bytes, Bytes>>);

    impl TestDB {
        pub(crate) fn new() -> Self {
            Self(RwLock::new(HashMap::new()))
        }
    }

    #[allow(clippy::unwrap_used)]
    impl ArbitrumDB for TestDB {
        fn get<K: AsRef<[u8]>>(&self, key: K) -> Option<Bytes> {
            self.0.read().unwrap().get(key.as_ref()).cloned()
        }
        fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&self, key: K, value: V) {
            self.0
                .write()
                .unwrap()
                .insert(key.as_ref().to_owned().into(), value.as_ref().to_owned().into());
        }
        fn delete<K: AsRef<[u8]>>(&self, key: K) {
            self.0.write().unwrap().remove(key.as_ref());
        }
    }

    fn get_test_context() -> (impl ArbitrumDB + Send + Sync, MchainMetrics, Mutex<Context>) {
        (
            TestDB::new(),
            MchainMetrics::new(&mut MetricsState::default().registry),
            Mutex::new(Context { finalized_block: 0, pending_ts: VecDeque::new(), subs: vec![] }),
        )
    }

    fn get_test_mblock() -> MBlock {
        MBlock {
            payload: Some((Bytes::default(), vec![DelayedMessage::default()])),
            timestamp: 1000,
            slot: Slot { seq_block_number: 1, ..Default::default() },
        }
    }

    #[test]
    fn test_eth_chain_id() {
        let result = eth_chain_id(Params::new(None), &get_test_context(), &Extensions::default());
        assert_eq!(result, format!("{:#x}", MCHAIN_ID));
    }

    #[test]
    fn test_eth_get_code() {
        let result = eth_get_code(Params::new(None), &get_test_context(), &Extensions::default());
        assert_eq!(result, "0x");
    }

    #[test]
    fn test_eth_get_block_by_hash() {
        let (mock_db, metrics, context) = get_test_context();

        // Create a test block
        let batch = get_test_mblock();
        mock_db.add_batch(batch).unwrap();

        // Create a hash for block 1
        let mut hash = [0u8; 32];
        hash[24..].copy_from_slice(&1u64.to_be_bytes());
        let hash = FixedBytes::from(hash);

        let json = serde_json::to_value((hash, false)).unwrap().to_string();
        let params = Params::new(Some(json.as_str()));
        let result =
            eth_get_block_by_hash(params, &(mock_db, metrics, context), &Extensions::default());

        assert!(result.is_ok());
        let block = result.unwrap();
        assert_eq!(block.header.number, 1);
        assert_eq!(block.header.timestamp, 1000);
    }

    #[test]
    fn test_eth_get_block_by_number() {
        let (mock_db, metrics, context) = get_test_context();

        // Create a test block
        let batch = get_test_mblock();
        mock_db.add_batch(batch).unwrap();

        let json = serde_json::to_value((BlockNumberOrTag::Latest, false)).unwrap().to_string();
        let params = Params::new(Some(json.as_str()));
        let result = eth_get_block_by_number(
            params,
            &(mock_db, metrics, context),
            &Extensions::default(),
            0,
        );

        assert!(result.is_ok());
        let block = result.unwrap();
        assert_eq!(block.header.number, 1);
        assert_eq!(block.header.timestamp, 1000);
    }

    #[test]
    fn test_eth_call() {
        // Test batchCount call
        let tx = TransactionRequest {
            to: Some(alloy::primitives::TxKind::Call(APPCHAIN_CONTRACT)),
            input: TransactionInput::new(Bytes::from(IInbox::maxDataSizeCall::SELECTOR)),
            ..Default::default()
        };

        let json = serde_json::to_value((tx, BlockNumberOrTag::Latest)).unwrap().to_string();
        let params = Params::new(Some(json.as_str()));
        let result = eth_call(params, &get_test_context(), &Extensions::default());

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response, Bytes::from(117964u64.abi_encode()));
    }
}
