//! The `ingestor` module  handles block polling from a remote Ethereum chain and forwards them to a consumer using a channel

use serde::Deserialize;

#[derive(Clone, Debug)]
/// A struct that contains the block and its receipts
pub struct BlockAndReceipts {
    /// The block       
    pub block: Block,
    /// The receipts
    pub receipts: Vec<Receipt>,
}

#[derive(Deserialize, Debug, Clone)]
/// A struct that contains the block
pub struct Block {
    /// The nonce
    pub nonce: String,
    /// The hash                
    pub hash: String,
    /// The number
    pub number: String,
    /// The parent hash
    #[serde(rename = "parentHash")]
    pub parent_hash: String, // Hash of the parent block
    /// The logs bloom
    #[serde(rename = "logsBloom")]
    pub logs_bloom: String,
    /// The transactions root
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: String, // Root of the transaction trie
    /// The state root
    #[serde(rename = "stateRoot")]
    pub state_root: String, // Root of the final state trie
    /// The receipts root
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: String, // Root of the receipts trie
    /// The timestamp
    pub timestamp: String, // Unix timestamp for the block
    /// The transactions
    pub transactions: Vec<MTransaction>, // Adjust based on actual data
}

#[derive(Deserialize, Debug, Clone)]
/// A struct that contains the transaction
pub struct MTransaction {
    /// The block hash
    #[serde(rename = "blockHash")]
    pub block_hash: String, // Hash of the block where the transaction was included, null if pending
    /// The block number
    #[serde(rename = "blockNumber")]
    pub block_number: String, // Block number where the transaction was included, null if pending
    /// The sender's address
    pub from: String, // Sender's address
    /// The hash of the transaction
    pub hash: String, // Hash of the transaction
    /// The data sent with the transaction
    pub input: String, // Data sent with the transaction
    /// The number of transactions made by the sender
    pub nonce: String, // Number of transactions made by the sender
    /// The receiver's address
    pub to: Option<String>, // Receiver's address, null for contract creation
    /// The index of the transaction in the block   
    #[serde(rename = "transactionIndex")]
    pub transaction_index: String, // Index of the transaction in the block, null if pending
    /// The value transferred in Wei
    pub value: String, // Value transferred in Wei
}

#[derive(Deserialize, Debug, Clone)]
/// A struct that contains the receipt  
pub struct Receipt {
    /// The block hash
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    /// The block number
    #[serde(rename = "blockNumber")]
    pub block_number: String,
    /// The sender's address
    pub from: String,
    /// The receiver's address
    pub to: Option<String>,
    /// The contract address
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<String>,
    /// The logs
    pub logs: Vec<Log>,
    /// The logs bloom
    #[serde(rename = "logsBloom")]
    pub logs_bloom: String,
    /// The status
    pub status: String,
    /// The type of the receipt
    #[serde(rename = "type")]
    pub receipt_type: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
/// A struct that contains the log
pub struct Log {
    /// The block hash
    #[serde(rename = "blockHash")]
    pub block_hash: String, // Can be null if pending
    /// The block number
    #[serde(rename = "blockNumber")]
    pub block_number: String, // Can be null if pending
    /// The transaction index
    pub transaction_index: Option<String>, // Can be null if pending
    /// The address
    #[serde(rename = "address")]
    pub address: String, // Address from which this log originated
    /// The log index
    #[serde(rename = "logIndex")]
    pub log_index: Option<String>, // Can be null if pending
    /// The data
    #[serde(rename = "data")]
    pub data: String, // Contains log data
    /// The removed flag
    #[serde(rename = "removed")]
    pub removed: bool, // True if removed due to reorganization
    /// The topics
    #[serde(rename = "topics")]
    pub topics: Vec<String>, // Array of topics
    /// The transaction hash
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>, // Transaction hash
}
