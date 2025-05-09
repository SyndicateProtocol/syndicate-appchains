use crate::{
    db::{to_err, ArbitrumDB, DelayedMessage, MBlock, Slot, State},
    metrics::MchainMetrics,
};
use alloy::{
    eips::BlockNumberOrTag,
    primitives::{address, keccak256, Address, Bytes, FixedBytes, U256},
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
    RpcModule, SubscriptionMessage, SubscriptionSink,
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

fn create_header(block_num: u64) -> alloy::rpc::types::Header {
    alloy::rpc::types::Header {
        inner: alloy::consensus::Header { number: block_num, ..Default::default() },
        hash: U256::from(block_num).into(),
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

#[derive(Debug)]
pub struct Context {
    finalized_block: u64,
    pending_ts: VecDeque<u64>,
    subs: Vec<SubscriptionSink>,
}

#[allow(clippy::unwrap_used)]
pub fn start_mchain<T: ArbitrumDB + Send + Sync + 'static>(
    chain_id: u64,
    rollup_owner: Address,
    finality_delay: u64,
    db: T,
    metrics: MchainMetrics,
) -> RpcModule<(T, MchainMetrics, Mutex<Context>)> {
    db.check_version();
    let init_msg = DelayedMessage {
        kind: 11, // L1MessageType::Initialize
        sender: Address::ZERO,
        data: (
            U256::from(chain_id),
            [1u8],      // initMsgVersion
            U256::ZERO, // currentDataCost
            rollup_config(chain_id, rollup_owner),
        )
            .abi_encode_packed()
            .into(),
        base_fee_l1: U256::ZERO,
    };
    let mut pending_ts: VecDeque<u64> = Default::default();
    let mut finalized_block = 1u64;
    if db.get_state().batch_count == 0 {
        // 000b00800203 corresponds to a batch containing a single delayed message
        db.add_batch(MBlock {
            payload: Some((Bytes::from_static(&alloy::hex!("000b00800203")), vec![init_msg])),
            ..Default::default()
        })
        .unwrap();
    } else {
        let db_init = &db.get_block(1).unwrap().messages[0].0;
        if db_init != &init_msg {
            error!("init message does not match - are the cli arguments correct?");
            assert_eq!(db_init, &init_msg);
        }
        // search for the finalized head. store unfinalized timestamps in a queue.
        finalized_block = db.get_state().batch_count;
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
    assert_eq!(finalized_block + pending_ts.len() as u64, db.get_state().batch_count);
    let mut module = RpcModule::new((
        db,
        metrics,
        Mutex::new(Context { finalized_block, pending_ts, subs: Default::default() }),
    ));

    // -------------------------------------------------
    // mchain methods
    // -------------------------------------------------
    module
        .register_method(
            "mchain_addBatch",
            move |p, (db, metrics, mutex), _| -> Result<Option<u64>, ErrorObjectOwned> {
                let (batch,): (MBlock,) = p.parse()?;
                let timestamp = batch.timestamp;
                let seq_block_number = batch.slot.seq_block_number;
                let block = db.add_batch(batch)?;
                metrics.record_sequencing_block(seq_block_number, timestamp);
                Ok(block.inspect(|&block| {
                    metrics.record_last_block(block, timestamp);
                    let mut data = mutex.lock().unwrap();
                    data.pending_ts.push_back(timestamp);
                    assert_eq!(data.finalized_block + data.pending_ts.len() as u64, block);
                    data.subs.retain_mut(|sink| {
                        !sink.is_closed() &&
                            sink.try_send(
                                SubscriptionMessage::from_json(&create_header(block)).unwrap(),
                            )
                            .inspect_err(|err| error!("try_send failed: {}", err))
                            .is_ok()
                    });
                    drop(data);
                }))
            },
        )
        .unwrap();
    module
        .register_method(
            "mchain_rollbackToBlock",
            move |p, (db, metrics, mutex), _| -> Result<(), ErrorObjectOwned> {
                let (block_number,): (u64,) = p.parse()?;
                let state = db.get_state();
                if block_number > state.batch_count {
                    return Err(err("cannot set head past the last block"));
                }
                if block_number == 0 {
                    return Err(err("cannot set head before the first block"));
                }
                let block = db.get_block(block_number).unwrap();
                if block_number == state.batch_count {
                    // reset the pending batch count.
                    // do not record or log the reorg since it has no user impact.
                    db.put_state(&State { timestamp: block.timestamp, slot: block.slot, ..state });
                    return Ok(());
                }
                let block_message_count = block.after_message_count();
                // first reset the state - it is okay if the other deletions fail, incomplete
                // data is ignored.
                db.put_state(&State {
                    batch_count: block_number,
                    batch_acc: block.after_batch_acc,
                    message_count: block.after_message_count(),
                    message_acc: block.after_message_acc(),
                    timestamp: block.timestamp,
                    slot: block.slot,
                });
                // next delete blocks, messages to free up space.
                metrics.record_reorg(block_number, state.batch_count, block.timestamp);
                for i in block_number..state.batch_count {
                    db.delete_block(i + 1);
                }
                for i in block_message_count..state.message_count {
                    db.delete_message_acc(i);
                }
                // finally update stale finality data.
                let mut data = mutex.lock().unwrap();
                if block_number < data.finalized_block {
                    metrics.record_finalized_block(block_number, block.timestamp);
                    data.finalized_block = block_number;
                    data.pending_ts.clear();
                } else {
                    let removed = (state.batch_count - block_number) as usize;
                    let data_len = data.pending_ts.len();
                    assert!(data_len >= removed);
                    data.pending_ts.truncate(data_len - removed);
                }
                assert_eq!(data.finalized_block + data.pending_ts.len() as u64, block_number);
                data.subs.retain_mut(|sink| {
                    !sink.is_closed() &&
                        sink.try_send(
                            SubscriptionMessage::from_json(&create_header(block_number)).unwrap(),
                        )
                        .inspect_err(|err| error!("try_send failed: {}", err))
                        .is_ok()
                });
                drop(data);
                Ok(())
            },
        )
        .unwrap();
    module
        .register_method(
            "mchain_rollupOwner",
            move |_, _, _| -> Result<Address, ErrorObjectOwned> { Ok(rollup_owner) },
        )
        .unwrap();
    module
        .register_method(
            "mchain_getSourceChainsProcessedBlocks",
            move |p, (db, _, _), _| -> Result<(Slot, u64), ErrorObjectOwned> {
                let (tag,): (BlockNumberOrTag,) = p.parse()?;
                match tag {
                    BlockNumberOrTag::Pending => {
                        let state = db.get_state();
                        Ok((state.slot, state.batch_count + 1))
                    }
                    BlockNumberOrTag::Number(block_num) => {
                        let block = db.get_block(block_num)?;
                        Ok((block.slot, block_num))
                    }
                    _ => Err(to_err(format!("unexpected block tag: {}", tag))),
                }
            },
        )
        .unwrap();

    // -------------------------------------------------
    // eth methods
    // -------------------------------------------------
    module
        .register_subscription(
            "eth_subscribe",
            "eth_subscription",
            "eth_unsubscribe",
            move |p, pending, ctx, _| async move {
                let (param,): (&str,) = p.parse()?;
                if param != "newHeads" {
                    return Err(format!("unknown subscription event: {}", param).into());
                }
                let sink = pending.accept().await.map_err(to_err)?;
                ctx.2.lock().unwrap().subs.push(sink.clone());
                sink.closed().await;
                drop(sink);
                Ok(())
            },
        )
        .unwrap();

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
                    header: create_header(u64::from_be_bytes(
                        hash[24..32].try_into().map_err(to_err)?,
                    )),
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
                    BlockNumberOrTag::Latest => db.get_state().batch_count,
                    BlockNumberOrTag::Finalized => {
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
                Ok(alloy::rpc::types::Block { header: create_header(number), ..Default::default() })
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
                    Ok(db.get_state().message_count.abi_encode().into())
                }
                ISequencerInbox::batchCountCall::SELECTOR => {
                    Ok(db.get_state().batch_count.abi_encode().into())
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
#[allow(clippy::redundant_pub_crate)]
pub(crate) mod tests {
    use std::{sync::RwLock, time::UNIX_EPOCH};

    pub(crate) struct SystemTime(RwLock<u64>);
    pub(crate) static TIME: SystemTime = SystemTime(RwLock::new(0));
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
        pub(crate) fn set(&self, secs: u64) {
            *self.0.write().unwrap() = secs;
        }
    }
}
