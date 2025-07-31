//! Shared traits and types for rollup-specific block builders.
//!
//! This module provides the core [`SequencingTransactionParser`] trait that defines how
//! sequencing transactions are captured and parsed.

use alloy::{
    primitives::{keccak256, Address, Bytes, Log},
    sol_types::SolEvent,
};
use common::compression::{get_compression_type, CompressionType};
use contract_bindings::synd::syndicate_sequencing_chain::SyndicateSequencingChain::TransactionProcessed;
use shared::zlib_compression::decompress_transactions;
use thiserror::Error;
use tracing::error;

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

    /// Decodes the event data into a vector of transactions
    pub fn decode_event_data(data: &Bytes) -> Result<Vec<Bytes>, SequencingParserError> {
        if data.is_empty() {
            return Err(SequencingParserError::NoDataProvided);
        }

        let compression_byte = &data[0];
        let compression_type = get_compression_type(*compression_byte);

        let mut transactions = Vec::new();
        match compression_type {
            CompressionType::None => {
                // Excluding the leading compression byte
                let uncompressed_data = Bytes::copy_from_slice(&data[1..]);
                transactions.push(uncompressed_data);
            }
            CompressionType::Zlib => {
                let mut decompressed_data = decompress_transactions(data)
                    .map_err(|e| SequencingParserError::DecompressionError(e.to_string()))?;
                transactions.append(&mut decompressed_data);
            }
            CompressionType::Unknown => {
                return Err(SequencingParserError::UnknownCompressionType(*compression_byte));
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
    use alloy::{hex, primitives::B256};
    use shared::zlib_compression::{compress_transactions, is_valid_zlib_cm_bits};

    const DUMMY_ENCODED_DATA: &[u8] = &hex!(
        "000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000020002000000000000000000000000000000000000000000000000000000000000"
    );
    const DUMMY_TXN_VALUE: &[u8] = &hex!("02");

    fn generate_valid_test_log(contract_address: Address) -> Log {
        let topics = vec![
            keccak256("TransactionProcessed(address,bytes)".as_bytes()),
            B256::from_slice(
                &hex::decode("eef456def456def456def456def456def456def456def456def456def456def4")
                    .unwrap(),
            ),
        ];

        Log::new_unchecked(contract_address, topics, Bytes::from(DUMMY_ENCODED_DATA))
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

    #[tokio::test]
    async fn test_decode_event_data() {
        let uncompressed_data = b"mock_data".to_vec();
        let input = Bytes::from([vec![0x0], uncompressed_data.clone()].concat());

        let result = SequencingTransactionParser::decode_event_data(&input);

        println!("Decoded result: {result:?}");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![Bytes::from(uncompressed_data)])
    }

    #[test]
    fn test_protocol_data_structure() {
        // Test that demonstrates the correct protocol data structure for each compression case

        // Case 1: No compression, so there's a leading compression byte
        let raw_data = b"raw_transaction_data".to_vec();
        let no_compression_input = Bytes::from([vec![0x00], raw_data.clone()].concat());

        let result = SequencingTransactionParser::decode_event_data(&no_compression_input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap()[0], Bytes::from(raw_data));

        // Case 2: Zlib compression - data starts directly with zlib header
        let tx = Bytes::from(b"test_transaction".to_vec());
        let transactions = vec![tx.clone()];
        let compressed_data = compress_transactions(&transactions).unwrap();

        // Verify the compressed data starts with valid zlib header
        assert!(is_valid_zlib_cm_bits(compressed_data[0]), "Should start with valid zlib CM=8");

        // Pass compressed data directly - no protocol prefix needed
        let result = SequencingTransactionParser::decode_event_data(&compressed_data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap()[0], tx);

        println!("Protocol structure verified:");
        println!("- No compression: [0x00][raw_data]");
        println!("- Zlib compression: [zlib_header (e.g., 0x78)][zlib_payload]");
    }
}
