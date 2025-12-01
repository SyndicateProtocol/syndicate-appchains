//! Arbitrum block builder implementation
//!
//! This module provides functionality for building an Arbitrum batch submitter transaction from a
//! list of transactions. It implements the [`RollupBlockBuilder`] trait to standardize block
//! construction across different rollup implementations

use crate::{
    appchains::{
        arbitrum::batch::{
            Batch, BatchMessage, L1IncomingMessage, L1IncomingMessageHeader, MAX_L2_MESSAGE_SIZE,
        },
        shared::{sequencing_transaction_parser::get_event_transactions, RollupAdapter},
    },
    config::BlockBuilderConfig,
};
use alloy::{
    primitives::{Address, Bytes, Log, TxHash, U256},
    sol_types::SolEvent as _,
};
use common::types::{SequencingBlock, SettlementBlock};
use contract_bindings::synd::i_bridge::IBridge::MessageDelivered;
use eyre::Result;
use shared::types::{BlockBuilder, DelayedMsgsData, PartialBlock};
use std::collections::HashMap;
use synd_mchain::db::DelayedMessage;
use thiserror::Error;
use tracing::{debug, error, trace};

// Limit the number of tx per a block so that they have enough gas to be included most of the time.
// Each tx can use 320k gas on average given the default block gas limit of 32 million.
// Eventually once the translator uses nitro to simulate tx execution, transactions can be slotted
// into blocks based on gas usage instead.
const MAX_TXS_PER_BLOCK: usize = 100;

#[allow(missing_docs)] // self-documenting
#[derive(Debug, Error)]
pub enum ArbitrumBlockBuilderError {
    #[error("Failed to decode {0}: {1}")]
    DecodingError(&'static str, eyre::Error),

    #[error("Missing inbox message data for message index: {0}")]
    MissingInboxMessageData(U256),

    #[error("Delayed message ignored: type = {0}")]
    DelayedMessageIgnored(L1MessageType),

    #[error("Unexpected initialize msg at message index: {0}")]
    UnexpectedInitializeMessage(U256),
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq)]
/// `<https://github.com/OffchainLabs/nitro/blob/c7f3429e2456bf5ca296a49cec3bb437420bc2bb/contracts/src/libraries/MessageTypes.sol>`
pub enum L1MessageType {
    L2Message = 3,
    EndOfBlock = 6, // dummy message that is not emitted by the nitro contracts
    L2FundedByL1 = 7,
    SubmitRetryable = 9,
    Initialize = 11,
    EthDeposit = 12,
    BatchPostingReport = 13,
}

impl TryFrom<u8> for L1MessageType {
    type Error = ();

    // EndOfBlock is deliberately excluded since it is a dummy message that is not emitted by the
    // nitro contracts
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
    /// Convert a u8 to a `L1MessageType`
    pub fn from_u8_panic(value: u8) -> Self {
        Self::try_from(value).unwrap_or_else(|_| panic!("Invalid L1MessageType value: {value}"))
    }
}

impl std::fmt::Display for L1MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone)]
/// Builder for constructing Arbitrum blocks from transactions
pub struct ArbitrumAdapter {
    /// sequencing contract
    pub sequencing_contract_address: Address,

    /// Settlement chain address
    pub bridge_address: Address,

    /// Settlement chain address
    pub inbox_address: Address,
}

impl RollupAdapter for ArbitrumAdapter {
    fn get_event_transactions(&self, eth_log: &Log) -> Result<Vec<(Bytes, TxHash)>> {
        get_event_transactions(eth_log, &self.sequencing_contract_address)
            .map_err(|e| eyre::eyre!("{}", e))
    }

    /// Builds a batch of transactions into an Arbitrum batch
    /// note: this must mirror the logic in the enclave go code
    /// for building batches.
    /// NOTE: this must mirror the logic of the TEE enclave
    #[allow(clippy::cognitive_complexity)]
    fn build_batch_bytes(
        &self,
        txs: Vec<Bytes>,
        l1_block_number: u64,
        mchain_timestamp: u64,
    ) -> Result<Bytes> {
        debug!("Sequenced transactions: {:?}", txs);

        let mut messages = vec![];
        let mut batch_txs = vec![];
        // Start with the batch header byte - see l2_msg_to_bytes in batch.rs for more
        // infomation.
        let mut size = 1;
        for tx in txs {
            // When multiple txs are included in the block, then each tx is prefixed a uint64
            // value indicating with the size of the tx.
            // See l2_msg_to_bytes in batch.rs for more infomation.
            let tx_size = 8 + tx.len();
            size += tx_size;
            if batch_txs.len() >= MAX_TXS_PER_BLOCK ||
                (!batch_txs.is_empty() && size > MAX_L2_MESSAGE_SIZE)
            {
                messages.push(BatchMessage::L2(L1IncomingMessage {
                    header: L1IncomingMessageHeader {
                        block_number: l1_block_number,
                        timestamp: mchain_timestamp,
                    },
                    l2_msg: batch_txs,
                }));
                batch_txs = vec![];
                // When multiple transactions are in the block, then the batch of transactions
                // is prefixed with a batch header byte.
                // See l2_msg_to_bytes in batch.rs for more infomation.
                size = 1 + tx_size;
            }
            batch_txs.push(tx);
        }
        if !batch_txs.is_empty() {
            messages.push(BatchMessage::L2(L1IncomingMessage {
                header: L1IncomingMessageHeader {
                    block_number: l1_block_number,
                    timestamp: mchain_timestamp,
                },
                l2_msg: batch_txs,
            }));
        }

        let batch = Batch(messages);
        debug!("New Batch: {:?}", batch);
        batch.encode()
    }
}

impl ArbitrumAdapter {
    /// Creates a new Arbitrum block builder.
    ///
    /// # Arguments
    /// - `config`: The configuration for the block builder.
    #[allow(clippy::unwrap_used)] //it's okay to unwrap here because we know the config is valid
    pub const fn new(config: &BlockBuilderConfig) -> Self {
        Self {
            sequencing_contract_address: config.sequencing_contract_address.unwrap(),
            bridge_address: config.arbitrum_bridge_address.unwrap(),
            inbox_address: config.arbitrum_inbox_address.unwrap(),
        }
    }

    /// Processes settlement chain receipts into delayed messages
    pub fn process_delayed_messages(
        &self,
        block: &PartialBlock,
        msgs_data: DelayedMsgsData,
    ) -> Result<Vec<DelayedMessage>> {
        // Process all bridge logs in all receipts
        let delayed_messages = block.logs.iter().filter(|log| {
            log.address == self.bridge_address &&
                log.topics()[0] == MessageDelivered::SIGNATURE_HASH
        });

        trace!("Delayed message data: {:?}", msgs_data);
        trace!("Delayed messages: {:?}", delayed_messages);

        let delayed_msg_txns = delayed_messages
            .filter_map(|msg_log| {
                match self.delayed_message_to_mchain_txn(msg_log, &msgs_data) {
                    Ok(txn) => Some(txn),
                    Err(ArbitrumBlockBuilderError::DelayedMessageIgnored(
                        L1MessageType::Initialize,
                    )) => {
                        error!("Ignoring init message: the rollup is already initialized");
                        None
                    }
                    Err(ArbitrumBlockBuilderError::DelayedMessageIgnored(e)) => {
                        // replace ignored messages with a dummy message to burn the nonce
                        error!("Replacing ignored delayed message with an empty block: {}", e);
                        Some(DelayedMessage {
                            kind: L1MessageType::EndOfBlock as u8,
                            sender: Address::ZERO,
                            data: Default::default(),
                            base_fee_l1: U256::ZERO,
                        })
                    }
                    Err(e) => {
                        panic!("Fatal error: {e}")
                    }
                }
            })
            .collect();

        // TODO(SEQ-851): compute & log delayed message tx hashes

        Ok(delayed_msg_txns)
    }

    /// Sequencer log addresses used for building batches
    pub fn sequencer_addresses(&self) -> Vec<Address> {
        vec![self.sequencing_contract_address]
    }

    /// Settlement log addresses used for building delayed messages
    pub fn settlement_addresses(&self) -> Vec<Address> {
        let mut addrs = vec![self.bridge_address, self.inbox_address];
        addrs.dedup();
        addrs
    }

    fn delayed_message_to_mchain_txn(
        &self,
        log: &Log,
        message_data: &HashMap<U256, Bytes>,
    ) -> Result<DelayedMessage, ArbitrumBlockBuilderError> {
        let msg = MessageDelivered::decode_raw_log_validate(log.topics(), &log.data.data)
            .map_err(|e| ArbitrumBlockBuilderError::DecodingError("MessageDelivered", e.into()))?;

        let kind = L1MessageType::from_u8_panic(msg.kind);

        if msg.kind == L1MessageType::Initialize as u8 && msg.messageIndex != U256::ZERO {
            return Err(ArbitrumBlockBuilderError::UnexpectedInitializeMessage(msg.messageIndex))
        }

        if Self::should_ignore_delayed_message(&kind) {
            return Err(ArbitrumBlockBuilderError::DelayedMessageIgnored(kind));
        }

        Ok(DelayedMessage {
            kind: msg.kind,
            sender: msg.sender,
            #[allow(clippy::unwrap_used)]
            data: message_data.get(&msg.messageIndex).unwrap().clone(),
            base_fee_l1: msg.baseFeeL1,
        })
    }

    /// Should ignore delayed message. Ignores `Initialize` & `BatchPostingReport` message types.
    pub fn should_ignore_delayed_message(kind: &L1MessageType) -> bool {
        // Always ignore Initialize & BatchPostingReport message types.
        // Except for the initial initialization message, these should not occur in practice.
        if matches!(kind, L1MessageType::Initialize | L1MessageType::BatchPostingReport) {
            error!("Ignoring unexpected delayed message. Kind: {:?}.", kind);
            return true;
        }

        false
    }
}

impl BlockBuilder<SequencingBlock> for ArbitrumAdapter {
    fn build_block(
        &self,
        block: PartialBlock,
        msgs_data: DelayedMsgsData,
    ) -> Result<SequencingBlock> {
        assert!(
            msgs_data.is_empty(),
            "delayed messages found on sequencing block: {block:?}, {msgs_data:?}"
        );
        Ok(block)
    }
}

impl BlockBuilder<SettlementBlock> for ArbitrumAdapter {
    fn build_block(
        &self,
        block: PartialBlock,
        msgs_data: DelayedMsgsData,
    ) -> Result<SettlementBlock> {
        Ok(SettlementBlock {
            block_ref: block.block_ref.clone(),
            parent_hash: block.parent_hash,
            messages: self.process_delayed_messages(&block, msgs_data)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{hex, keccak256, FixedBytes};
    use assert_matches::assert_matches;
    use std::str::FromStr;

    fn test_config() -> BlockBuilderConfig {
        BlockBuilderConfig {
            sequencing_contract_address: Some(
                Address::from_str("0x1234000000000000000000000000000000000000").unwrap(),
            ),
            arbitrum_bridge_address: Some(
                Address::from_str("0x1234000000000000000000000000000000000000").unwrap(),
            ),
            arbitrum_inbox_address: Some(
                Address::from_str("0x1234000000000000000000000000000000000000").unwrap(),
            ),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_build_batch_empty_txs() {
        let builder = ArbitrumAdapter::new(&test_config());
        let txs = vec![];
        let batch = builder.build_batch_bytes(txs, 0, 0).unwrap();

        // For empty batch, should create BatchMessage::Delayed
        let expected_batch = Batch(vec![]);
        let expected_encoded = expected_batch.encode().unwrap();

        assert_eq!(batch, expected_encoded);
    }

    #[tokio::test]
    async fn test_build_batch_with_txs() {
        let builder = ArbitrumAdapter::new(&test_config());
        let txs = vec![
            hex!("1234").into(), // Sample transaction data
            hex!("5678").into(),
        ];
        let batch = builder.build_batch_bytes(txs.clone(), 0, 0).unwrap();

        // For non-empty batch, should create BatchMessage::L2
        let expected_batch = Batch(vec![BatchMessage::L2(L1IncomingMessage {
            header: L1IncomingMessageHeader { timestamp: 0, block_number: 0 },
            l2_msg: txs,
        })]);
        let expected_encoded = expected_batch.encode().unwrap();

        assert_eq!(batch, expected_encoded);
    }

    #[test]
    fn test_delayed_message_to_mchain_txn_success() {
        let builder = ArbitrumAdapter::new(&test_config());

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
        let log = Log::new_unchecked(
            builder.bridge_address,
            vec![
                MessageDelivered::SIGNATURE_HASH,
                message_index.into(),
                FixedBytes::from([1u8; 32]),
            ],
            msg_delivered.encode_data().into(),
        );

        // Call the function
        let result = builder.delayed_message_to_mchain_txn(&log, &message_map);
        assert!(result.is_ok());

        let txn = result.unwrap();

        // Verify the input data matches expected DelayedMessage
        assert_eq!(
            txn,
            DelayedMessage {
                kind: L1MessageType::L2Message as u8,
                sender: Address::repeat_byte(1),
                data: message_data,
                base_fee_l1: U256::ZERO,
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_delayed_message_to_mchain_txn_missing_data() {
        let builder = ArbitrumAdapter::new(&test_config());

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

        let log = Log::new_unchecked(
            builder.bridge_address,
            vec![
                MessageDelivered::SIGNATURE_HASH,
                message_index.into(),
                FixedBytes::from([1u8; 32]),
            ],
            msg_delivered.encode_data().into(),
        );

        // Empty message data map
        let message_map = HashMap::new();

        // Call should panic
        _ = builder.delayed_message_to_mchain_txn(&log, &message_map);
    }

    #[test]
    fn test_delayed_message_to_mchain_txn_invalid_event_data() {
        let builder = ArbitrumAdapter::new(&test_config());
        let message_map = HashMap::new();

        // Create log with invalid event data
        let log = Log::new_unchecked(
            builder.bridge_address,
            vec![MessageDelivered::SIGNATURE_HASH],
            Bytes::from(vec![1, 2, 3]), // Invalid data that can't be decoded
        );

        // Call should fail with DecodingError
        let result = builder.delayed_message_to_mchain_txn(&log, &message_map);
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), ArbitrumBlockBuilderError::DecodingError(_, _));
    }

    #[test]
    fn test_delayed_message_to_mchain_txn_do_not_ignore_deposit() {
        let builder = ArbitrumAdapter::new(&test_config());

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
        let log = Log::new_unchecked(
            builder.bridge_address,
            vec![
                MessageDelivered::SIGNATURE_HASH,
                message_index.into(),
                FixedBytes::from([1u8; 32]),
            ],
            msg_delivered.encode_data().into(),
        );

        let result = builder.delayed_message_to_mchain_txn(&log, &message_map);
        assert!(result.is_ok());

        let txn = result.unwrap();

        // Verify the input data matches expected DelayedMessage
        assert_eq!(
            txn,
            DelayedMessage {
                kind: L1MessageType::EthDeposit as u8,
                sender: Address::repeat_byte(1),
                data: message_data,
                base_fee_l1: U256::ZERO,
            }
        );
    }

    #[test]
    fn test_should_ignore_delayed_message() {
        // Should ignore
        assert!(ArbitrumAdapter::should_ignore_delayed_message(&L1MessageType::Initialize));
        assert!(ArbitrumAdapter::should_ignore_delayed_message(&L1MessageType::BatchPostingReport));

        // Should not ignore
        assert!(!ArbitrumAdapter::should_ignore_delayed_message(&L1MessageType::EthDeposit));
        assert!(!ArbitrumAdapter::should_ignore_delayed_message(&L1MessageType::L2Message));
        assert!(!ArbitrumAdapter::should_ignore_delayed_message(&L1MessageType::L2FundedByL1));
        assert!(!ArbitrumAdapter::should_ignore_delayed_message(&L1MessageType::SubmitRetryable));
        assert!(!ArbitrumAdapter::should_ignore_delayed_message(&L1MessageType::EthDeposit));

        assert!(ArbitrumAdapter::should_ignore_delayed_message(&L1MessageType::Initialize));
        assert!(ArbitrumAdapter::should_ignore_delayed_message(&L1MessageType::BatchPostingReport));
    }
}
