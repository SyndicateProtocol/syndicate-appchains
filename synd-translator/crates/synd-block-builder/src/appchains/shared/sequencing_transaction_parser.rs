//! Shared traits and types for rollup-specific block builders.
//!
//! This module provides the core [`SequencingTransactionParser`] trait that defines how
//! sequencing transactions are captured and parsed.

use alloy::{
    primitives::{keccak256, Address, Bytes, Log},
    sol_types::SolEvent,
};
use contract_bindings::synd::syndicate_sequencing_chain::SyndicateSequencingChain::TransactionProcessed;
use rlp::Rlp;
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

enum TransactionType {
    Unsigned = 0,
    Contract = 1,
    Compressed = 3,
    Signed = 4,
}

impl TryFrom<u8> for TransactionType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unsigned),
            1 => Ok(Self::Contract),
            3 => Ok(Self::Compressed),
            4 => Ok(Self::Signed),
            _ => Err(()),
        }
    }
}

/// Decompresses `zlib`-compressed Ethereum transactions back into their original form
fn decompress_transactions(data: &[u8]) -> Result<Vec<Bytes>, SequencingParserError> {
    // Check for empty data first
    if data.is_empty() {
        return Err(SequencingParserError::DecompressionError("Empty compressed data".to_string()));
    }

    let mut decoded_bytes = Vec::new();
    brotli::BrotliDecompress(&mut &data[..], &mut decoded_bytes)
        .map_err(|e| SequencingParserError::DecompressionError(e.to_string()))?;

    // Decode RLP
    Rlp::new(&decoded_bytes)
        .as_list::<Vec<u8>>()
        .map(|vec_list| {
            vec_list
                .into_iter()
                .map(|mut tx| {
                    tx.insert(0, TransactionType::Signed as u8);
                    Bytes::from(tx)
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
            TransactionType::Compressed => {
                transactions.append(&mut decompress_transactions(&data[1..])?)
            }
            _ => transactions.push(data.clone()),
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
    use rlp::RlpStream;

    const DUMMY_TXN_VALUE: &[u8] = &[TransactionType::Signed as u8];

    /// RLP encodes and compresses a slice of Ethereum transactions using brotli compression
    fn compress_transactions(transactions: &[Bytes]) -> std::io::Result<Bytes> {
        // RLP encode the list of transactions
        let mut stream = RlpStream::new_list(transactions.len());
        for tx in transactions {
            stream.append(&tx.as_ref());
        }
        let encoded = stream.out();

        // Compress the RLP encoded bytes using brotli
        let mut compressed = Vec::new();
        brotli::enc::BrotliCompress(
            &mut &encoded[..],
            &mut compressed,
            &brotli::enc::BrotliEncoderInitParams(),
        )?;

        // Append the compression prefix
        compressed.insert(0, TransactionType::Compressed as u8);

        Ok(Bytes::from(compressed))
    }

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

    #[test]
    fn test_decode_event_data() {
        // Test that demonstrates the correct protocol data structure for each compression case

        // Case 1: No compression, so there's a leading signed byte
        let mut raw_data = b"raw_transaction_data".to_vec();
        raw_data.insert(0, TransactionType::Signed as u8);
        let raw_data = raw_data.into();

        let result = SequencingTransactionParser::decode_event_data(&raw_data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![raw_data]);

        // Case 2: Brotli compression, the uncompressed tx includes a leading signed byte
        let tx = Bytes::from(b"test_transaction".to_vec());
        let transactions = vec![tx.clone()];
        let compressed_data = compress_transactions(&transactions).unwrap();
        let mut signed_tx = Vec::from(tx);
        signed_tx.insert(0, TransactionType::Signed as u8);

        let result = SequencingTransactionParser::decode_event_data(&compressed_data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![signed_tx]);
    }
}
