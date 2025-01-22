//! Shared traits and types for rollup-specific block builders.
//!
//! This module provides the core [`SequencingTransactionParser`] trait that defines how
//! sequencing transactions are captured and parsed.

use alloy::{
    dyn_abi::{DynSolEvent, DynSolType, DynSolValue},
    primitives::{keccak256, Address, Bytes, LogData, B256},
};
use common::{
    compression::{get_compression_type, CompressionType},
    types::Log,
};
use eyre::{eyre, Error};

/// `TransactionProcessed` event data
#[derive(Debug, Clone)]
pub struct TransactionProcessed {
    /// The encoded data of the transaction
    pub encoded_data: Bytes,
    /// The sender of the transaction
    pub sender: Address,
}

/// The parser for meta-based transactions
#[derive(Debug)]
pub struct SequencingTransactionParser {
    /// The ABI for the sequencing contract
    event_signature_hash: B256,
    /// The address of the sequencing contract
    sequencing_contract_address: Address,
}

const EVENT_SIGNATURE: &str = "TransactionProcessed(address,bytes)";

impl SequencingTransactionParser {
    /// Creates a new `SequencingTransactionParser`
    pub fn new(sequencing_contract_address: Address) -> Result<Self, Error> {
        // The signature for the TransactionProcessed event
        // "TransactionProcessed(address,bytes)";
        Ok(Self {
            event_signature_hash: keccak256(EVENT_SIGNATURE.as_bytes()),
            sequencing_contract_address,
        })
    }

    /// Checks if a log is a `TransactionProcessed` event
    pub fn is_log_transaction_processed(&self, eth_log: Log) -> bool {
        eth_log.address == self.sequencing_contract_address
            && eth_log
                .topics
                .first()
                .is_some_and(|t| *t == *self.event_signature_hash)
    }

    /// Decodes the event data into a vector of transactions
    pub fn decode_event_data(&self, data: Bytes) -> Result<Vec<Bytes>, Error> {
        if data.is_empty() {
            return Err(eyre!("No data provided for decoding"));
        }

        let compression_byte = &data[0];
        let compressed_data = &data[1..];
        let compression_type = get_compression_type(*compression_byte);

        let mut transactions = Vec::new();
        match compression_type {
            CompressionType::None => {
                transactions.push(Bytes::copy_from_slice(compressed_data));
            }
            CompressionType::Unknown => {
                return Err(eyre!("Unknown compression type: {:?}", compression_byte));
            }
        }
        Ok(transactions)
    }
    /// Decodes the event data into a vector of transactions
    pub fn get_event_transactions(&self, eth_log: &Log) -> Result<Vec<Bytes>, Error> {
        if !self.is_log_transaction_processed(eth_log.clone()) {
            return Err(eyre!("Log is not a TransactionProcessed event"));
        }

        let indexed = vec![DynSolType::Address]; // "Sender" is indexed
        let body = DynSolType::Tuple(vec![DynSolType::Bytes]); // "EncodedData" is non-indexed
        let event = DynSolEvent::new(Some(self.event_signature_hash), indexed, body)
            .ok_or_else(|| eyre!("Failed to construct DynSolEvent"))?;
        let log_data = LogData::new_unchecked(eth_log.topics.clone(), eth_log.data.clone());
        let decoded_event = event.decode_log_data(&log_data, true)?;

        // Ensure the decoded event has the expected structure
        if decoded_event.body.len() != 1 {
            return Err(eyre!("Unexpected decoded event structure"));
        }

        // Extract the transactions from the decoded event body
        let data = match &decoded_event.body[0] {
            DynSolValue::Bytes(d) => d.clone(),
            _ => return Err(eyre!("Unexpected type for data")),
        };

        // Decode the transactions
        let transactions = self.decode_event_data(data.into())?;
        Ok(transactions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::hex;

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

        Log {
            block_hash_test: B256::from([0u8; 32]),
            transaction_hash: B256::from([0u8; 32]),
            transaction_index: 1,
            block_number: 100,
            log_index: 0,
            removed: false,
            address: contract_address,
            data: Bytes::from(DUMMY_ENCODED_DATA),
            topics,
        }
    }
    #[tokio::test]
    async fn test_new_parser() {
        let contract_address: Address = "0x000000000000000000000000000000000000abcd"
            .parse()
            .unwrap();

        let parser = SequencingTransactionParser::new(contract_address);

        assert!(parser.is_ok());
        let parser = parser.unwrap();
        assert_eq!(parser.sequencing_contract_address, contract_address);
    }

    #[tokio::test]
    async fn test_is_log_transaction_processed() {
        let contract_address: Address = "0x000000000000000000000000000000000000abcd"
            .parse()
            .unwrap();
        let parser: SequencingTransactionParser =
            SequencingTransactionParser::new(contract_address).unwrap();

        let log = generate_valid_test_log(contract_address);

        assert!(parser.is_log_transaction_processed(log));

        let unrelated_contract_address: Address = "0x110000000000000000000000000000000000abcd"
            .parse()
            .unwrap();
        let unrelated_log = generate_valid_test_log(unrelated_contract_address);

        assert!(!parser.is_log_transaction_processed(unrelated_log));
    }

    #[tokio::test]
    async fn test_decode_event_data() {
        let uncompressed_data = b"mock_data".to_vec();
        let input = Bytes::from([vec![0x0], uncompressed_data.clone()].concat());

        let parser = SequencingTransactionParser::new(
            "0x000000000000000000000000000000000000abcd"
                .parse()
                .unwrap(),
        )
        .unwrap();

        let result = parser.decode_event_data(input);

        println!("Decoded result: {:?}", result);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![Bytes::from(uncompressed_data)])
    }

    #[tokio::test]
    async fn test_get_event_transactions_valid_log() {
        let contract_address: Address = "0x000000000000000000000000000000000000abcd"
            .parse()
            .unwrap();
        let parser = SequencingTransactionParser::new(contract_address).unwrap();
        let log = generate_valid_test_log(contract_address);
        let result = parser.get_event_transactions(&log);
        assert!(result.is_ok());
        let transactions = result.unwrap();
        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0], Bytes::from(DUMMY_TXN_VALUE));
    }

    #[tokio::test]
    async fn test_get_event_transactions_invalid_log() {
        let contract_address: Address = "0x000000000000000000000000000000000000abcd"
            .parse()
            .unwrap();
        let parser = SequencingTransactionParser::new(contract_address).unwrap();

        let unrelated_contract_address: Address = "0x110000000000000000000000000000000000abcd"
            .parse()
            .unwrap();
        let log = generate_valid_test_log(unrelated_contract_address);

        let result = parser.get_event_transactions(&log);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Log is not a TransactionProcessed event"
        );
    }
}
