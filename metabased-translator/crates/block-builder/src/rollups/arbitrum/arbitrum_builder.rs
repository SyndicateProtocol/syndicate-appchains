//! Arbitrum block builder implementation
//!
//! This module provides functionality for building an Arbitrum batch submitter transaction from a
//! list of transactions. It implements the [`RollupBlockBuilder`] trait to standardize block
//! construction across different rollup implementations

use crate::rollups::{
    arbitrum::batch::{Batch, BatchMessage, L1IncomingMessage},
    shared::{RollupBlockBuilder, SequencingTransactionParser},
};
use alloy::{
    primitives::{address, Address, Bytes, FixedBytes, U256},
    rpc::types::TransactionRequest,
    sol_types::{SolCall, SolEvent},
};
use async_trait::async_trait;
use common::types::{BlockAndReceipts, Slot};
use contract_bindings::arbitrum::{
    ibridge::IBridge::MessageDelivered,
    idelayedmessageprovider::IDelayedMessageProvider::{
        InboxMessageDelivered, InboxMessageDeliveredFromOrigin,
    },
    irollup::IRollup,
};
use eyre::{Error, Result};
use std::collections::HashMap;
use tracing::debug;

const MSG_DELIVERED_EVENT_HASH: FixedBytes<32> = MessageDelivered::SIGNATURE_HASH;
const INBOX_MSG_DELIVERED_EVENT_HASH: FixedBytes<32> = InboxMessageDelivered::SIGNATURE_HASH;
const INBOX_MSG_DELIVERED_FROM_ORIGIN_EVENT_HASH: FixedBytes<32> =
    InboxMessageDeliveredFromOrigin::SIGNATURE_HASH;

#[derive(Debug)]
/// Builder for constructing Arbitrum blocks from transactions
pub struct ArbitrumBlockBuilder {
    // MChain rollup address
    mchain_rollup_address: Address,

    // Sequencing chain address
    transaction_parser: SequencingTransactionParser,

    // Settlement chain address
    delayed_inbox: Address,

    delayed_message_count: u64,
}

impl Default for ArbitrumBlockBuilder {
    fn default() -> Self {
        Self {
            transaction_parser: SequencingTransactionParser::new(address!(
                "0xEF741D37485126A379Bfa32b6b260d85a0F00380"
            )),

            mchain_rollup_address: address!("0xEF741D37485126A379Bfa32b6b260d85a0F00380"),
            delayed_inbox: address!("0xEF741D37485126A379Bfa32b6b260d85a0F00380"),
            delayed_message_count: 0,
        }
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
    ) -> Result<Vec<TransactionRequest>, Error> {
        let delayed_messages = self.process_delayed_messages(slot.settlement_chain_blocks).await?;

        let mbtxs = self.parse_blocks_to_mbtxs(slot.sequencing_chain_blocks);

        let batch_txn = self.build_batch_txn(mbtxs).await?;

        let mut result: Vec<TransactionRequest> = Vec::new();
        result.extend(delayed_messages);
        result.push(batch_txn);
        Ok(result)
    }
}

impl ArbitrumBlockBuilder {
    /// Creates a new Arbitrum block builder.
    ///
    /// # Arguments
    /// - `sequencing_contract_address`: The address of the sequencing contract to monitor.
    pub fn new(sequencing_contract_address: Address) -> Self {
        Self {
            transaction_parser: SequencingTransactionParser::new(sequencing_contract_address),
            ..Default::default()
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
            .filter(|log| Address::from_slice(log.address.as_slice()) == self.delayed_inbox)
            .filter_map(|log| {
                // Get the first topic (event signature)
                log.topics.first().and_then(|topic| {
                    let topic_bytes = FixedBytes::from_slice(topic.as_slice());

                    if topic_bytes == MSG_DELIVERED_EVENT_HASH {
                        return Some(log);
                    }

                    if topic_bytes == INBOX_MSG_DELIVERED_EVENT_HASH {
                        let message_num = U256::from_be_slice(log.topics[1].as_slice());
                        message_data.insert(message_num, log.data.clone());
                    } else if topic_bytes == INBOX_MSG_DELIVERED_FROM_ORIGIN_EVENT_HASH {
                        let message_num = U256::from_be_slice(log.topics[1].as_slice());

                        // NOTE: This assumes that the blocks and transactions are in order
                        // not sure if this is always the case, but saves us from looping through
                        // the blocks to find the correct transaction by hashe
                        let block_index = (log.block_number - blocks[0].block.number) as usize;
                        let txn_index = log.transaction_index as usize;
                        let txn = blocks[block_index].block.transactions[txn_index].clone();

                        let data = Bytes::from(txn.input);
                        message_data.insert(message_num, data);
                    }

                    None
                })
            })
            .collect::<Vec<_>>();

        debug!("Delayed message data: {:?}", message_data);
        debug!("Delayed messages: {:?}", delayed_messages);

        let mut delayed_msg_txns: Vec<TransactionRequest> = Vec::new();

        for msg_log in delayed_messages {
            let message_index = U256::from_be_slice(msg_log.topics[1].as_slice());

            // Data layout (each field is 32 bytes):
            // [0:32]    - inbox (address, right-padded)
            // [32:64]   - kind (uint8, right-padded)
            // [64:96]   - sender (address, right-padded)
            // [96:128]  - messageDataHash (bytes32)
            // [128:160] - baseFeeL1 (uint256)
            // [160:192] - timestamp (uint64, right-padded)
            let kind = msg_log.data[63] as u8; // Last byte of the second 32-byte word
            let sender = Address::from_slice(&msg_log.data[76..96]); // Last 20 bytes of the third 32-byte word

            // Get corresponding message data
            let data = message_data
                .get(&message_index)
                .ok_or_else(|| Error::msg("Missing inbox message data"))?;

            let delivered_msg_tx =
                TransactionRequest::default().to(self.mchain_rollup_address).input(
                    IRollup::deliverMessageCall { kind, sender, messageData: data.clone() }
                        .abi_encode()
                        .into(),
                );

            delayed_msg_txns.push(delivered_msg_tx);
        }

        Ok(delayed_msg_txns)
    }

    /// Builds a batch of transactions into an Arbitrum batch
    async fn build_batch_txn(&self, txs: Vec<Bytes>) -> Result<TransactionRequest, Error> {
        let batch = if txs.is_empty() {
            Batch(vec![BatchMessage::Delayed])
        } else {
            Batch(vec![BatchMessage::L2(L1IncomingMessage {
                // TODO: Add meaningful values for the header
                header: Default::default(),
                l2_msg: txs,
            })])
        };

        // Encode the batch data
        let encoded_batch = batch.encode()?;

        // Create the transaction request
        let request = TransactionRequest::default().to(self.mchain_rollup_address).input(
            // Encode the function call with parameters
            IRollup::postBatchCall::new((
                U256::from(self.delayed_message_count), // after delayed messages read
                encoded_batch,                          // batch data
            ))
            .abi_encode()
            .into(), // Convert the tokenized call data to bytes
        );

        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{hex, keccak256, TxKind};
    use common::types::{Block, Log, Receipt, SlotState, Transaction};
    use std::str::FromStr;

    #[test]
    fn test_new_builder() {
        let sequencing_contract_address =
            Address::from_str("0x1234000000000000000000000000000000000000")
                .expect("Invalid address format");
        let builder = ArbitrumBlockBuilder::new(sequencing_contract_address);
        let parser = builder.transaction_parser();
        assert!(!std::ptr::eq(parser, std::ptr::null()), "Transaction parser should not be null");
    }

    #[tokio::test]
    async fn test_build_batch_empty_txs() {
        let builder = ArbitrumBlockBuilder::default();
        let txs = vec![];
        let batch = builder.build_batch_txn(txs).await.unwrap();

        // Verify transaction is sent to sequencer inbox
        assert_eq!(batch.to, Some(TxKind::Call(builder.mchain_rollup_address)));

        // For empty batch, should create BatchMessage::Delayed
        let expected_batch = Batch(vec![BatchMessage::Delayed]);
        let expected_encoded = expected_batch.encode().unwrap();

        // Verify the input data contains the correct parameters
        let call_data = IRollup::postBatchCall::new((
            U256::from(0),    // delayed message count
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
        let batch = builder.build_batch_txn(txs.clone()).await.unwrap();

        // Verify transaction is sent to sequencer inbox
        assert_eq!(batch.to, Some(TxKind::Call(builder.mchain_rollup_address)));

        // For non-empty batch, should create BatchMessage::L2
        let expected_batch = Batch(vec![BatchMessage::L2(L1IncomingMessage {
            header: Default::default(),
            l2_msg: txs,
        })]);
        let expected_encoded = expected_batch.encode().unwrap();

        // Verify the input data contains the correct parameters
        let call_data = IRollup::postBatchCall::new((
            U256::from(0),    // delayed message count
            expected_encoded, // batch data
        ))
        .abi_encode();

        assert_eq!(batch.input, call_data.into());
    }

    #[tokio::test]
    async fn test_build_block_from_slot() {
        let builder = ArbitrumBlockBuilder::default();
        let mut builder = builder;

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
        assert_eq!(txns.len(), 1); // Should contain just the batch transaction

        // Verify the batch transaction
        let batch_tx = &txns[0];
        assert_eq!(batch_tx.to, Some(TxKind::Call(builder.mchain_rollup_address)));
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
            inbox: builder.delayed_inbox,
            kind: 0u8,
            sender: Address::ZERO,
            messageDataHash: keccak256(message_data.clone()),
            baseFeeL1: U256::ZERO,
            timestamp: 0u64,
        };

        // Create mock logs
        let msg_delivered_log = create_mock_log(
            builder.delayed_inbox,
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
            builder.delayed_inbox,
            vec![INBOX_MSG_DELIVERED_EVENT_HASH, FixedBytes::from(message_index.to_be_bytes())],
            message_data.clone(),
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
        let tx_data: Bytes = hex!("1234").into();
        let block = Block {
            number: 1,
            transactions: vec![Transaction {
                input: tx_data.clone().to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };

        let slot = Slot {
            slot_number: 1,
            timestamp: 0,
            state: SlotState::Safe,
            settlement_chain_blocks: vec![],
            sequencing_chain_blocks: vec![BlockAndReceipts { block, receipts: vec![] }],
        };

        let result = builder.build_block_from_slot(slot).await;
        assert!(result.is_ok());

        let txns = result.unwrap();
        assert_eq!(txns.len(), 1);

        // Verify the batch transaction contains our tx data
        let batch_tx = &txns[0];
        assert_eq!(batch_tx.to, Some(TxKind::Call(builder.mchain_rollup_address)));
    }

    #[tokio::test]
    async fn test_build_block_from_slot_with_sequencing_and_delayed_txns() {
        let builder = ArbitrumBlockBuilder::default();
        let mut builder = builder;

        // Create a mock L2 transaction
        let tx_data: Bytes = hex!("1234").into();
        let sequencing_block = Block {
            number: 1,
            transactions: vec![Transaction {
                input: tx_data.clone().to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };

        // Create mock delayed message logs
        let message_num = U256::from(1);
        let before_inbox_acc = FixedBytes::from([1u8; 32]);
        let delayed_message_data: Bytes = hex!("5678").into();

        let msg_delivered_event = MessageDelivered {
            messageIndex: message_num,
            beforeInboxAcc: before_inbox_acc,
            inbox: builder.delayed_inbox,
            kind: 2u8,
            sender: Address::repeat_byte(1),
            messageDataHash: keccak256(delayed_message_data.clone()),
            baseFeeL1: U256::ZERO,
            timestamp: 0u64,
        };

        let delayed_log = create_mock_log(
            builder.delayed_inbox,
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
            builder.delayed_inbox,
            vec![INBOX_MSG_DELIVERED_EVENT_HASH, FixedBytes::from(message_num.to_be_bytes::<32>())],
            delayed_message_data.clone(),
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
                receipts: vec![],
            }],
        };

        let result = builder.build_block_from_slot(slot).await;
        assert!(result.is_ok());

        let txns = result.unwrap();
        assert_eq!(txns.len(), 2); // Should contain batch transaction + delayed message transaction

        // Verify delayed message transaction
        let delayed_tx = &txns[0];
        assert_eq!(delayed_tx.to, Some(TxKind::Call(builder.mchain_rollup_address)));
        let expected_delayed_call = IRollup::deliverMessageCall {
            kind: 2u8,
            sender: Address::repeat_byte(1),
            messageData: delayed_message_data,
        }
        .abi_encode()
        .into();
        assert_eq!(delayed_tx.input, expected_delayed_call);

        // Verify the batch transaction contains our sequencing tx data
        let batch_tx = &txns[1];
        assert_eq!(batch_tx.to, Some(TxKind::Call(builder.mchain_rollup_address)));
        let expected_batch = Batch(vec![BatchMessage::L2(L1IncomingMessage {
            header: Default::default(),
            l2_msg: vec![tx_data],
        })]);
        let expected_encoded = expected_batch.encode().unwrap();
        let _expected_batch_call =
            IRollup::postBatchCall::new((U256::from(0), expected_encoded)).abi_encode();
        // assert_eq!(batch_tx.input.data.unwrap(), expected_batch_call);
    }
}
