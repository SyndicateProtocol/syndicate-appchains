//! Arbitrum block builder implementation
//!
//! This module provides functionality for building an Arbitrum batch submitter transaction from a
//! list of transactions. It implements the [`RollupBlockBuilder`] trait to standardize block
//! construction across different rollup implementations

use crate::{
    config::BlockBuilderConfig,
    rollups::{
        arbitrum::batch::{Batch, BatchMessage, L1IncomingMessage, L1IncomingMessageHeader},
        shared::{RollupBlockBuilder, SequencingTransactionParser},
    },
};
use alloy::{
    primitives::{Address, Bytes, FixedBytes, U256},
    rpc::types::TransactionRequest,
    sol_types::{SolCall, SolEvent},
};
use async_trait::async_trait;
use common::types::{BlockAndReceipts, Log, Slot};
use contract_bindings::arbitrum::{
    ibridge::IBridge::MessageDelivered,
    idelayedmessageprovider::IDelayedMessageProvider::{
        InboxMessageDelivered, InboxMessageDeliveredFromOrigin,
    },
    iinboxbase::IInboxBase::sendL2MessageFromOriginCall,
    rollup::Rollup,
};
use eyre::Result;
use slotting::config::FIRST_SLOT;
use std::collections::HashMap;
use thiserror::Error;
use tracing::{debug, error};

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
#[derive(Debug)]
pub enum L1MessageType {
    L2Message = 3,
    L2FundedByL1 = 7,
    SubmitRetryable = 9,
    Initialize = 11,
    EthDeposit = 12,
    BatchPostingReport = 13,
}

impl std::fmt::Display for L1MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
/// Builder for constructing Arbitrum blocks from transactions
pub struct ArbitrumBlockBuilder {
    // MChain rollup address
    mchain_rollup_address: Address,

    // Sequencing chain address
    transaction_parser: SequencingTransactionParser,

    // Settlement chain address
    delayed_inbox_address: Address,
}

impl Default for ArbitrumBlockBuilder {
    fn default() -> Self {
        let config = BlockBuilderConfig::default();
        Self::new(config)
    }
}

#[async_trait]
impl RollupBlockBuilder for ArbitrumBlockBuilder {
    fn transaction_parser(&self) -> &SequencingTransactionParser {
        &self.transaction_parser
    }

    /// Builds a block from a slot
    async fn build_block_from_slot(
        &mut self,
        slot: Slot,
    ) -> Result<Vec<TransactionRequest>, eyre::Error> {
        let delayed_messages = self.process_delayed_messages(slot.settlement_chain_blocks).await?;

        let mb_transactions = self.parse_blocks_to_mbtxs(slot.sequencing_chain_blocks);

        if delayed_messages.is_empty() &&
            mb_transactions.is_empty() &&
            slot.slot_number != FIRST_SLOT
        {
            return Ok(Default::default());
        }

        let batch_transaction = self
            .build_batch_txn(
                mb_transactions,
                slot.slot_number,
                slot.timestamp,
                // include the pre-existing init message in the first batch
                delayed_messages.len() + ((slot.slot_number == FIRST_SLOT) as usize),
            )
            .await?;

        let mut result: Vec<TransactionRequest> = Vec::new();
        result.extend(delayed_messages);
        result.push(batch_transaction);
        Ok(result)
    }
}

impl ArbitrumBlockBuilder {
    /// Creates a new Arbitrum block builder.
    ///
    /// # Arguments
    /// - `config`: The configuration for the block builder.
    pub fn new(config: BlockBuilderConfig) -> Self {
        Self {
            transaction_parser: SequencingTransactionParser::new(
                config.sequencing_contract_address,
            ),
            mchain_rollup_address: config.mchain_rollup_address,
            delayed_inbox_address: config.delayed_inbox_address,
        }
    }

    /// Processes settlement chain receipts into delayed messages
    async fn process_delayed_messages(
        &self,
        blocks: Vec<BlockAndReceipts>,
    ) -> Result<Vec<TransactionRequest>> {
        // Create a local map to store message data
        let mut message_data: HashMap<U256, Bytes> = HashMap::new();

        // Process all logs in all receipts in all blocks
        let delayed_messages = blocks
            .iter()
            .flat_map(|block| &block.receipts)
            .flat_map(|receipt| &receipt.logs)
            .filter(|log| Address::from_slice(log.address.as_slice()) == self.delayed_inbox_address)
            .filter(|log| {
                let topic_bytes = FixedBytes::from_slice(log.topics[0].as_slice());

                match topic_bytes {
                    MSG_DELIVERED_EVENT_HASH => true,

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
                        false
                    }

                    INBOX_MSG_DELIVERED_FROM_ORIGIN_EVENT_HASH => {
                        let message_num = U256::from_be_slice(log.topics[1].as_slice());

                        let block_index = (log.block_number - blocks[0].block.number) as usize;
                        let txn_index = log.transaction_index as usize;
                        let txn = blocks[block_index].block.transactions[txn_index].clone();

                        // Decode the transaction input using the contract bindings
                        match sendL2MessageFromOriginCall::abi_decode(txn.input.as_bytes(), false) {
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

                        false
                    }

                    _ => false,
                }
            })
            .collect::<Vec<_>>();

        debug!("Delayed message data: {:?}", message_data);
        debug!("Delayed messages: {:?}", delayed_messages);

        let delayed_msg_txns: Vec<TransactionRequest> = delayed_messages
            .iter()
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
        let msg_delivered = match MessageDelivered::abi_decode_data(&log.data, true) {
            Ok(decoded) => decoded,
            Err(e) => {
                return Err(
                    ArbitrumBlockBuilderError::DecodingError("MessageDelivered", e.into()).into()
                );
            }
        };

        let message_index = U256::from_be_slice(log.topics[1].as_slice()); // First indexed field is message index
        let kind = msg_delivered.1; // Second non-indexed field is kind
        if kind == L1MessageType::Initialize as u8 {
            return Err(
                ArbitrumBlockBuilderError::DelayedMessageIgnored(L1MessageType::Initialize).into()
            );
        }
        if kind == L1MessageType::BatchPostingReport as u8 {
            return Err(ArbitrumBlockBuilderError::DelayedMessageIgnored(
                L1MessageType::BatchPostingReport,
            )
            .into());
        }
        if kind != L1MessageType::L2Message as u8 &&
            kind != L1MessageType::L2FundedByL1 as u8 &&
            kind != L1MessageType::SubmitRetryable as u8 &&
            kind != L1MessageType::EthDeposit as u8
        {
            panic!("unexpected message kind={}", kind);
        }
        let sender = msg_delivered.2; // Third non-indexed field is sender

        let data = match message_data.get(&message_index) {
            Some(data) => data,
            None => {
                return Err(ArbitrumBlockBuilderError::MissingInboxMessageData(message_index).into());
            }
        };

        let delivered_msg_txn = TransactionRequest::default().to(self.mchain_rollup_address).input(
            Rollup::deliverMessageCall { kind, sender, messageData: data.clone() }
                .abi_encode()
                .into(),
        );

        Ok(delivered_msg_txn)
    }

    /// Builds a batch of transactions into an Arbitrum batch
    async fn build_batch_txn(
        &self,
        txs: Vec<Bytes>,
        slot_number: u64,
        slot_timestamp: u64,
        delayed_message_count: usize,
    ) -> Result<TransactionRequest> {
        let mut messages = vec![BatchMessage::Delayed; delayed_message_count];

        if !txs.is_empty() {
            messages.push(BatchMessage::L2(L1IncomingMessage {
                header: L1IncomingMessageHeader {
                    block_number: slot_number,
                    timestamp: slot_timestamp,
                },
                l2_msg: txs,
            }));
        };

        let batch = Batch(messages);

        // Encode the batch data
        let encoded_batch = batch.encode()?;

        // Create the transaction request
        let request = TransactionRequest::default().to(self.mchain_rollup_address).input(
            // Encode the function call with parameters
            Rollup::postBatchCall::new((encoded_batch,)).abi_encode().into(), // Convert the tokenized call data to bytes
        );

        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{hex, keccak256, TxKind};
    use assert_matches::assert_matches;
    use common::types::{Block, Log, Receipt, SlotState, Transaction};
    use contract_bindings::metabased::metabasedsequencerchain::MetabasedSequencerChain::TransactionProcessed;
    use std::str::FromStr;

    #[test]
    fn test_new_builder() {
        let sequencing_contract_address =
            Address::from_str("0x1234000000000000000000000000000000000000")
                .expect("Invalid address format");
        let config = BlockBuilderConfig {
            sequencing_contract_address,
            mchain_rollup_address: sequencing_contract_address,
            delayed_inbox_address: sequencing_contract_address,
            ..Default::default()
        };

        let builder = ArbitrumBlockBuilder::new(config);
        let parser = builder.transaction_parser();
        assert!(!std::ptr::eq(parser, std::ptr::null()), "Transaction parser should not be null");
    }

    #[tokio::test]
    async fn test_build_batch_empty_txs() {
        let builder = ArbitrumBlockBuilder::default();
        let txs = vec![];
        let batch = builder.build_batch_txn(txs, 0, 0, 0).await.unwrap();

        // Verify transaction is sent to sequencer inbox
        assert_eq!(batch.to, Some(TxKind::Call(builder.mchain_rollup_address)));

        // For empty batch, should create BatchMessage::Delayed
        let expected_batch = Batch(vec![]);
        let expected_encoded = expected_batch.encode().unwrap();

        // Verify the input data contains the correct parameters
        let call_data = Rollup::postBatchCall::new((
            expected_encoded, // batch data
        ))
        .abi_encode();

        assert_eq!(batch.input, call_data.into());
    }

    #[tokio::test]
    async fn test_build_batch_with_txs() {
        let builder = ArbitrumBlockBuilder::default();
        let txs = vec![
            hex!("1234").into(), // Sample transaction data
            hex!("5678").into(),
        ];
        let batch = builder.build_batch_txn(txs.clone(), 0, 0, 0).await.unwrap();

        // Verify transaction is sent to sequencer inbox
        assert_eq!(batch.to, Some(TxKind::Call(builder.mchain_rollup_address)));

        // For non-empty batch, should create BatchMessage::L2
        let expected_batch = Batch(vec![BatchMessage::L2(L1IncomingMessage {
            header: Default::default(),
            l2_msg: txs,
        })]);
        let expected_encoded = expected_batch.encode().unwrap();

        // Verify the input data contains the correct parameters
        let call_data = Rollup::postBatchCall::new((
            expected_encoded, // batch data
        ))
        .abi_encode();

        assert_eq!(batch.input, call_data.into());
    }

    #[tokio::test]
    async fn test_empty_slot() {
        let mut builder = ArbitrumBlockBuilder::default();

        // Create an empty slot
        let slot = Slot {
            slot_number: 1,
            timestamp: 0,
            state: SlotState::Safe,
            settlement_chain_blocks: vec![],
            sequencing_chain_blocks: vec![],
        };

        let result = builder.build_block_from_slot(slot).await;
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
        transactions: Vec<Transaction>,
        receipts: Vec<Receipt>,
    ) -> BlockAndReceipts {
        BlockAndReceipts { block: Block { number, transactions, ..Default::default() }, receipts }
    }

    #[tokio::test]
    async fn test_build_block_from_slot_with_delayed_messages() {
        let builder = ArbitrumBlockBuilder::default();
        let mut builder = builder;

        // Create message data
        let message_index = U256::from(1);
        let before_inbox_acc = FixedBytes::from([1u8; 32]);
        let message_data: Bytes = hex!("1234").into();

        let msg_delivered_event = MessageDelivered {
            messageIndex: message_index,
            beforeInboxAcc: before_inbox_acc,
            inbox: builder.delayed_inbox_address,
            kind: L1MessageType::L2Message as u8,
            sender: Address::ZERO,
            messageDataHash: keccak256(message_data.clone()),
            baseFeeL1: U256::ZERO,
            timestamp: 0u64,
        };

        // Create mock logs
        let msg_delivered_log = create_mock_log(
            builder.delayed_inbox_address,
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
            builder.delayed_inbox_address,
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
            vec![],
            vec![Receipt { logs: vec![msg_delivered_log, inbox_msg_log], ..Default::default() }],
        );

        let slot = Slot {
            slot_number: 1,
            timestamp: 0,
            state: SlotState::Safe,
            settlement_chain_blocks: vec![block],
            sequencing_chain_blocks: vec![],
        };

        let result = builder.build_block_from_slot(slot).await;
        assert!(result.is_ok());

        let txns = result.unwrap();
        assert_eq!(txns.len(), 2); // Should contain batch transaction + delayed message transaction

        // Verify delayed message transaction
        assert_eq!(txns[0].to, Some(TxKind::Call(builder.mchain_rollup_address)));

        // Verify the batch transaction
        assert_eq!(txns[1].to, Some(TxKind::Call(builder.mchain_rollup_address)));
    }

    #[tokio::test]
    async fn test_build_block_from_slot_with_sequencing_txns() {
        let builder = ArbitrumBlockBuilder::default();
        let mut builder = builder;

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
            slot_number: 1,
            timestamp: 0,
            state: SlotState::Safe,
            settlement_chain_blocks: vec![],
            sequencing_chain_blocks: vec![BlockAndReceipts {
                block,
                receipts: vec![Receipt { logs: vec![txn_processed_log], ..Default::default() }],
            }],
        };

        let result = builder.build_block_from_slot(slot).await;
        assert!(result.is_ok());

        let txns = result.unwrap();
        assert_eq!(txns.len(), 1);

        // Verify the batch transaction contains our tx data
        let batch_txn = &txns[0];
        assert_eq!(batch_txn.to, Some(TxKind::Call(builder.mchain_rollup_address)));
        let txn_data_without_prefix = txn_data[1..].to_vec();
        let expected_batch = Batch(vec![BatchMessage::L2(L1IncomingMessage {
            header: L1IncomingMessageHeader { block_number: 1, timestamp: 0 },
            l2_msg: vec![txn_data_without_prefix.into()],
        })]);
        let expected_encoded = expected_batch.encode().unwrap();
        let expected_batch_call =
            Rollup::postBatchCall::new((expected_encoded,)).abi_encode().into();
        assert_eq!(batch_txn.input, expected_batch_call);
    }

    #[tokio::test]
    async fn test_build_block_from_slot_with_sequencing_and_delayed_txns() {
        let builder = ArbitrumBlockBuilder::default();
        let mut builder = builder;

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
            inbox: builder.delayed_inbox_address,
            kind: L1MessageType::L2Message as u8,
            sender: Address::repeat_byte(1),
            messageDataHash: keccak256(delayed_message_data.clone()),
            baseFeeL1: U256::ZERO,
            timestamp: 0u64,
        };

        let delayed_log = create_mock_log(
            builder.delayed_inbox_address,
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
            builder.delayed_inbox_address,
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
            slot_number: 1,
            timestamp: 0,
            state: SlotState::Safe,
            settlement_chain_blocks: vec![BlockAndReceipts {
                block: settlement_block,
                receipts: vec![settlement_receipt],
            }],
            sequencing_chain_blocks: vec![BlockAndReceipts {
                block: sequencing_block,
                receipts: vec![sequencing_receipt],
            }],
        };

        let result = builder.build_block_from_slot(slot).await;
        assert!(result.is_ok());

        let txns = result.unwrap();
        assert_eq!(txns.len(), 2); // Should contain batch transaction + delayed message transaction

        // Verify delayed message transaction
        let delayed_tx = &txns[0];
        assert_eq!(delayed_tx.to, Some(TxKind::Call(builder.mchain_rollup_address)));
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
        assert_eq!(batch_txn.to, Some(TxKind::Call(builder.mchain_rollup_address)));
        let expected_batch = Batch(vec![
            BatchMessage::Delayed,
            BatchMessage::L2(L1IncomingMessage {
                header: L1IncomingMessageHeader { block_number: 1, timestamp: 0 },
                l2_msg: vec![txn_data_without_prefix.into()],
            }),
        ]);
        let expected_encoded = expected_batch.encode().unwrap();
        let expected_batch_call =
            Rollup::postBatchCall::new((expected_encoded,)).abi_encode().into();
        assert_eq!(batch_txn.input, expected_batch_call);
    }

    #[test]
    fn test_delayed_message_to_mchain_txn_success() {
        let builder = ArbitrumBlockBuilder::default();

        // Create message data
        let message_index = U256::from(1);
        let message_data: Bytes = hex!("1234").into();
        let mut message_map = HashMap::new();
        message_map.insert(message_index, message_data.clone());

        // Create MessageDelivered event data
        let msg_delivered = MessageDelivered {
            messageIndex: message_index,
            beforeInboxAcc: FixedBytes::from([1u8; 32]),
            inbox: builder.delayed_inbox_address,
            kind: L1MessageType::L2Message as u8,
            sender: Address::repeat_byte(1),
            messageDataHash: keccak256(message_data.clone()),
            baseFeeL1: U256::ZERO,
            timestamp: 0u64,
        };

        // Create the log
        let log = Log {
            address: builder.delayed_inbox_address,
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
        assert_eq!(txn.to, Some(TxKind::Call(builder.mchain_rollup_address)));

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
        let builder = ArbitrumBlockBuilder::default();

        // Create MessageDelivered event data without corresponding message data
        let message_index = U256::from(1);
        let msg_delivered = MessageDelivered {
            messageIndex: message_index,
            beforeInboxAcc: FixedBytes::from([1u8; 32]),
            inbox: builder.delayed_inbox_address,
            kind: L1MessageType::L2Message as u8,
            sender: Address::repeat_byte(1),
            messageDataHash: FixedBytes::from([2u8; 32]),
            baseFeeL1: U256::ZERO,
            timestamp: 0u64,
        };

        let log = Log {
            address: builder.delayed_inbox_address,
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
        let builder = ArbitrumBlockBuilder::default();
        let message_map = HashMap::new();

        // Create log with invalid event data
        let log = Log {
            address: builder.delayed_inbox_address,
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
}
