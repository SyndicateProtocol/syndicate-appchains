use alloy::dyn_abi::{DynSolEvent, DynSolType, DynSolValue};
use alloy::primitives::{keccak256, Address, Bytes, B256};
use alloy_primitives::LogData;
use common::types::Log;
use eyre::{eyre, Error};
use lazy_static::lazy_static;
use rlp::Rlp;

/// TransactionProcessed event data
#[derive(Debug, Clone)]
pub struct TransactionProcessed {
    /// The encoded data of the transaction
    pub encoded_data: Bytes,
    /// The sender of the transaction
    pub sender: Address,
}

/// The ABI for the TransactionProcessed event
// const TRANSACTION_PROCESSED_ABI: &str = r#"[{"anonymous":false,"inputs":[{"indexed":true,"name":"Sender","type":"address"},{"indexed":false,"name":"EncodedData","type":"bytes"}],"name":"TransactionProcessed","type":"event"}]"#;

/// The signature for the TransactionProcessed event
const TRANSACTION_PROCESSED_SIG: &str = "TransactionProcessed(address,bytes)";

lazy_static! {
    static ref TRANSACTION_PROCESSED_SIG_HASH: B256 =
        keccak256(TRANSACTION_PROCESSED_SIG.as_bytes());
}

/// The parser for meta-based transactions
#[derive(Debug)]
pub struct MBTransactionParser {
    /// The ABI for the sequencing contract
    // sequencing_contract_abi: DynSolType,
    /// The address of the sequencing contract
    sequencing_contract_address: Address,
}

impl MBTransactionParser {
    /// Creates a new MBTransactionParser
    pub fn new(sequencing_contract_address: Address) -> Result<Self, Error> {
        Ok(Self {
            // sequencing_contract_abi,
            sequencing_contract_address,
        })
    }

    /// Checks if a log is a TransactionProcessed event
    pub fn is_log_transaction_processed(&self, eth_log: Log) -> bool {
        eth_log.address == self.sequencing_contract_address
            && eth_log
                .topics
                .first()
                .map_or(false, |t| *t == *TRANSACTION_PROCESSED_SIG_HASH)
    }

    /// Decodes the event data into a vector of transactions
    pub fn decode_event_data(&self, data: Bytes) -> Result<Vec<Bytes>, Error> {
        if data.is_empty() {
            return Err(eyre!("No data provided for decoding"));
        }

        let rlp = Rlp::new(&data);

        let mut transactions = Vec::new();
        for item in rlp.iter() {
            match item.data() {
                Ok(transaction) => transactions.push(Bytes::from(transaction.to_vec())),
                Err(_) => return Err(eyre!("RLP decoding failed for a transaction")),
            }
        }

        Ok(transactions)
    }
    /// Decodes the event data into a vector of transactions
    pub fn get_event_transactions(&self, eth_log: &Log) -> Result<Vec<Bytes>, Error> {
        println!("Log data: {:?}", eth_log.data);
        println!("Log topics: {:?}", eth_log.topics);
        if !self.is_log_transaction_processed(eth_log.clone()) {
            return Err(eyre!("Log is not a TransactionProcessed event"));
        }

        let indexed = vec![DynSolType::Address]; // "Sender" is indexed
        let body = DynSolType::Tuple(vec![DynSolType::Bytes]); // "EncodedData" is non-indexed
        let event = DynSolEvent::new(Some(*TRANSACTION_PROCESSED_SIG_HASH), indexed, body)
            .expect("Failed to construct DynSolEvent");
        println!("Event: {:?}", event);
        let log_data = LogData::new_unchecked(eth_log.topics.clone(), eth_log.data.clone());
        println!(
            "Preparing to decode log: topics = {:?}, data = {:?}",
            log_data.topics(),
            log_data.data
        );

        let decoded_event = event.decode_log_data(&log_data, true)?;
        println!("Decoded event: {:?}", decoded_event);

        // Ensure the decoded event has the expected structure
        if decoded_event.body.len() != 1 {
            return Err(eyre!("Unexpected decoded event structure"));
        }
        println!("Decoded event: {:?}", decoded_event);

        // Extract the transactions from the decoded event body
        let encoded_data = match &decoded_event.body[0] {
            DynSolValue::Bytes(data) => data.clone(),
            _ => return Err(eyre!("Unexpected type for encoded_data")),
        };

        // Decode the transactions
        let transactions = self.decode_event_data(encoded_data.into())?;
        Ok(transactions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn generate_valid_test_log(contract_address: Address) -> Log {
        // Example data for the test log
        let block_hash = B256::from_slice(
            &hex::decode("abc123abc123abc123abc123abc123abc123abc123abc123abc123abc123abc1")
                .unwrap(),
        );
        let transaction_hash = B256::from_slice(
            &hex::decode("def456def456def456def456def456def456def456def456def456def456def4")
                .unwrap(),
        );
        let transaction_index = 1;
        let block_number = 100;
        let log_index = 0;
        let removed = false;
        let address = contract_address;
        let data = Bytes::from(hex::decode("c68401020304").unwrap());

        let topics = vec![
            keccak256("TransactionProcessed(address,bytes)".as_bytes()), // Event signature hash
            B256::from_slice(
                &hex::decode("eef456def456def456def456def456def456def456def456def456def456def4")
                    .unwrap(),
            ), // Example indexed topic
        ];

        Log {
            block_hash_test: block_hash,
            transaction_hash,
            transaction_index,
            block_number,
            log_index,
            removed,
            address,
            data,
            topics,
        }
    }

    #[tokio::test]
    async fn test_is_log_transaction_processed() {
        let contract_address: Address = "0x000000000000000000000000000000000000abcd"
            .parse()
            .unwrap();
        let parser: MBTransactionParser = MBTransactionParser::new(contract_address).unwrap();

        let log = generate_valid_test_log(contract_address);

        assert!(parser.is_log_transaction_processed(log));

        let unrelated_contract_address: Address = "0x110000000000000000000000000000000000abcd"
            .parse()
            .unwrap();
        let unrelated_log = generate_valid_test_log(unrelated_contract_address);

        assert!(!parser.is_log_transaction_processed(unrelated_log));
    }

    // #[tokio::test]
    // async fn test_get_event_transactions() {
    //     // Define the contract address for the test
    //     let contract_address: Address = "0x000000000000000000000000000000000000abcd"
    //         .parse()
    //         .unwrap();

    //     // Initialize the parser
    //     let parser = MBTransactionParser::new(contract_address).unwrap();

    //     // Generate a valid test log for the TransactionProcessed event
    //     let log = generate_valid_test_log(contract_address);

    //     // Call the method to get transactions
    //     let transactions_result = parser.get_event_transactions(&log);

    //     // Assert that the result is Ok and contains the expected transactions
    //     match transactions_result {
    //         Ok(transactions) => {
    //             // Check the number of transactions decoded
    //             assert_eq!(transactions.len(), 1);

    //             // Example of validating the decoded transaction data
    //             let expected_transaction_data = Bytes::from(
    //                 hex::decode("000000000000000000000000000000000000000000000000000000000000000f")
    //                     .unwrap(),
    //             );
    //             assert_eq!(transactions[0], expected_transaction_data);
    //         }
    //         Err(err) => panic!("Failed to get transactions: {}", err),
    //     }
    // }
}
