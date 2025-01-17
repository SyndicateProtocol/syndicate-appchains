use alloy::primitives::{keccak256, Address, Bytes, B256};
use common::types::Log;
use eyre::{eyre, Error};
use lazy_static::lazy_static;
use rlp::Rlp;
use serde_json;

/// TransactionProcessed event data
#[derive(Debug, Clone)]
pub struct TransactionProcessed {
    /// The encoded data of the transaction
    pub encoded_data: Bytes,
    /// The sender of the transaction
    pub sender: Address,
}

/// The ABI for the TransactionProcessed event
const TRANSACTION_PROCESSED_ABI: &str = r#"[
    {
        "anonymous": false,
        "inputs": [
            {"indexed": true, "name": "Sender", "type": "address"},
            {"indexed": false, "name": "EncodedData", "type": "bytes"}
        ],
        "name": "TransactionProcessed",
        "type": "event"
    }
]"#;

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
    sequencing_contract_abi: String,
    /// The address of the sequencing contract
    sequencing_contract_address: Address,
}

impl MBTransactionParser {
    /// Creates a new MBTransactionParser
    pub fn new(abi: String, sequencing_contract_address: Address) -> Result<Self, Error> {
        let sequencing_contract_abi =
            serde_json::from_str(&abi).map_err(|err| eyre!("Failed to parse ABI: {:?}", err))?;
        Ok(Self {
            sequencing_contract_abi,
            sequencing_contract_address,
        })
    }

    /// Checks if a log is a TransactionProcessed event
    pub fn is_log_transaction_processed(&self, eth_log: Log) -> bool {
        eth_log.address == self.sequencing_contract_address
            && eth_log
                .topics
                .get(0)
                .map(|t| {
                    let topic_hash: B256 = B256::from_slice(t.as_bytes());
                    topic_hash == *TRANSACTION_PROCESSED_SIG_HASH
                })
                .unwrap_or(false)
    }

    /// Decodes the event data into a vector of transactions
    pub fn decode_event_data(&self, data: Bytes) -> Result<Vec<Bytes>, Error> {
        if data.is_empty() {
            return Err(eyre!("No data provided for decoding"));
        }

        let rlp = Rlp::new(&data);
        let mut transactions = Vec::new();

        for item in rlp.iter() {
            let transaction: Vec<u8> = item.data()?.to_vec();
            transactions.push(Bytes::from(transaction));
        }

        Ok(transactions)
    }

    /// Decodes the event data into a vector of transactions
    pub fn get_event_transactions(&self, eth_log: &Log) -> Result<Vec<Bytes>, Error> {
        let sender = &eth_log.topics[1];

        let data_bytes = hex::decode(&eth_log.data)
            .map_err(|_| eyre::eyre!("Failed to decode eth_log data from hex"))?;

        let decoded_data: Vec<u8> =
            rlp::decode(&data_bytes).map_err(|_| eyre::eyre!("Failed to decode RLP data"))?;

        // Convert the encoded data into Bytes if needed
        let encoded_data: [u8; 32] = decoded_data
            .try_into()
            .map_err(|_| eyre::eyre!("Invalid encoded data format"))?;

        let encoded_data_bytes = Bytes::from(encoded_data);

        let transactions = self.decode_event_data(encoded_data_bytes)?;

        Ok(transactions)
    }
}
