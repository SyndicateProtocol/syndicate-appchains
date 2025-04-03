//! Arbitrum block builder implementation
//!
//! This module provides functionality for building an Arbitrum batch submitter transaction from a
//! list of transactions. It implements the [`RollupBlockBuilder`] trait to standardize block
//! construction across different rollup implementations

use crate::{
    config::{get_rollup_contract_address, BlockBuilderConfig},
    rollups::{
        arbitrum::batch::{Batch, BatchMessage, L1IncomingMessage, L1IncomingMessageHeader},
        shared::{RollupAdapter, SequencingTransactionParser},
    },
};
use alloy::{
    eips::{BlockId, BlockNumberOrTag},
    primitives::{Address, Bytes, FixedBytes, U256},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol_types::{SolCall, SolEvent, SolInterface},
};
use async_trait::async_trait;
use common::types::{BlockAndReceipts, BlockRef, KnownState, Log, Slot};
use contract_bindings::arbitrum::{
    ibridge::IBridge::MessageDelivered,
    idelayedmessageprovider::IDelayedMessageProvider::{
        InboxMessageDelivered, InboxMessageDeliveredFromOrigin,
    },
    iinboxbase::IInboxBase::sendL2MessageFromOriginCall,
    rollup::Rollup,
};
use eyre::Result;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;
use tracing::{debug, error, info, trace};

const MSG_DELIVERED_EVENT_HASH: FixedBytes<32> = MessageDelivered::SIGNATURE_HASH;
const INBOX_MSG_DELIVERED_EVENT_HASH: FixedBytes<32> = InboxMessageDelivered::SIGNATURE_HASH;
const INBOX_MSG_DELIVERED_FROM_ORIGIN_EVENT_HASH: FixedBytes<32> =
    InboxMessageDeliveredFromOrigin::SIGNATURE_HASH;

#[allow(missing_docs)] // self-documenting
#[derive(Debug, Error)]
pub enum ArbitrumBlockBuilderError {
    #[error("Failed to decode {0}: {1}")]
    DecodingError(&'static str, eyre::Error),

    #[error("Missing inbox message data for message index: {0}")]
    MissingInboxMessageData(U256),

    #[error("Delayed message ignored: type = {0}")]
    DelayedMessageIgnored(L1MessageType),
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq)]
/// `<https://github.com/OffchainLabs/nitro/blob/c7f3429e2456bf5ca296a49cec3bb437420bc2bb/contracts/src/libraries/MessageTypes.sol>`
pub enum L1MessageType {
    L2Message = 3,
    L2FundedByL1 = 7,
    SubmitRetryable = 9,
    Initialize = 11,
    EthDeposit = 12,
    BatchPostingReport = 13,
}

impl TryFrom<u8> for L1MessageType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(Self::L2Message),
            7 => Ok(Self::L2FundedByL1),
            9 => Ok(Self::SubmitRetryable),
            11 => Ok(Self::Initialize),
            12 => Ok(Self::EthDeposit),
            13 => Ok(Self::BatchPostingReport),
            _ => Err(()),
        }
    }
}

impl L1MessageType {
    fn from_u8_panic(value: u8) -> Self {
        Self::try_from(value).unwrap_or_else(|_| panic!("Invalid L1MessageType value: {}", value))
    }
}

impl std::fmt::Display for L1MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
/// Builder for constructing Arbitrum blocks from transactions
pub struct ArbitrumAdapter {
    // Transaction parser for sequencing chain
    transaction_parser: SequencingTransactionParser,

    // Settlement chain address
    bridge_address: Address,

    // Settlement chain address
    inbox_address: Address,

    // Flag used to ignore Delayed messages (except Deposits)
    ignore_delayed_messages: bool,
}

impl Default for ArbitrumAdapter {
    fn default() -> Self {
        Self::new(&BlockBuilderConfig::default())
    }
}

#[async_trait]
impl RollupAdapter for ArbitrumAdapter {
    fn transaction_parser(&self) -> &SequencingTransactionParser {
        &self.transaction_parser
    }

    /// Builds a block from a slot
    async fn build_block_from_slot(
        &self,
        slot: &Slot,
        mchain_block_number: u64,
    ) -> Result<Vec<TransactionRequest>, eyre::Error> {
        let delayed_messages = self.process_delayed_messages(slot.settlement.clone()).await?;
        debug!("Delayed messages: {:?}", delayed_messages);

        let mb_transactions = self.parse_block_to_mbtxs(slot.sequencing.clone());

        if delayed_messages.is_empty() && mb_transactions.is_empty() {
            trace!("No delayed messages or MB transactions, skipping block");
            return Ok(Default::default());
        }

        // Always build a batch transaction even if there are no transactions
        // This ensures we always produce a block
        let batch_transaction = self
            .build_batch_txn(
                mb_transactions,
                mchain_block_number,
                slot.timestamp(),
                &slot.sequencing,
                slot.settlement.last(),
                delayed_messages.len(),
            )
            .await?;

        let mut result: Vec<TransactionRequest> = Vec::new();
        result.extend(delayed_messages);
        result.push(batch_transaction);
        Ok(result)
    }

    // NOTE: timestamp of the blockRefs will be 0
    async fn get_processed_blocks<T: Provider>(
        &self,
        provider: &T,
        block: BlockNumberOrTag,
    ) -> Result<Option<(KnownState, u64)>> {
        let rollup = Rollup::new(get_rollup_contract_address(), provider);

        let block_number = match block {
            BlockNumberOrTag::Number(num) => num,
            tag => {
                provider.get_block(BlockId::Number(tag)).await?.unwrap_or_default().header.number
            }
        };
        let block_id = BlockId::Number(BlockNumberOrTag::Number(block_number));

        let res = rollup.getSourceChainsProcessedBlocks().call().block(block_id).await?;
        let seq_num = res._seqBlockNumber;
        let seq_hash = res._seqBlockHash;
        let set_num = res._setBlockNumber;
        let set_hash = res._setBlockHash;

        if seq_num == 0 {
            return Ok(None);
        }

        Ok(Some((
            KnownState {
                mchain_block_number: block_number,
                sequencing_block: BlockRef { number: seq_num, timestamp: 0, hash: seq_hash.into() },
                settlement_block: BlockRef { number: set_num, timestamp: 0, hash: set_hash.into() },
            },
            block_number,
        )))
    }

    async fn get_last_sequencing_block_processed<T: Provider>(&self, provider: &T) -> Result<u64> {
        let rollup = Rollup::new(get_rollup_contract_address(), provider);
        let seq_num = rollup
            .seqBlockNumber()
            .call()
            .block(BlockId::Number(BlockNumberOrTag::Latest))
            .await?
            ._0;
        Ok(seq_num)
    }

    fn decode_error(&self, output: &Bytes) -> String {
        Rollup::RollupErrors::abi_decode(output, true).map_or_else(
            |_| String::from_utf8(output[4 + 32 + 32..].to_vec()).unwrap_or_default(),
            |decoded| match decoded {
                Rollup::RollupErrors::DataTooLarge(err) => format!(
                    "DataTooLarge, dataLength: {}, maxDataLength: {}",
                    err.dataLength, err.maxDataLength
                ),
            },
        )
    }

    fn interesting_sequencing_addresses(&self) -> Vec<Address> {
        vec![self.transaction_parser.sequencing_contract_address]
    }

    fn interesting_settlement_addresses(&self) -> Vec<Address> {
        vec![self.bridge_address, self.inbox_address]
    }
}

impl ArbitrumAdapter {
    /// Creates a new Arbitrum block builder.
    ///
    /// # Arguments
    /// - `config`: The configuration for the block builder.
    pub const fn new(config: &BlockBuilderConfig) -> Self {
        Self {
            transaction_parser: SequencingTransactionParser::new(
                config.sequencing_contract_address,
            ),
            bridge_address: config.arbitrum_bridge_address,
            inbox_address: config.arbitrum_inbox_address,
            ignore_delayed_messages: config.arbitrum_ignore_delayed_messages,
        }
    }

    /// Processes settlement chain receipts into delayed messages
    async fn process_delayed_messages(
        &self,
        blocks: Vec<Arc<BlockAndReceipts>>,
    ) -> Result<Vec<TransactionRequest>> {
        // Create a local map to store message data
        let mut message_data: HashMap<U256, Bytes> = HashMap::new();
        // Process all bridge logs in all receipts in all blocks
        let delayed_messages = blocks
            .iter()
            .flat_map(|block| &block.receipts)
            .flat_map(|receipt| &receipt.logs)
            .filter(|log| {
                log.address == self.bridge_address && log.topics[0] == MSG_DELIVERED_EVENT_HASH
            });

        // Process all inbox logs in all receipts in all blocks
        blocks
            .iter()
            .flat_map(|block| &block.receipts)
            .flat_map(|receipt| &receipt.logs)
            .filter(|log| log.address == self.inbox_address)
            .for_each(|log| {
                match log.topics[0] {
                    INBOX_MSG_DELIVERED_EVENT_HASH => {
                        let message_num = U256::from_be_slice(log.topics[1].as_slice());

                        // Decode the event using the contract bindings
                        match InboxMessageDelivered::abi_decode_data(&log.data, false) {
                            Ok(decoded) => {
                                message_data.insert(message_num, decoded.0);
                            }
                            Err(e) => {
                                panic!(
                                    "{}",
                                    ArbitrumBlockBuilderError::DecodingError(
                                        "InboxMessageDelivered",
                                        e.into()
                                    )
                                );
                            }
                        }
                    }

                    INBOX_MSG_DELIVERED_FROM_ORIGIN_EVENT_HASH => {
                        let message_num = U256::from_be_slice(log.topics[1].as_slice());

                        let block_index = (log.block_number - blocks[0].block.number) as usize;
                        let txn_index = log.transaction_index as usize;
                        let txn = blocks[block_index].block.transactions[txn_index].clone();

                        // Decode the transaction input using the contract bindings
                        match sendL2MessageFromOriginCall::abi_decode(&txn.input, false) {
                            Ok(decoded) => {
                                message_data.insert(message_num, decoded.messageData);
                            }
                            Err(e) => {
                                panic!(
                                    "{}",
                                    ArbitrumBlockBuilderError::DecodingError(
                                        "sendL2MessageFromOriginCall",
                                        e.into()
                                    )
                                );
                            }
                        }
                    }
                    _ => {}
                }
            });

        trace!("Delayed message data: {:?}", message_data);
        trace!("Delayed messages: {:?}", delayed_messages);

        let delayed_msg_txns: Vec<TransactionRequest> = delayed_messages
            .filter_map(|msg_log| {
                match self.delayed_message_to_mchain_txn(msg_log, message_data.clone()) {
                    Ok(txn) => Some(txn),
                    Err(e) => {
                        error!("Failed to process delayed message: {}", e);
                        None
                    }
                }
            })
            .collect();

        Ok(delayed_msg_txns)
    }

    fn delayed_message_to_mchain_txn(
        &self,
        log: &Log,
        message_data: HashMap<U256, Bytes>,
    ) -> Result<TransactionRequest> {
        let msg_delivered = MessageDelivered::abi_decode_data(&log.data, true)
            .map_err(|e| ArbitrumBlockBuilderError::DecodingError("MessageDelivered", e.into()))?;

        let message_index = U256::from_be_slice(log.topics[1].as_slice()); // First indexed field is message index
        let kind = L1MessageType::from_u8_panic(msg_delivered.1);

        if self.should_ignore_delayed_message(&kind) {
            return Err(ArbitrumBlockBuilderError::DelayedMessageIgnored(kind).into());
        }

        // Extract sender (third non-indexed field)
        let sender = msg_delivered.2;

        let data = message_data
            .get(&message_index)
            .ok_or_else(|| ArbitrumBlockBuilderError::MissingInboxMessageData(message_index))?;

        Ok(TransactionRequest::default().to(get_rollup_contract_address()).input(
            Rollup::deliverMessageCall { kind: kind as u8, sender, messageData: data.clone() }
                .abi_encode()
                .into(),
        ))
    }

    /// Builds a batch of transactions into an Arbitrum batch
    async fn build_batch_txn(
        &self,
        txs: Vec<Bytes>,
        mchain_block_number: u64,
        mchain_timestamp: u64,
        latest_sequencing_block: &Arc<BlockAndReceipts>,
        latest_settlement_block: Option<&Arc<BlockAndReceipts>>,
        delayed_message_count: usize,
    ) -> Result<TransactionRequest> {
        if delayed_message_count > 0 {
            info!("Adding {} delayed messages to batch", delayed_message_count);
        }
        let mut messages = vec![BatchMessage::Delayed; delayed_message_count];

        if !txs.is_empty() {
            info!("Adding {} sequenced transactions to batch", txs.len());
            debug!("Sequenced transactions: {:?}", txs);
            messages.push(BatchMessage::L2(L1IncomingMessage {
                header: L1IncomingMessageHeader {
                    block_number: mchain_block_number,
                    timestamp: mchain_timestamp,
                },
                l2_msg: txs,
            }));
        };

        let batch = Batch(messages);
        debug!("New Batch: {:?}", batch);

        // Encode the batch data
        let encoded_batch = batch.encode()?;

        let (set_num, set_hash) = latest_settlement_block
            .map_or((0, FixedBytes::ZERO), |b| (b.block.number, b.block.hash));

        // Create the transaction request
        let request = TransactionRequest::default().to(get_rollup_contract_address()).input(
            // Encode the function call with parameters
            Rollup::postBatchCall::new((
                encoded_batch,
                latest_sequencing_block.block.number,
                latest_sequencing_block.block.hash.into(),
                set_num,
                set_hash.into(),
            ))
            .abi_encode()
            .into(),
        );

        Ok(request)
    }

    fn should_ignore_delayed_message(&self, kind: &L1MessageType) -> bool {
        // If self.ignore_delayed_messages enabled, ignore everything except for EthDeposit
        if self.ignore_delayed_messages && *kind != L1MessageType::EthDeposit {
            debug!("Delayed message ignored. Kind: {:?}.", kind);
            return true;
        }

        // Ignore Initialize & BatchPostingReport message types
        if matches!(kind, L1MessageType::Initialize | L1MessageType::BatchPostingReport) {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{hex, keccak256, TxKind};
    use assert_matches::assert_matches;
    use common::types::{Block, BlockAndReceipts, Log, Receipt, Transaction};
    use contract_bindings::metabased::metabasedsequencerchain::MetabasedSequencerChain::TransactionProcessed;
    use std::{str::FromStr, sync::Arc};

    #[test]
    fn test_new_builder() {
        let sequencing_contract_address =
            Address::from_str("0x1234000000000000000000000000000000000000")
                .expect("Invalid address format");
        let config = BlockBuilderConfig {
            sequencing_contract_address,
            arbitrum_bridge_address: sequencing_contract_address,
            ..Default::default()
        };

        let builder = ArbitrumAdapter::new(&config);
        let parser = builder.transaction_parser();
        assert!(!std::ptr::eq(parser, std::ptr::null()), "Transaction parser should not be null");
    }

    #[tokio::test]
    async fn test_build_batch_empty_txs() {
        let builder = ArbitrumAdapter::default();
        let txs = vec![];
        let seq_block = create_mock_block_and_receipts(1, U256::from(1111).into(), vec![], vec![]);
        let set_block = create_mock_block_and_receipts(2, U256::from(2222).into(), vec![], vec![]);
        let batch =
            builder.build_batch_txn(txs, 0, 0, &seq_block, Some(&set_block), 0).await.unwrap();

        // Verify transaction is sent to sequencer inbox
        assert_eq!(batch.to, Some(TxKind::Call(get_rollup_contract_address())));

        // For empty batch, should create BatchMessage::Delayed
        let expected_batch = Batch(vec![]);
        let expected_encoded = expected_batch.encode().unwrap();

        // Verify the input data contains the correct parameters
        let call_data = Rollup::postBatchCall::new((
            expected_encoded, // batch data
            seq_block.block.number,
            seq_block.block.hash.into(),
            set_block.block.number,
            set_block.block.hash.into(),
        ))
        .abi_encode()
        .into();

        assert_eq!(batch.input, call_data);
    }

    #[tokio::test]
    async fn test_build_batch_with_txs() {
        let builder = ArbitrumAdapter::default();
        let txs = vec![
            hex!("1234").into(), // Sample transaction data
            hex!("5678").into(),
        ];
        let seq_block = create_mock_block_and_receipts(1, U256::from(1111).into(), vec![], vec![]);
        let set_block = create_mock_block_and_receipts(2, U256::from(2222).into(), vec![], vec![]);
        let batch = builder
            .build_batch_txn(txs.clone(), 0, 0, &seq_block, Some(&set_block), 0)
            .await
            .unwrap();

        // Verify transaction is sent to sequencer inbox
        assert_eq!(batch.to, Some(TxKind::Call(get_rollup_contract_address())));

        // For non-empty batch, should create BatchMessage::L2
        let expected_batch = Batch(vec![BatchMessage::L2(L1IncomingMessage {
            header: L1IncomingMessageHeader { timestamp: 0, block_number: 0 },
            l2_msg: txs,
        })]);
        let expected_encoded = expected_batch.encode().unwrap();

        // Verify the input data contains the correct parameters
        // Create a transaction request directly to compare with the actual output
        let call_data = Rollup::postBatchCall::new((
            expected_encoded, // batch data
            1,
            U256::from(1111),
            2,
            U256::from(2222),
        ))
        .abi_encode()
        .into();

        assert_eq!(batch.input, call_data);
    }

    #[tokio::test]
    async fn test_empty_slot() {
        let builder = ArbitrumAdapter::default();

        // Create an empty slot
        let slot = Slot { settlement: vec![], sequencing: Arc::new(BlockAndReceipts::default()) };

        let result = builder.build_block_from_slot(&slot, 1).await;
        assert!(result.is_ok());

        let txns = result.unwrap();
        assert_eq!(txns.len(), 0); // Should contain no transactions
    }

    fn create_mock_log(
        address: Address,
        topics: Vec<FixedBytes<32>>,
        data: Bytes,
        block_number: u64,
        transaction_index: u64,
    ) -> Log {
        Log { address, topics, data, block_number, transaction_index, ..Default::default() }
    }

    fn create_mock_block_and_receipts(
        number: u64,
        hash: FixedBytes<32>,
        transactions: Vec<Transaction>,
        receipts: Vec<Receipt>,
    ) -> Arc<BlockAndReceipts> {
        Arc::new(BlockAndReceipts {
            block: Block { number, hash, transactions, ..Default::default() },
            receipts,
        })
    }

    #[tokio::test]
    async fn test_build_block_from_slot_with_delayed_messages() {
        let builder = ArbitrumAdapter::default();

        // Create message data
        let message_index = U256::from(1);
        let before_inbox_acc = FixedBytes::from([1u8; 32]);
        let message_data: Bytes = hex!("1234").into();

        let msg_delivered_event = MessageDelivered {
            messageIndex: message_index,
            beforeInboxAcc: before_inbox_acc,
            inbox: builder.inbox_address,
            kind: L1MessageType::L2Message as u8,
            sender: Address::ZERO,
            messageDataHash: keccak256(message_data.clone()),
            baseFeeL1: U256::ZERO,
            timestamp: 0u64,
        };

        // Create mock logs
        let msg_delivered_log = create_mock_log(
            builder.bridge_address,
            vec![
                MSG_DELIVERED_EVENT_HASH,
                FixedBytes::from(message_index.to_be_bytes()),
                before_inbox_acc,
            ],
            msg_delivered_event.encode_data().into(),
            1,
            0,
        );

        let inbox_msg_log = create_mock_log(
            builder.inbox_address,
            vec![INBOX_MSG_DELIVERED_EVENT_HASH, FixedBytes::from(message_index.to_be_bytes())],
            InboxMessageDelivered { messageNum: message_index, data: message_data }
                .encode_data()
                .into(),
            1,
            0,
        );

        // Create mock block with receipts
        let block = create_mock_block_and_receipts(
            1,
            U256::from(1111).into(),
            vec![],
            vec![Receipt { logs: vec![msg_delivered_log, inbox_msg_log], ..Default::default() }],
        );

        let slot =
            Slot { settlement: vec![block], sequencing: Arc::new(BlockAndReceipts::default()) };

        let result = builder.build_block_from_slot(&slot, 1).await;
        assert!(result.is_ok());

        let txns = result.unwrap();
        assert_eq!(txns.len(), 2); // Should contain batch transaction + delayed message transaction

        // Verify delayed message transaction
        assert_eq!(txns[0].to, Some(TxKind::Call(get_rollup_contract_address())));

        // Verify the batch transaction
        assert_eq!(txns[1].to, Some(TxKind::Call(get_rollup_contract_address())));
    }

    #[tokio::test]
    async fn test_build_block_from_slot_with_sequencing_txns() {
        let builder = ArbitrumAdapter::default();

        // Create a mock L2 transaction
        let txn_data: Bytes = hex!("001234").into();
        let txn_processed_event =
            TransactionProcessed { sender: Address::ZERO, data: txn_data.clone() };

        let txn_processed_log = create_mock_log(
            builder.transaction_parser.sequencing_contract_address,
            vec![TransactionProcessed::SIGNATURE_HASH, Address::ZERO.into_word()],
            txn_processed_event.encode_data().into(),
            1,
            0,
        );
        let block =
            Block { number: 1, transactions: vec![Transaction::default()], ..Default::default() };

        let slot = Slot {
            settlement: vec![],
            sequencing: Arc::new(BlockAndReceipts {
                block,
                receipts: vec![Receipt { logs: vec![txn_processed_log], ..Default::default() }],
            }),
        };

        let result = builder.build_block_from_slot(&slot, 1).await;
        assert!(result.is_ok());

        let txns = result.unwrap();
        assert_eq!(txns.len(), 1);

        // Verify the batch transaction contains our tx data
        let batch_txn = &txns[0];
        assert_eq!(batch_txn.to, Some(TxKind::Call(get_rollup_contract_address())));
        let txn_data_without_prefix = txn_data[1..].to_vec();
        let expected_batch = Batch(vec![BatchMessage::L2(L1IncomingMessage {
            header: L1IncomingMessageHeader { block_number: 1, timestamp: 0 },
            l2_msg: vec![txn_data_without_prefix.into()],
        })]);
        let expected_encoded = expected_batch.encode().unwrap();

        let batch_txn_input = batch_txn.input.clone();
        let expected_batch_call = Rollup::postBatchCall::new((
            expected_encoded,
            1,             // sequencing block number
            U256::from(0), // sequencing block hash (default is zero in the test)
            0,             // settlement block number
            U256::from(0), // settlement block hash (default is zero in the test)
        ))
        .abi_encode()
        .into();

        assert_eq!(batch_txn_input, expected_batch_call);
    }

    #[tokio::test]
    async fn test_build_block_from_slot_with_sequencing_and_delayed_txns() {
        let builder = ArbitrumAdapter::default();

        // Create a mock L2 transaction
        let txn_data: Bytes = hex!("001234").into();
        let txn_processed_event =
            TransactionProcessed { sender: Address::ZERO, data: txn_data.clone() };

        let txn_processed_log = create_mock_log(
            builder.transaction_parser.sequencing_contract_address,
            vec![TransactionProcessed::SIGNATURE_HASH, Address::ZERO.into_word()],
            txn_processed_event.encode_data().into(),
            1,
            0,
        );

        let sequencing_receipt = Receipt { logs: vec![txn_processed_log], ..Default::default() };
        let sequencing_block =
            Block { number: 1, transactions: vec![Transaction::default()], ..Default::default() };

        // Create mock delayed message logs
        let message_num = U256::from(1);
        let before_inbox_acc = FixedBytes::from([1u8; 32]);
        let delayed_message_data: Bytes = hex!("5678").into();

        let msg_delivered_event = MessageDelivered {
            messageIndex: message_num,
            beforeInboxAcc: before_inbox_acc,
            inbox: builder.inbox_address,
            kind: L1MessageType::L2Message as u8,
            sender: Address::repeat_byte(1),
            messageDataHash: keccak256(delayed_message_data.clone()),
            baseFeeL1: U256::ZERO,
            timestamp: 0u64,
        };

        let delayed_log = create_mock_log(
            builder.bridge_address,
            vec![
                MSG_DELIVERED_EVENT_HASH,
                FixedBytes::from(message_num.to_be_bytes::<32>()),
                before_inbox_acc,
            ],
            msg_delivered_event.encode_data().into(),
            1,
            0,
        );

        let inbox_log = create_mock_log(
            builder.inbox_address,
            vec![INBOX_MSG_DELIVERED_EVENT_HASH, FixedBytes::from(message_num.to_be_bytes::<32>())],
            InboxMessageDelivered { messageNum: message_num, data: delayed_message_data.clone() }
                .encode_data()
                .into(),
            1,
            0,
        );

        let settlement_block = Block { number: 1, ..Default::default() };
        let settlement_receipt =
            Receipt { logs: vec![delayed_log, inbox_log], ..Default::default() };

        let slot = Slot {
            settlement: vec![Arc::new(BlockAndReceipts {
                block: settlement_block,
                receipts: vec![settlement_receipt],
            })],
            sequencing: Arc::new(BlockAndReceipts {
                block: sequencing_block,
                receipts: vec![sequencing_receipt],
            }),
        };

        let result = builder.build_block_from_slot(&slot, 1).await;
        assert!(result.is_ok());

        let txns = result.unwrap();
        assert_eq!(txns.len(), 2); // Should contain batch transaction + delayed message transaction

        // Verify delayed message transaction
        let delayed_tx = &txns[0];
        assert_eq!(delayed_tx.to, Some(TxKind::Call(get_rollup_contract_address())));
        let expected_delayed_call = Rollup::deliverMessageCall {
            kind: L1MessageType::L2Message as u8,
            sender: Address::repeat_byte(1),
            messageData: delayed_message_data,
        }
        .abi_encode()
        .into();
        assert_eq!(delayed_tx.input, expected_delayed_call);

        // Verify the batch transaction contains our sequencing tx data
        let batch_txn = &txns[1];
        let txn_data_without_prefix = txn_data[1..].to_vec();
        assert_eq!(batch_txn.to, Some(TxKind::Call(get_rollup_contract_address())));
        let expected_batch = Batch(vec![
            BatchMessage::Delayed,
            BatchMessage::L2(L1IncomingMessage {
                header: L1IncomingMessageHeader { block_number: 1, timestamp: 0 },
                l2_msg: vec![txn_data_without_prefix.into()],
            }),
        ]);
        let expected_encoded = expected_batch.encode().unwrap();
        let expected_batch_call =
            Rollup::postBatchCall::new((expected_encoded, 1, U256::from(0), 1, U256::from(0)))
                .abi_encode()
                .into();
        assert_eq!(batch_txn.input, expected_batch_call);
    }

    #[test]
    fn test_delayed_message_to_mchain_txn_success() {
        let builder = ArbitrumAdapter::default();

        // Create message data
        let message_index = U256::from(1);
        let message_data: Bytes = hex!("1234").into();
        let mut message_map = HashMap::new();
        message_map.insert(message_index, message_data.clone());

        // Create MessageDelivered event data
        let msg_delivered = MessageDelivered {
            messageIndex: message_index,
            beforeInboxAcc: FixedBytes::from([1u8; 32]),
            inbox: builder.inbox_address,
            kind: L1MessageType::L2Message as u8,
            sender: Address::repeat_byte(1),
            messageDataHash: keccak256(message_data.clone()),
            baseFeeL1: U256::ZERO,
            timestamp: 0u64,
        };

        // Create the log
        let log = Log {
            address: builder.bridge_address,
            topics: vec![
                MSG_DELIVERED_EVENT_HASH,
                FixedBytes::from(message_index.to_be_bytes::<32>()),
                FixedBytes::from([1u8; 32]),
            ],
            data: msg_delivered.encode_data().into(),
            block_number: 1,
            transaction_index: 0,
            ..Default::default()
        };

        // Call the function
        let result = builder.delayed_message_to_mchain_txn(&log, message_map);
        assert!(result.is_ok());

        let txn = result.unwrap();

        // Verify the transaction
        assert_eq!(txn.to, Some(TxKind::Call(get_rollup_contract_address())));

        // Verify the input data matches expected deliverMessageCall
        let expected_call = Rollup::deliverMessageCall {
            kind: L1MessageType::L2Message as u8,
            sender: Address::repeat_byte(1),
            messageData: message_data,
        }
        .abi_encode()
        .into();
        assert_eq!(txn.input, expected_call);
    }

    #[test]
    fn test_delayed_message_to_mchain_txn_missing_data() {
        let builder = ArbitrumAdapter::default();

        // Create MessageDelivered event data without corresponding message data
        let message_index = U256::from(1);
        let msg_delivered = MessageDelivered {
            messageIndex: message_index,
            beforeInboxAcc: FixedBytes::from([1u8; 32]),
            inbox: builder.inbox_address,
            kind: L1MessageType::L2Message as u8,
            sender: Address::repeat_byte(1),
            messageDataHash: FixedBytes::from([2u8; 32]),
            baseFeeL1: U256::ZERO,
            timestamp: 0u64,
        };

        let log = Log {
            address: builder.bridge_address,
            topics: vec![
                MSG_DELIVERED_EVENT_HASH,
                FixedBytes::from(message_index.to_be_bytes::<32>()),
                FixedBytes::from([1u8; 32]),
            ],
            data: msg_delivered.encode_data().into(),
            block_number: 1,
            transaction_index: 0,
            ..Default::default()
        };

        // Empty message data map
        let message_map = HashMap::new();

        // Call should fail with MissingInboxMessageData error
        let result = builder.delayed_message_to_mchain_txn(&log, message_map);
        assert!(result.is_err());
        assert_matches!(
            result.unwrap_err().downcast::<ArbitrumBlockBuilderError>().unwrap(),
            ArbitrumBlockBuilderError::MissingInboxMessageData(_)
        );
    }

    #[test]
    fn test_delayed_message_to_mchain_txn_invalid_event_data() {
        let builder = ArbitrumAdapter::default();
        let message_map = HashMap::new();

        // Create log with invalid event data
        let log = Log {
            address: builder.bridge_address,
            topics: vec![MSG_DELIVERED_EVENT_HASH],
            data: Bytes::from(vec![1, 2, 3]), // Invalid data that can't be decoded
            block_number: 1,
            transaction_index: 0,
            ..Default::default()
        };

        // Call should fail with DecodingError
        let result = builder.delayed_message_to_mchain_txn(&log, message_map);
        assert!(result.is_err());
        assert_matches!(
            result.unwrap_err().downcast::<ArbitrumBlockBuilderError>().unwrap(),
            ArbitrumBlockBuilderError::DecodingError(_, _)
        );
    }

    #[test]
    fn test_delayed_message_to_mchain_txn_ignore_message() {
        let builder = ArbitrumAdapter::new(&BlockBuilderConfig {
            arbitrum_ignore_delayed_messages: true,
            ..Default::default()
        });

        // Create message data
        let message_index = U256::from(1);
        let message_data: Bytes = hex!("1234").into();
        let mut message_map = HashMap::new();
        message_map.insert(message_index, message_data.clone());

        // Create MessageDelivered event data
        let msg_delivered = MessageDelivered {
            messageIndex: message_index,
            beforeInboxAcc: FixedBytes::from([1u8; 32]),
            inbox: builder.inbox_address,
            kind: L1MessageType::L2Message as u8,
            sender: Address::repeat_byte(1),
            messageDataHash: keccak256(message_data),
            baseFeeL1: U256::ZERO,
            timestamp: 0u64,
        };

        // Create the log
        let log = Log {
            address: builder.bridge_address,
            topics: vec![
                MSG_DELIVERED_EVENT_HASH,
                FixedBytes::from(message_index.to_be_bytes::<32>()),
                FixedBytes::from([1u8; 32]),
            ],
            data: msg_delivered.encode_data().into(),
            block_number: 1,
            transaction_index: 0,
            ..Default::default()
        };

        // Call the function
        let result = builder.delayed_message_to_mchain_txn(&log, message_map);
        assert!(result.is_err());
        assert_matches!(
            result.unwrap_err().downcast::<ArbitrumBlockBuilderError>().unwrap(),
            ArbitrumBlockBuilderError::DelayedMessageIgnored(_)
        );
    }

    #[test]
    fn test_delayed_message_to_mchain_txn_do_not_ignore_deposit() {
        let builder = ArbitrumAdapter::new(&BlockBuilderConfig {
            arbitrum_ignore_delayed_messages: true,
            ..Default::default()
        });

        // Create message data
        let message_index = U256::from(1);
        let message_data: Bytes = hex!("1234").into();
        let mut message_map = HashMap::new();
        message_map.insert(message_index, message_data.clone());

        // Create MessageDelivered event data
        let msg_delivered = MessageDelivered {
            messageIndex: message_index,
            beforeInboxAcc: FixedBytes::from([1u8; 32]),
            inbox: builder.inbox_address,
            kind: L1MessageType::EthDeposit as u8,
            sender: Address::repeat_byte(1),
            messageDataHash: keccak256(message_data.clone()),
            baseFeeL1: U256::ZERO,
            timestamp: 0u64,
        };

        // Create the log
        let log = Log {
            address: builder.bridge_address,
            topics: vec![
                MSG_DELIVERED_EVENT_HASH,
                FixedBytes::from(message_index.to_be_bytes::<32>()),
                FixedBytes::from([1u8; 32]),
            ],
            data: msg_delivered.encode_data().into(),
            block_number: 1,
            transaction_index: 0,
            ..Default::default()
        };

        let result = builder.delayed_message_to_mchain_txn(&log, message_map);
        assert!(result.is_ok());

        let txn = result.unwrap();

        // Verify the transaction
        assert_eq!(txn.to, Some(TxKind::Call(get_rollup_contract_address())));

        // Verify the input data matches expected deliverMessageCall
        let expected_call = Rollup::deliverMessageCall {
            kind: L1MessageType::EthDeposit as u8,
            sender: Address::repeat_byte(1),
            messageData: message_data,
        }
        .abi_encode()
        .into();
        assert_eq!(txn.input, expected_call);
    }

    #[test]
    fn test_should_ignore_delayed_message() {
        let builder = ArbitrumAdapter::new(&BlockBuilderConfig {
            arbitrum_ignore_delayed_messages: true,
            ..Default::default()
        });

        assert!(builder.should_ignore_delayed_message(&L1MessageType::L2Message));
        assert!(builder.should_ignore_delayed_message(&L1MessageType::L2FundedByL1));
        assert!(builder.should_ignore_delayed_message(&L1MessageType::SubmitRetryable));
        assert!(builder.should_ignore_delayed_message(&L1MessageType::Initialize));
        assert!(builder.should_ignore_delayed_message(&L1MessageType::BatchPostingReport));

        // Message that should NOT be ignored (even if ignore_delayed_messages is true)
        assert!(!builder.should_ignore_delayed_message(&L1MessageType::EthDeposit));

        let builder = ArbitrumAdapter::new(&BlockBuilderConfig {
            arbitrum_ignore_delayed_messages: false,
            ..Default::default()
        });

        assert!(!builder.should_ignore_delayed_message(&L1MessageType::L2Message));
        assert!(!builder.should_ignore_delayed_message(&L1MessageType::L2FundedByL1));
        assert!(!builder.should_ignore_delayed_message(&L1MessageType::SubmitRetryable));
        assert!(!builder.should_ignore_delayed_message(&L1MessageType::EthDeposit));

        assert!(builder.should_ignore_delayed_message(&L1MessageType::Initialize));
        assert!(builder.should_ignore_delayed_message(&L1MessageType::BatchPostingReport));
    }
}
