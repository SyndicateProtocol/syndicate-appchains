use crate::db::{to_err, ArbitrumDB as _, KVDB};
use alloy::{
    eips::BlockNumberOrTag,
    primitives::{keccak256, Address, Bytes, FixedBytes, U256},
    rpc::types::{FilterBlockOption, TransactionRequest},
    sol_types::{SolCall, SolEvent as _, SolValue as _},
};
use block_builder::{
    config::get_rollup_contract_address,
    connectors::mchain::{rollup_config, MCHAIN_ID},
    rollups::{
        arbitrum::arbitrum_adapter::L1MessageType,
        shared::rollup_adapter::{DelayedMessage, MBlock},
    },
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
use std::sync::Arc;

fn err(message: &'static str) -> ErrorObjectOwned {
    ErrorObjectOwned::borrowed(INTERNAL_ERROR_CODE, message, None)
}

fn create_log(block_num: u64, data: alloy::primitives::LogData) -> alloy::rpc::types::Log {
    alloy::rpc::types::Log {
        inner: alloy::primitives::Log { address: get_rollup_contract_address(), data },
        transaction_hash: Some(FixedBytes::ZERO),
        block_number: Some(block_num),
        block_hash: Some(U256::from(block_num).into()),
        ..Default::default()
    }
}

pub async fn start_mchain<T: KVDB + Send + Sync + 'static>(
    chain_id: u64,
    chain_owner: Address,
    db_: Arc<T>,
) -> eyre::Result<RpcModule<()>> {
    let init_msg = DelayedMessage {
        kind: L1MessageType::Initialize as u8,
        sender: Address::ZERO,
        data: (
            U256::from(chain_id),
            [1u8],      // initMsgVersion
            U256::ZERO, // currentDataCost
            rollup_config(chain_id, chain_owner),
        )
            .abi_encode_packed()
            .into(),
    };
    if db_.get_block_number() == 0 {
        // 000b00800203 corresponds to a batch containing a single delayed message
        db_.add_batch(MBlock {
            timestamp: 0,
            messages: vec![init_msg],
            batch: alloy::hex!("000b00800203").into(),
            seq_block_hash: FixedBytes::ZERO,
            seq_block_number: 0,
            set_block_hash: FixedBytes::ZERO,
            set_block_number: 0,
        });
    } else {
        let db_init = &db_.get_block(1)?.messages[0].0;
        if db_init != &init_msg {
            eprintln!("init message does not match - are the cli arguments correct?");
            assert_eq!(db_init, &init_msg);
        }
    }
    let mut module = RpcModule::new(());

    // -------------------------------------------------
    // mchain methods
    // -------------------------------------------------
    let db = db_.clone();
    module.register_method("mchain_addBatch", move |p, _, _| -> Result<(), ErrorObjectOwned> {
        let (batch,): (MBlock,) = p.parse()?;
        db.add_batch(batch);
        Ok(())
    })?;
    let db = db_.clone();
    module.register_method(
        "mchain_setLatestHead",
        move |p, _, _| -> Result<(), ErrorObjectOwned> {
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
            db.put_block_number(block);
            let start_block = db.get_block(block)?;
            let end_block = db.get_block(latest)?;
            for i in block..latest {
                db.delete_block(i + 1);
            }
            for i in start_block.after_message_count()..end_block.after_message_count() {
                db.delete_message_acc(i);
            }
            Ok(())
        },
    )?;
    let db = db_.clone();
    module.register_method(
        "mchain_getSourceChainsProcessedBlocks",
        move |p, _, _| -> Result<(u64, FixedBytes<32>, u64, FixedBytes<32>), ErrorObjectOwned> {
            let tag: BlockNumberOrTag = p.parse()?;
            let block_num = match tag {
                BlockNumberOrTag::Latest => Ok(db.get_block_number()),
                BlockNumberOrTag::Number(i) => Ok(i),
                _ => Err(to_err(format!("unexpected block tag: {}", tag))),
            }?;
            let block = db.get_block(block_num)?;
            Ok((
                block.seq_block_number,
                block.seq_block_hash,
                block.set_block_number,
                block.set_block_hash,
            ))
        },
    )?;

    // -------------------------------------------------
    // eth methods
    // -------------------------------------------------
    module.register_method("eth_chainId", move |_, _, _| format!("{:#x}", MCHAIN_ID))?;
    module.register_method("eth_getCode", move |_, _, _| "0x")?;
    let db = db_.clone();
    module.register_method(
        "eth_getLogs",
        move |p, _, _| -> Result<Vec<alloy::rpc::types::Log>, ErrorObjectOwned> {
            let (f,): (alloy::rpc::types::Filter,) = p.parse()?;
            // these are unique
            if f.address != get_rollup_contract_address().into() {
                return Ok(Default::default());
            }
            if f.topics[0].matches(&ISequencerInbox::SequencerBatchData::SIGNATURE_HASH) {
                let index = match f.topics[1].to_value_or_array() {
                    Some(alloy::rpc::types::ValueOrArray::Value(i)) => Ok(i),
                    _ => Err(err("missing topic1")),
                }?;
                let ind = u64::from_be_bytes(index[24..32].try_into().map_err(to_err)?);
                if f.block_option != FilterBlockOption::AtBlockHash(U256::from(ind + 1).into()) {
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
                } => Ok((from_block, to_block)),
                _ => Err(err("block range not found")),
            }?;
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
                                inbox: get_rollup_contract_address(),
                                kind: msg.kind,
                                sender: msg.sender,
                                messageDataHash: keccak256(&msg.data),
                                baseFeeL1: U256::ZERO,
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
    )?;
    module.register_method(
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
    )?;
    let db = db_.clone();
    module.register_method(
        "eth_getBlockByNumber",
        move |p, _, _| -> Result<alloy::rpc::types::Block, ErrorObjectOwned> {
            let (tag, _): (BlockNumberOrTag, bool) = p.parse()?;
            let number = match tag {
                BlockNumberOrTag::Latest => Ok(db.get_block_number()),
                // TODO(SEQ-708): add safe, finalized support
                BlockNumberOrTag::Safe | BlockNumberOrTag::Finalized => Ok(0),
                _ => Err(format!("invalid tag: {}", tag)).map_err(to_err),
            }?;
            Ok(alloy::rpc::types::Block {
                header: alloy::rpc::types::Header {
                    inner: alloy::consensus::Header { number, ..Default::default() },
                    ..Default::default()
                },
                ..Default::default()
            })
        },
    )?;
    let db = db_;
    module.register_method("eth_call", move |p, _, _| -> Result<Bytes, ErrorObjectOwned> {
        let (input, _): (TransactionRequest, BlockNumberOrTag) = p.parse()?;
        if input.to != Some(alloy::primitives::TxKind::Call(get_rollup_contract_address())) {
            return Ok(Default::default());
        }
        let input = input.input.input.ok_or_else(|| err("missing calldata"))?;
        let selector = input.get(0..4).ok_or_else(|| err("missing selector"))?;
        match TryInto::<[u8; 4]>::try_into(selector).map_err(to_err)? {
            // TODO(SEQ-767): make sure the max data size property is set properly
            IInbox::maxDataSizeCall::SELECTOR => Ok(117964.abi_encode().into()),
            IBridge::delayedMessageCountCall::SELECTOR => {
                Ok((db.get_block(db.get_block_number())?.after_message_count()).abi_encode().into())
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
    })?;

    Ok(module)
}
