//! Shared traits and types for rollup-specific block builders.
//!
//! This module provides the core [`SequencingTransactionParser`] trait that defines how
//! sequencing transactions are captured and parsed.

use crate::appchains::arbitrum::batch::MAX_L2_MESSAGE_SIZE;
use alloy::{
    consensus::{transaction::RlpEcdsaDecodableTx as _, TxEip1559, TxEip2930, TxEip7702, TxLegacy},
    primitives::{keccak256, Address, Bytes, Log},
    sol_types::SolEvent,
};
use contract_bindings::synd::syndicate_sequencing_chain::SyndicateSequencingChain::TransactionProcessed;
use rlp::Rlp;
use std::io::Cursor;
use thiserror::Error;
use tracing::{debug, error};

/// Represents errors that can occur during sequencing transaction parsing.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SequencingParserError {
    /// An error occurred while constructing the `DynSolEvent` object.
    #[error("Failed to construct DynSolEvent")]
    DynSolEventCreation,

    /// The decoded event structure does not match the expected format.
    #[error("Unexpected decoded event structure")]
    UnexpectedDecodedEventStructure,

    /// The decoded event data contains an unexpected type.
    #[error("Unexpected type for data")]
    UnexpectedDataType,

    /// The log does not correspond to a `TransactionProcessed` event.
    #[error("Log is not a TransactionProcessed event")]
    InvalidLogEvent,

    /// The compression type in the provided data is unknown.
    #[error("Unknown compression type: {0:?}")]
    UnknownCompressionType(u8),

    /// No data was provided for decoding.
    #[error("No data provided for decoding")]
    NoDataProvided,

    /// An error occurred while decompressing the transaction.
    #[error("Failed to decompress transaction: {0:?}")]
    DecompressionError(String),
}

/// The parser for appchain transactions
#[derive(Debug, Clone)]
pub struct SequencingTransactionParser {
    /// The address of the sequencing contract
    pub sequencing_contract_address: Address,
}

/// See `arbos/parse_l2.go` for details.
#[derive(Debug)]
#[allow(missing_docs)]
pub enum L2MessageKind {
    UnsignedUserTx = 0,
    ContractTx = 1,
    Batch = 3,
    SignedTx = 4,
}

impl TryFrom<u8> for L2MessageKind {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::UnsignedUserTx),
            1 => Ok(Self::ContractTx),
            3 => Ok(Self::Batch),
            4 => Ok(Self::SignedTx),
            _ => Err(()),
        }
    }
}

fn is_signed_tx_valid(mut b: &[u8]) -> bool {
    if b.is_empty() {
        debug!("dropping empty signed tx");
        return false;
    }

    if let Some(err) = match b[0] {
        0x80.. => TxLegacy::eip2718_decode(&mut b).err(),
        TxEip2930::DEFAULT_TX_TYPE => TxEip2930::eip2718_decode(&mut b).err(), // AccessListTxType
        TxEip1559::DEFAULT_TX_TYPE => TxEip1559::eip2718_decode(&mut b).err(), // DynamicFeeTxType
        TxEip7702::DEFAULT_TX_TYPE => TxEip7702::eip2718_decode(&mut b).err(), // SetCodeTxType
        // Arbitrum doesn't support EIP-4844 blob txs yet
        x => Some(alloy::eips::eip2718::Eip2718Error::UnexpectedType(x)),
    } {
        debug!("dropping tx with invalid rlp encoding: {}", err);
        return false;
    }

    if !b.is_empty() {
        debug!("dropping tx with extra bytes");
        return false;
    }

    true
}

/// Decompresses `brotli`-compressed Ethereum transactions back into their original form
fn decompress_transactions(data: &[u8]) -> Result<Vec<Bytes>, SequencingParserError> {
    // Check for empty data first
    if data.is_empty() {
        return Err(SequencingParserError::DecompressionError("Empty compressed data".to_string()));
    }

    let mut buffer = vec![0u8; MAX_L2_MESSAGE_SIZE];
    let mut decoded_bytes = Cursor::new(buffer.as_mut_slice());
    brotli::BrotliDecompress(&mut &data[..], &mut decoded_bytes)
        .map_err(|e| SequencingParserError::DecompressionError(e.to_string()))?;

    if decoded_bytes.position() as usize > data.len() * 10 {
        return Err(SequencingParserError::DecompressionError(
            "Compression ratio above 10".to_string(),
        ));
    }

    // Decode RLP
    Rlp::new(&decoded_bytes.get_ref()[..decoded_bytes.position() as usize])
        .as_list::<Vec<u8>>()
        .map(|vec_list| {
            vec_list
                .into_iter()
                .filter_map(|mut tx| {
                    (tx.len() < MAX_L2_MESSAGE_SIZE && is_signed_tx_valid(&tx)).then(|| {
                        tx.insert(0, L2MessageKind::SignedTx as u8);
                        Bytes::from(tx)
                    })
                })
                .collect()
        })
        .map_err(|e| SequencingParserError::DecompressionError(e.to_string()))
}

impl SequencingTransactionParser {
    /// Creates a new `SequencingTransactionParser`
    pub const fn new(sequencing_contract_address: Address) -> Self {
        Self { sequencing_contract_address }
    }

    /// Checks if a log is a `TransactionProcessed` event
    pub fn is_log_transaction_processed(&self, eth_log: &Log) -> bool {
        eth_log.address == self.sequencing_contract_address &&
            eth_log
                .topics()
                .first()
                .is_some_and(|t| *t == keccak256(TransactionProcessed::SIGNATURE.as_bytes()))
    }

    /// Decodes the event data into a vector of typed transactions
    pub fn decode_event_data(data: &Bytes) -> Result<Vec<Bytes>, SequencingParserError> {
        if data.is_empty() {
            return Err(SequencingParserError::NoDataProvided);
        }

        let mut transactions = Vec::new();
        match data[0].try_into().map_err(|_| SequencingParserError::UnexpectedDataType)? {
            L2MessageKind::Batch => transactions.append(&mut decompress_transactions(&data[1..])?),
            L2MessageKind::SignedTx => {
                if data.len() <= MAX_L2_MESSAGE_SIZE && is_signed_tx_valid(&data[1..]) {
                    transactions.push(data.clone());
                }
            }
            // The sequencing contract ensures that unsigned transactions are valid
            L2MessageKind::UnsignedUserTx | L2MessageKind::ContractTx => {
                if data.len() <= MAX_L2_MESSAGE_SIZE {
                    transactions.push(data.clone());
                }
            }
        }

        Ok(transactions)
    }

    /// Decodes the event data into a vector of transactions
    pub fn get_event_transactions(
        &self,
        eth_log: &Log,
    ) -> Result<Vec<Bytes>, SequencingParserError> {
        if !self.is_log_transaction_processed(eth_log) {
            return Err(SequencingParserError::InvalidLogEvent);
        }
        let decoded_event = TransactionProcessed::decode_log_data_validate(&eth_log.data)
            .map_err(|_e| SequencingParserError::DynSolEventCreation)?;

        // Decode the transactions
        Self::decode_event_data(&decoded_event.data).map_err(|e| {
            error!("Error decoding event data: {:?}", e);
            e
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{hex, primitives::B256, sol_types::SolValue};

    const DUMMY_TXN_VALUE: &[u8] = &[L2MessageKind::UnsignedUserTx as u8];

    fn generate_valid_test_log(contract_address: Address) -> Log {
        let topics = vec![
            keccak256("TransactionProcessed(address,bytes)".as_bytes()),
            B256::from_slice(
                &hex::decode("eef456def456def456def456def456def456def456def456def456def456def4")
                    .unwrap(),
            ),
        ];

        Log::new_unchecked(contract_address, topics, Bytes::from(DUMMY_TXN_VALUE.abi_encode()))
    }

    #[tokio::test]
    async fn test_new_parser() {
        let contract_address: Address =
            "0x000000000000000000000000000000000000abcd".parse().unwrap();

        let parser = SequencingTransactionParser::new(contract_address);
        assert_eq!(parser.sequencing_contract_address, contract_address);
    }

    #[tokio::test]
    async fn test_is_log_transaction_processed() {
        let contract_address: Address =
            "0x000000000000000000000000000000000000abcd".parse().unwrap();
        let parser: SequencingTransactionParser =
            SequencingTransactionParser::new(contract_address);

        let log = generate_valid_test_log(contract_address);

        assert!(parser.is_log_transaction_processed(&log));

        let unrelated_contract_address: Address =
            "0x110000000000000000000000000000000000abcd".parse().unwrap();
        let unrelated_log = generate_valid_test_log(unrelated_contract_address);

        assert!(!parser.is_log_transaction_processed(&unrelated_log));
    }

    #[tokio::test]
    async fn test_get_event_transactions_valid_log() {
        let contract_address: Address =
            "0x000000000000000000000000000000000000abcd".parse().unwrap();
        let parser = SequencingTransactionParser::new(contract_address);
        let log = generate_valid_test_log(contract_address);
        let result = parser.get_event_transactions(&log);
        assert!(result.is_ok());
        let transactions = result.unwrap();
        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0], Bytes::from(DUMMY_TXN_VALUE));
    }

    #[tokio::test]
    async fn test_get_event_transactions_invalid_log() {
        let contract_address: Address =
            "0x000000000000000000000000000000000000abcd".parse().unwrap();
        let parser = SequencingTransactionParser::new(contract_address);

        let unrelated_contract_address: Address =
            "0x110000000000000000000000000000000000abcd".parse().unwrap();
        let log = generate_valid_test_log(unrelated_contract_address);

        let result = parser.get_event_transactions(&log);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SequencingParserError::InvalidLogEvent);
    }
}
