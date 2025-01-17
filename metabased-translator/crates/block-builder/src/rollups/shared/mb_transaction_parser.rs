use alloy::{
    json_abi::JsonAbi,
    primitives::{keccak256, Address, Bytes, B256},
    sol_types::abi::decode,
};
use common::types::Log;
use eyre::{eyre, Error};
use rlp::Rlp;

#[derive(Debug, Clone)]
pub struct TransactionProcessed {
    pub encoded_data: Bytes,
    pub sender: Address,
}

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

const TRANSACTION_PROCESSED_SIG: &str = "TransactionProcessed(address,bytes)";
const TRANSACTION_PROCESSED_SIG_HASH: B256 =
    B256::from(keccak256(TRANSACTION_PROCESSED_SIG.as_bytes()));

pub struct MBTransactionParser {
    sequencing_contract_abi: String,
    sequencing_contract_address: Address,
}

impl MBTransactionParser {
    pub fn new(abi: String, sequencing_contract_address: Address) -> Self {
        let sequencing_contract_abi = JsonAbi::from_str(abi).unwrap();
        Self {
            sequencing_contract_abi,
            sequencing_contract_address,
        }
    }

    pub fn is_log_transaction_processed(&self, eth_log: Log) -> bool {
        eth_log.address == self.sequencing_contract_address
            && eth_log
                .topics
                .get(0)
                .map(|t| *t == *TRANSACTION_PROCESSED_SIG_HASH)
                .unwrap_or(false)
    }

    pub fn decode_event_data(&self, data: Bytes) -> Result<Vec<Bytes>, Error> {
        if data.is_empty() {
            return eyre!("No data provided for decoding".to_string(),);
        }

        let rlp = Rlp::new(&data);
        let mut transactions = Vec::new();

        for item in rlp.iter() {
            let transaction: Vec<u8> = item.data()?.to_vec();
            transactions.push(Bytes::from(transaction));
        }

        Ok(transactions)
    }

    pub fn get_event_transactions(&self, eth_log: &Log) -> Result<Vec<Bytes>, Error> {
        let sender = Address::from_slice(&eth_log.topics[1][..]);

        let encoded_data = decode(&[Bytes], &eth_log.data)?
            .pop()
            .ok_or_else(|| eyre!("Missing EncodedData field".to_string()))?;

        let encoded_data: Bytes = encoded_data
            .into_bytes()
            .ok_or_else(|| eyre!("Invalid EncodedData".to_string()))?
            .into();

        let transactions = self.decode_event_data(encoded_data)?;

        Ok(transactions)
    }
}
