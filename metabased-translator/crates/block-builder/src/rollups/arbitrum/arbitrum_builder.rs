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
use tracing::info;

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

        let mut result: Vec<TransactionRequest> = vec![batch_txn];
        result.extend(delayed_messages);
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

        info!("Delayed message data: {:?}", message_data);
        info!("Delayed messages: {:?}", delayed_messages);

        let mut delayed_msg_txns: Vec<TransactionRequest> = Vec::new();

        for msg_log in delayed_messages {
            let message_index = U256::from_be_slice(msg_log.topics[1].as_slice());
            let kind = msg_log.data[31] as u8; // kind is a uint8, extract from the last byte of the first 32-byte chunk
            let sender = Address::from_slice(&msg_log.data[32..52]); // sender is a 20-byte address starting at offset 32

            // Get corresponding message data
            let data = message_data
                .get(&message_index)
                .ok_or_else(|| Error::msg("Missing inbox message data"))?;

            // IRollup
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
        let request = TransactionRequest::default()
            .to(self.mchain_rollup_address) // Sequencer Inbox address
            .input(
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
    use alloy::primitives::{hex, TxKind};
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
}
