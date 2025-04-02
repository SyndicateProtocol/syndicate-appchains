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
use eyre::{eyre, OptionExt, Result};
use jsonrpsee::types::{error::INTERNAL_ERROR_CODE, ErrorObjectOwned};
use rocksdb::{DBWithThreadMode, ThreadMode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct MDB<DB: KVDB> {
    handle: Option<jsonrpsee::server::ServerHandle>,
    db: Arc<DB>,
}

/// key-value db trait
#[allow(missing_docs)]
pub trait KVDB {
    fn get(&self, key: &[u8]) -> Result<Option<Bytes>>;
    fn put(&self, key: &[u8], value: &[u8]) -> Result<()>;
    fn delete(&self, key: &[u8]) -> Result<()>;
}

/// rocksdb implements the key-value trait
impl<T: ThreadMode> KVDB for DBWithThreadMode<T> {
    fn get(&self, key: &[u8]) -> Result<Option<Bytes>> {
        Ok(self.get(key)?.map(|x| x.into()))
    }
    fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        Ok(self.put(key, value)?)
    }
    fn delete(&self, key: &[u8]) -> Result<()> {
        Ok(self.delete(key)?)
    }
}

/// generic db trait for reading and writing block data
#[allow(missing_docs)]
pub trait DB {
    fn get_block(&self, key: u64) -> Result<Block>;
    fn put_block(&self, key: u64, value: Block) -> Result<()>;
    fn delete_block(&self, key: u64) -> Result<()>;
    fn get_block_number(&self) -> Result<u64>;
    fn put_block_number(&self, value: u64) -> Result<()>;
    fn get_message_acc(&self, key: u64) -> Result<FixedBytes<32>>;
    fn put_message_acc(&self, key: u64, value: FixedBytes<32>) -> Result<()>;
    fn delete_message_acc(&self, key: u64) -> Result<()>;
    /// Create a new block that a contains a batch
    fn add_batch(&self, block: MBlock) -> Result<()>;
}

impl<T: KVDB> DB for T {
    fn get_block(&self, key: u64) -> Result<Block> {
        Ok(bincode::deserialize(&self.get(format!("b{key}").as_bytes())?.ok_or_eyre("empty")?)?)
    }
    fn put_block(&self, key: u64, value: Block) -> Result<()> {
        self.put(format!("b{key}").as_bytes(), &bincode::serialize(&value)?)
    }
    fn delete_block(&self, key: u64) -> Result<()> {
        self.delete(format!("b{key}").as_bytes())
    }
    // count starts at and defaults to 0
    fn get_block_number(&self) -> Result<u64> {
        Ok(bincode::deserialize(&self.get(b"n")?.unwrap_or(bincode::serialize(&0u64)?.into()))?)
    }
    fn put_block_number(&self, value: u64) -> Result<()> {
        self.put(b"n", &bincode::serialize(&value)?)
    }
    fn get_message_acc(&self, key: u64) -> Result<FixedBytes<32>> {
        Ok(bincode::deserialize(
            &self.get(format!("m{key}").as_bytes())?.ok_or_eyre("message acc not found")?,
        )?)
    }
    fn put_message_acc(&self, key: u64, value: FixedBytes<32>) -> Result<()> {
        self.put(format!("m{key}").as_bytes(), &bincode::serialize(&value)?)
    }
    fn delete_message_acc(&self, key: u64) -> Result<()> {
        self.delete(format!("m{key}").as_bytes())
    }
    fn add_batch(&self, mblock: MBlock) -> Result<()> {
        let block_number = self.get_block_number()? + 1;
        let prev_block = if block_number > 1 {
            self.get_block(block_number - 1)?
        } else {
            // genesis block
            Block::default()
        };
        let mut block = Block {
            timestamp: mblock.timestamp,
            batch: mblock.batch,
            seq_block_number: mblock.seq_block_number,
            seq_block_hash: mblock.seq_block_hash,
            set_block_number: mblock.set_block_number,
            set_block_hash: mblock.set_block_hash,
            before_batch_acc: prev_block.after_batch_acc,
            before_message_count: prev_block.after_message_count(),
            before_message_acc: prev_block.after_message_acc(),
            messages: mblock.messages.iter().map(|x| (x.to_owned(), FixedBytes::ZERO)).collect(),
            after_batch_acc: Default::default(),
        };
        let mut before_inbox_acc = block.before_message_acc;
        for (i, (msg, acc)) in block.messages.iter_mut().enumerate() {
            let message_hash = keccak256(
                (
                    [msg.kind],
                    msg.sender,
                    block_number,
                    mblock.timestamp,
                    U256::from(block.before_message_count + i as u64),
                    U256::ZERO,
                    keccak256(&msg.data),
                )
                    .abi_encode_packed(),
            );
            before_inbox_acc = keccak256((before_inbox_acc, message_hash).abi_encode_packed());
            *acc = before_inbox_acc;
            self.put_message_acc(block.before_message_count + i as u64, before_inbox_acc)?;
        }
        let data_hash = keccak256(
            (0u64, u64::MAX, 0u64, u64::MAX, block.after_message_count()).abi_encode_packed(),
        );
        block.after_batch_acc = keccak256(
            (block.before_batch_acc, data_hash, block.after_message_acc()).abi_encode_packed(),
        );
        self.put_block(block_number, block)?;
        // update the block number last - incomplete blocks can be ignored
        self.put_block_number(block_number)?;
        Ok(())
    }
}

/// Block data stored in rocksdb
/// Note that the block hash does not affect derived block hashes and therefore
/// this implementation should be fully compatible with existing reth metachains.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Block {
    /// block epoch timestamp in seconds
    pub timestamp: u64,
    /// batch data
    pub batch: Bytes,
    /// accumulator
    pub after_batch_acc: FixedBytes<32>,
    /// delayed messages included in the batch & accumulator values
    pub messages: Vec<(DelayedMessage, FixedBytes<32>)>,
    /// previous sequencer inbox accumulator
    /// note that this is used to detect reorgs instead of block hash
    pub before_batch_acc: FixedBytes<32>,
    /// previous delayed message (inbox) accumulator
    /// note that this is used to detect reorgs instead of block hash
    pub before_message_acc: FixedBytes<32>,
    /// previous delayed messages read
    pub before_message_count: u64,
    /// reorg data
    pub seq_block_number: u64,
    pub seq_block_hash: FixedBytes<32>,
    pub set_block_number: u64,
    pub set_block_hash: FixedBytes<32>,
}

#[allow(missing_docs)]
impl Block {
    pub fn after_message_acc(&self) -> FixedBytes<32> {
        self.messages.last().map_or(self.before_message_acc, |x| x.1)
    }
    pub fn after_message_count(&self) -> u64 {
        self.before_message_count + self.messages.len() as u64
    }
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

fn to_err<T: ToString>(err: T) -> ErrorObjectOwned {
    ErrorObjectOwned::owned(INTERNAL_ERROR_CODE, err.to_string(), None::<()>)
}

fn err(message: &'static str) -> ErrorObjectOwned {
    ErrorObjectOwned::borrowed(INTERNAL_ERROR_CODE, message, None)
}

impl<T: KVDB + Send + Sync + 'static> MDB<T> {
    /// create a MDB object and start the jsonrpsee server
    pub async fn start(
        chain_id: u64,
        chain_owner: Address,
        port: u64,
        _metrics_port: u64,
        db: Arc<T>,
    ) -> Result<Self> {
        let server = jsonrpsee::server::Server::builder()
            .set_rpc_middleware(jsonrpsee::server::RpcServiceBuilder::new().rpc_logger(1024))
            .build(format!("0.0.0.0:{port}"))
            .await?;
        let mut this = Self { handle: Default::default(), db };

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
        if this.db.get_block_number()? == 0 {
            // 000b00800203 corresponds to a batch containing a single delayed message
            this.db.add_batch(MBlock {
                timestamp: 0,
                messages: vec![init_msg],
                batch: alloy::hex!("000b00800203").into(),
                seq_block_hash: FixedBytes::ZERO,
                seq_block_number: 0,
                set_block_hash: FixedBytes::ZERO,
                set_block_number: 0,
            })?;
        } else if this.db.get_block(1)?.messages[0].0 != init_msg {
            return Err(eyre!("init message does not match - are the cli arguments correct?"));
        }
        let mut module = jsonrpsee::RpcModule::new(());
        let db = this.db.clone();
        module.register_method(
            "mchain_addBatch",
            move |p, _, _| -> Result<(), ErrorObjectOwned> {
                let (batch,): (MBlock,) = p.parse()?;
                db.add_batch(batch).map_err(to_err)
            },
        )?;
        let db = this.db.clone();
        module.register_method(
            "mchain_setLatestHead",
            move |p, _, _| -> Result<(), ErrorObjectOwned> {
                let (block,): (u64,) = p.parse()?;
                let latest = db.get_block_number().map_err(to_err)?;
                if block == latest {
                    return Ok(());
                }
                if block > latest {
                    return Err(err("cannot set head past the last block"));
                }
                if block == 0 {
                    return Err(err("cannot set head before the first block"));
                }
                db.put_block_number(block).map_err(to_err)?;
                for i in block..latest {
                    db.delete_block(i + 1).map_err(to_err)?;
                    db.delete_message_acc(i + 1).map_err(to_err)?;
                }
                Ok(())
            },
        )?;
        module.register_method("eth_chainId", move |_, _, _| format!("{:#x}", MCHAIN_ID))?;
        module.register_method("eth_getCode", move |_, _, _| "0x")?;
        let db = this.db.clone();
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
                    if f.block_option != FilterBlockOption::AtBlockHash(U256::from(ind + 1).into())
                    {
                        return Err(err("block hash and batch index mismatch"));
                    }
                    let block = db.get_block(ind + 1).map_err(to_err)?;
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
                        let block = db.get_block(i).map_err(to_err)?;
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
                        let block = db.get_block(i).map_err(to_err)?;
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
                        let block = db.get_block(i).map_err(to_err)?;
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
        let db = this.db.clone();
        module.register_method(
            "eth_getBlockByNumber",
            move |p, _, _| -> Result<alloy::rpc::types::Block, ErrorObjectOwned> {
                let (tag, _): (BlockNumberOrTag, bool) = p.parse()?;
                let number = match tag {
                    BlockNumberOrTag::Latest => db.get_block_number().map_err(to_err),
                    // TODO: add safe, finalized support
                    BlockNumberOrTag::Safe | BlockNumberOrTag::Finalized => Ok(0),
                    _ => Err(eyre!("invalid tag: {}", tag)).map_err(to_err),
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
        let db = this.db.clone();
        module.register_method("eth_call", move |p, _, _| -> Result<Bytes, ErrorObjectOwned> {
            let (input, _): (TransactionRequest, BlockNumberOrTag) = p.parse()?;
            if input.to != Some(alloy::primitives::TxKind::Call(get_rollup_contract_address())) {
                return Ok(Default::default());
            }
            let input = input.input.input.ok_or_else(|| err("missing calldata"))?;
            let selector = input.get(0..4).ok_or_else(|| err("missing selector"))?;
            match TryInto::<[u8; 4]>::try_into(selector).map_err(to_err)? {
                // TODO: make sure the max data size property is set properly
                IInbox::maxDataSizeCall::SELECTOR => Ok(117964.abi_encode().into()),
                IBridge::delayedMessageCountCall::SELECTOR => Ok((db
                    .get_block(db.get_block_number().map_err(to_err)?)
                    .map_err(to_err)?
                    .after_message_count())
                .abi_encode()
                .into()),
                ISequencerInbox::batchCountCall::SELECTOR => {
                    Ok((db.get_block_number().map_err(to_err)?).abi_encode().into())
                }
                IBridge::delayedInboxAccsCall::SELECTOR => {
                    let data = IBridge::delayedInboxAccsCall::abi_decode(input.as_ref(), false)
                        .map_err(to_err)?;
                    let index = data._0.try_into().map_err(to_err)?;
                    Ok(db.get_message_acc(index).map_err(to_err)?.abi_encode().into())
                }
                ISequencerInbox::inboxAccsCall::SELECTOR => {
                    let data = ISequencerInbox::inboxAccsCall::abi_decode(input.as_ref(), false)
                        .map_err(to_err)?;
                    let index: u64 = data.index.try_into().map_err(to_err)?;
                    Ok(db.get_block(index + 1).map_err(to_err)?.after_batch_acc.abi_encode().into())
                }
                _ => Err(err("unknown selector")),
            }
        })?;
        let db = this.db.clone();
        module.register_method(
            "mchain_getSourceChainsProcessedBlocks",
            move |p, _, _| -> Result<(u64, FixedBytes<32>, u64, FixedBytes<32>), ErrorObjectOwned> {
                let tag: BlockNumberOrTag = p.parse()?;
                let block_num = match tag {
                    BlockNumberOrTag::Latest => db.get_block_number(),
                    BlockNumberOrTag::Number(i) => Ok(i),
                    _ => Err(eyre!("unexpected block tag: {}", tag))
                }.map_err(to_err)?;
                let block = db.get_block(block_num).map_err(to_err)?;
                Ok((
                    block.seq_block_number,
                    block.seq_block_hash,
                    block.set_block_number,
                    block.set_block_hash,
                ))
            },
        )?;
        this.handle = Some(server.start(module));
        Ok(this)
    }
}
