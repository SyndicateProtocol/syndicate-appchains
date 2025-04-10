//! Types module for metabased-translator

use alloy::{
    hex,
    primitives::{Address, Bytes, B256},
    rpc::types::Header,
};
use async_trait::async_trait;
use eyre::Error;
use fmt::{Display, Formatter, Result as FmtResult};
use serde::{
    de::{self, Deserializer},
    Deserialize, Serialize, Serializer,
};
use std::fmt;
use strum_macros::Display;

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct PartialLogWithTxdata {
    pub address: Address,
    pub topics: Vec<B256>,
    pub data: Bytes,
    pub tx_calldata: Bytes,
}

#[allow(missing_docs)]
impl PartialLogWithTxdata {
    pub fn new(log: alloy::rpc::types::Log, tx_data: Bytes) -> Self {
        Self {
            address: log.address(),
            topics: log.topics().into(),
            data: log.data().data.clone(),
            tx_calldata: tx_data,
        }
    }
}

#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct PartialBlock {
    pub number: u64,
    pub hash: B256,
    pub timestamp: u64,
    pub parent_hash: B256,
    pub logs: Vec<PartialLogWithTxdata>,
}

#[allow(missing_docs)]
impl PartialBlock {
    pub fn new(header: Header, logs: Vec<PartialLogWithTxdata>) -> Self {
        Self {
            hash: header.hash,
            logs,
            parent_hash: header.parent_hash,
            timestamp: header.timestamp,
            number: header.number,
        }
    }
}

#[allow(missing_docs)] // self-explanatory
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Chain {
    Sequencing,
    Settlement,
}

impl From<Chain> for &'static str {
    fn from(chain: Chain) -> &'static str {
        match chain {
            Chain::Sequencing => "sequencing",
            Chain::Settlement => "settlement",
        }
    }
}

/// A trait for processing slots of blocks from the sequencing and settlement chains.
/// Implementors of this trait define how to translate each slot into the mchain state.
#[async_trait]
pub trait SlotProcessor: Send {
    /// Process a single slot
    async fn process_slot(&self, slot: &Slot) -> Result<(), Error>;
}

/// A `Slot` is a collection of source chain blocks to be sent to the block builder.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Slot {
    /// the block from the sequencing chain to be included in the slot.
    pub sequencing: PartialBlock,
    /// the blocks from the settlement chain to be included in the slot.
    pub settlement: Vec<PartialBlock>,
}

impl Slot {
    /// Creates a new slot
    pub const fn new(sequencing_block: PartialBlock) -> Self {
        Self { sequencing: sequencing_block, settlement: Vec::new() }
    }

    /// Adds a block to the slot's chain-specific block list
    pub fn push_settlement_block(&mut self, block: PartialBlock) {
        self.settlement.push(block)
    }

    /// Returns the timestamp of the slot
    pub const fn timestamp(&self) -> u64 {
        self.sequencing.timestamp
    }
}

impl Display for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Slot [Sequencing block: {}, Settlement blocks (total: {}): {}]",
            format_block(&self.sequencing),
            self.settlement.len(),
            format_blocks(&self.settlement),
        )
    }
}

fn format_blocks(blocks: &[PartialBlock]) -> String {
    if blocks.is_empty() {
        return "none".to_string();
    }
    blocks.iter().map(|b| format!("#{} ({})", b.number, b.hash)).collect::<Vec<_>>().join(", ")
}

fn format_block(b: &PartialBlock) -> String {
    format!("number: {}, hash: {}, total_logs: {}", b.number, b.hash, b.logs.len())
}

/// A reference to a block containing just its number and timestamp.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct BlockRef {
    /// The block number.
    pub number: u64,
    /// The block timestamp.
    pub timestamp: u64,
    /// The block hash.
    pub hash: B256,
}

impl BlockRef {
    /// Creates a new `BlockRef` from a `Block`.
    pub const fn new(block: &PartialBlock) -> Self {
        Self { number: block.number, timestamp: block.timestamp, hash: block.hash }
    }
}

impl Display for BlockRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "number: {}, ts: {}, hash: {}", self.number, self.timestamp, self.hash)
    }
}

/// A known state of the translator
#[derive(Debug)]
pub struct KnownState {
    /// mchain block number for this state
    pub mchain_block_number: u64,
    /// The latest block from the sequencing chain that has been processed
    pub sequencing_block: BlockRef,
    /// The latest block from the settlement chain that has been processed
    pub settlement_block: BlockRef,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
/// **`Block`**: Represents an Ethereum block, including details like its hash, number, timestamp,
/// and the transactions it contains.
pub struct Block {
    /// The hash of the block.
    #[serde(deserialize_with = "deserialize_b256", serialize_with = "serialize_b256")]
    pub hash: B256,
    /// The block number.
    #[serde(deserialize_with = "deserialize_hex_to_u64", serialize_with = "serialize_hex_u64")]
    pub number: u64,
    /// The hash of the parent block.
    #[serde(deserialize_with = "deserialize_b256", serialize_with = "serialize_b256")]
    pub parent_hash: B256,
    /// The logs bloom filter for the block.
    pub logs_bloom: String,
    /// The root hash of the transactions trie.
    pub transactions_root: String,
    /// The root hash of the final state trie.
    pub state_root: String,
    /// The root hash of the receipts trie.
    pub receipts_root: String,
    /// The timestamp when the block was mined, in Unix time.
    #[serde(deserialize_with = "deserialize_hex_to_u64", serialize_with = "serialize_hex_u64")]
    pub timestamp: u64,
    /// The transactions included in the block.
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
/// **`Transaction`**: Represents a single transaction within a block, including fields such as the
/// transaction hash, sender/recipient addresses, value, and input data.
pub struct Transaction {
    /// The hash of the block containing this transaction, or `null` if pending.
    #[serde(deserialize_with = "deserialize_b256", serialize_with = "serialize_b256")]
    pub block_hash: B256,
    /// The number of the block containing this transaction, or `null` if pending.
    #[serde(deserialize_with = "deserialize_hex_to_u64", serialize_with = "serialize_hex_u64")]
    pub block_number: u64,
    /// The sender's address.
    #[serde(deserialize_with = "deserialize_address", serialize_with = "serialize_address")]
    pub from: Address,
    /// The transaction hash.
    #[serde(deserialize_with = "deserialize_b256", serialize_with = "serialize_b256")]
    pub hash: B256,
    /// The data payload of the transaction.
    #[serde(deserialize_with = "deserialize_bytes", serialize_with = "serialize_bytes")]
    pub input: Bytes,
    /// The number of transactions sent by the sender.
    #[serde(deserialize_with = "deserialize_hex_to_u64", serialize_with = "serialize_hex_u64")]
    pub nonce: u64,
    /// The recipient's address, or `null` if the transaction creates a contract.
    #[serde(
        deserialize_with = "deserialize_optional_address",
        serialize_with = "serialize_optional_address"
    )]
    pub to: Option<Address>,
    /// The index of this transaction in the block.
    pub transaction_index: String,
    /// The amount of Wei transferred.
    pub value: String,
    /// The amount of gas
    pub gas: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
/// **`Receipt`**: Contains the result of a transaction, including fields like status, logs, and
/// potentially a contract address if one was created.
pub struct Receipt {
    /// The hash of the block containing the transaction.
    #[serde(deserialize_with = "deserialize_b256", serialize_with = "serialize_b256")]
    pub block_hash: B256,
    /// The number of the block containing the transaction.
    #[serde(deserialize_with = "deserialize_hex_to_u64", serialize_with = "serialize_hex_u64")]
    pub block_number: u64,
    /// The sender's address.
    #[serde(deserialize_with = "deserialize_address", serialize_with = "serialize_address")]
    pub from: Address,
    /// The recipient's address.
    #[serde(
        deserialize_with = "deserialize_optional_address",
        serialize_with = "serialize_optional_address"
    )]
    pub to: Option<Address>,
    /// The address of the contract created by the transaction, if applicable.
    #[serde(
        deserialize_with = "deserialize_optional_address",
        serialize_with = "serialize_optional_address"
    )]
    pub contract_address: Option<Address>,
    /// The logs generated by the transaction.
    pub logs: Vec<Log>,
    /// The logs bloom filter for the transaction.
    pub logs_bloom: String,
    /// The transaction's execution status.
    #[serde(deserialize_with = "deserialize_hex_to_u64", serialize_with = "serialize_hex_u64")]
    pub status: u64,
    /// The receipt type, if available.
    #[serde(rename = "type")]
    pub receipt_type: String,
    /// Transaction index in block
    #[serde(deserialize_with = "deserialize_hex_to_u64", serialize_with = "serialize_hex_u64")]
    pub transaction_index: u64,
    /// Transaction hash
    #[serde(deserialize_with = "deserialize_b256", serialize_with = "serialize_b256")]
    pub transaction_hash: B256,
    /// Gas used
    #[serde(deserialize_with = "deserialize_hex_to_u64", serialize_with = "serialize_hex_u64")]
    pub gas_used: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
/// **`Log`**: Represents an individual log entry emitted by a smart contract during a transaction,
/// containing information such as topics, data, and whether it was removed due to a reorganization.
pub struct Log {
    /// The hash of the block containing the log, or `null` if pending.
    #[serde(deserialize_with = "deserialize_b256", serialize_with = "serialize_b256")]
    pub block_hash: B256,
    /// The number of the block containing the log, or `null` if pending.
    #[serde(deserialize_with = "deserialize_hex_to_u64", serialize_with = "serialize_hex_u64")]
    pub block_number: u64,
    /// The index of the transaction that generated the log.
    #[serde(deserialize_with = "deserialize_hex_to_u64", serialize_with = "serialize_hex_u64")]
    pub transaction_index: u64,
    /// The address from which the log originated.
    #[serde(deserialize_with = "deserialize_address", serialize_with = "serialize_address")]
    pub address: Address,
    /// The index of the log entry.
    #[serde(deserialize_with = "deserialize_hex_to_u64", serialize_with = "serialize_hex_u64")]
    pub log_index: u64,
    /// The data associated with the log.
    #[serde(deserialize_with = "deserialize_bytes", serialize_with = "serialize_bytes")]
    pub data: Bytes,
    /// A flag indicating if the log was removed due to a chain reorganization.
    pub removed: bool,
    /// The topics associated with the log.
    #[serde(deserialize_with = "deserialize_b256_vec", serialize_with = "serialize_b256_vec")]
    pub topics: Vec<B256>,
    /// The hash of the transaction that generated the log.
    #[serde(deserialize_with = "deserialize_b256", serialize_with = "serialize_b256")]
    pub transaction_hash: B256,
}

fn deserialize_address<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    let address_str: String = Deserialize::deserialize(deserializer)?;
    let address: Address = address_str
        .parse()
        .map_err(|err| de::Error::custom(format!("Failed to parse address: {err}")))?;
    Ok(address)
}

fn deserialize_optional_address<'de, D>(deserializer: D) -> Result<Option<Address>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    if let Some(address_str) = opt {
        let address: Address = address_str
            .parse()
            .map_err(|err| de::Error::custom(format!("Failed to parse address: {err}")))?;
        Ok(Some(address))
    } else {
        Ok(None)
    }
}

fn deserialize_hex_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let hex_str: String = Deserialize::deserialize(deserializer)?;
    u64::from_str_radix(hex_str.trim_start_matches("0x"), 16)
        .map_err(|err| de::Error::custom(format!("Failed to parse hex to u64: {}", err)))
}

fn deserialize_b256<'de, D>(deserializer: D) -> Result<B256, D::Error>
where
    D: Deserializer<'de>,
{
    let hex_str: String = Deserialize::deserialize(deserializer)?;
    let decoded = hex::decode(hex_str.trim_start_matches("0x"))
        .map_err(|err| de::Error::custom(format!("Failed to decode hex string: {err}")))?;
    let array: [u8; 32] = decoded
        .try_into()
        .map_err(|_| de::Error::custom("Failed to convert to a 32-byte array"))?;
    Ok(B256::from(array))
}

fn deserialize_b256_vec<'de, D>(deserializer: D) -> Result<Vec<B256>, D::Error>
where
    D: Deserializer<'de>,
{
    let vec: Vec<String> = Deserialize::deserialize(deserializer)?;
    vec.into_iter()
        .map(|s| {
            let decoded = hex::decode(s.trim_start_matches("0x"))
                .map_err(|err| de::Error::custom(format!("Failed to decode hex string: {err}")))?;
            let array: [u8; 32] = decoded
                .try_into()
                .map_err(|_| de::Error::custom("Failed to convert to a 32-byte array"))?;
            Ok(B256::from(array))
        })
        .collect()
}

fn deserialize_bytes<'de, D>(deserializer: D) -> Result<Bytes, D::Error>
where
    D: Deserializer<'de>,
{
    let hex_str: String = Deserialize::deserialize(deserializer)?;
    hex::decode(hex_str.trim_start_matches("0x"))
        .map(Bytes::from)
        .map_err(|err| de::Error::custom(format!("Failed to decode hex string: {err}")))
}

fn serialize_b256<S>(b256: &B256, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("0x{}", hex::encode(b256.as_slice())))
}

fn serialize_b256_vec<S>(vec: &[B256], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::SerializeSeq;
    let mut seq = serializer.serialize_seq(Some(vec.len()))?;
    for b256 in vec {
        seq.serialize_element(&format!("0x{}", hex::encode(b256.as_slice())))?;
    }
    seq.end()
}

fn serialize_address<S>(addr: &Address, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("0x{}", hex::encode(addr.as_slice())))
}

fn serialize_optional_address<S>(opt: &Option<Address>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match opt {
        Some(addr) => serializer.serialize_str(&format!("0x{}", hex::encode(addr.as_slice()))),
        None => serializer.serialize_none(),
    }
}

fn serialize_bytes<S>(bytes: &Bytes, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("0x{}", hex::encode(bytes)))
}

fn serialize_hex_u64<S>(num: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("0x{:x}", num))
}

#[cfg(test)]
mod test {
    use super::*;
    use alloy::{hex::FromHex, primitives::B256};

    fn create_test_slot() -> Slot {
        // Add sequencing chain block
        let sequencing_block = PartialBlock {
            hash: B256::from_hex(
                "0x1234567890123456789012345678901234567890123456789012345678901234",
            )
            .unwrap(),
            number: 100,
            parent_hash: B256::ZERO,
            timestamp: 1000,
            logs: vec![PartialLogWithTxdata {
                address: Address::from_hex("0x1234567890123456789012345678901234567890").unwrap(),
                topics: vec![B256::from_hex(
                    "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
                )
                .unwrap()],
                data: Bytes::from_hex("0xdead").unwrap(),
                tx_calldata: Bytes::from_hex("0xbeef").unwrap(),
            }],
        };

        let mut slot = Slot::new(sequencing_block);

        // Add settlement chain block
        slot.push_settlement_block(PartialBlock {
            hash: B256::from_hex(
                "0x5678901234567890123456789012345678901234567890123456789012345678",
            )
            .unwrap(),
            number: 200,
            parent_hash: B256::from_hex(
                "0x1111111111111111111111111111111111111111111111111111111111111111",
            )
            .unwrap(),
            timestamp: 1100,
            logs: vec![],
        });

        slot
    }

    #[test]
    fn test_bincode_serialization() {
        let slot = create_test_slot();
        let encoded = bincode::serialize(&slot).unwrap();
        let decoded: Slot = bincode::deserialize(&encoded).unwrap();
        assert_eq!(decoded, slot);
    }

    #[test]
    fn test_json_serialization() {
        let slot = create_test_slot();
        let json = serde_json::to_string(&slot).unwrap();
        println!("{}", json);
        let decoded: Slot = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, slot);
    }
}
