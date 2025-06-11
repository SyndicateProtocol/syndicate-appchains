//! Types for the `synd-seqchain-verifier`

use alloy::{
    primitives::{keccak256, Address, Bytes, B256, U256},
    sol_types::SolValue as _,
};
use serde::{Deserialize, Serialize};

/// `BlockVerifierInput` is the output of the `synd-seqchain-verifier`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockVerifierInput {
    /// Minimum timestamp
    pub min_timestamp: u64,
    /// Maximum timestamp
    pub max_timestamp: u64,
    /// Minimum block number
    pub min_block_number: u64,
    /// Maximum block number    
    pub max_block_number: u64,
    /// Messages
    pub messages: Vec<L1IncomingMessage>,
    /// Batch
    pub batch: Bytes,
}

/// L1 incoming message
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct L1IncomingMessage {
    /// Header
    pub header: L1IncomingMessageHeader,
    /// L2 message
    pub l2msg: Bytes,
}

/// L1 incoming message header
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct L1IncomingMessageHeader {
    /// Kind
    pub kind: u8,
    /// Sender
    pub sender: Address,
    /// Block number
    pub block_number: u64,
    /// Timestamp
    pub timestamp: u64,
    /// Request ID
    pub request_id: B256,
    /// L1 base fee
    pub base_fee_l1: U256,
}

impl L1IncomingMessage {
    /// Hash the L1 incoming message
    fn hash(&self) -> B256 {
        let message_hash = keccak256(&self.l2msg);
        keccak256(
            (
                [self.header.kind],
                self.header.sender,
                self.header.block_number,
                self.header.timestamp,
                self.header.request_id,
                self.header.base_fee_l1,
                message_hash,
            )
                .abi_encode_packed(),
        )
    }

    /// Accumulate the L1 incoming message
    pub fn accumulate(&self, acc: B256) -> B256 {
        let message_hash = self.hash();
        keccak256((acc, message_hash).abi_encode_packed())
    }
}

/// Validate the settlement chain input
pub fn get_delayed_messages_accumulator(
    delayed_messages: &Vec<L1IncomingMessage>,
    start_delayed_messages_accumulator: B256,
) -> B256 {
    let mut acc = start_delayed_messages_accumulator;
    for delayed_message in delayed_messages {
        acc = delayed_message.accumulate(acc);
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{address, bytes, fixed_bytes};

    #[test]
    fn test_get_delayed_messages_accumulator() {
        let start_acc =
            fixed_bytes!("0x1A2A83E39CD5BE8DAAF9FA3F9AA5A5C1C69654F7CD53485D5CA9B6B5AFE3DECC");

        let msg1 = L1IncomingMessage {
            header: L1IncomingMessageHeader {
                kind: 12,
                sender: address!("0x3A0BB3a5B69711cc64b09240D2694d9f0F07fD07"),
                block_number: 3454550,
                timestamp: 1741397652,
                request_id: fixed_bytes!("0x0000000000000000000000000000000000000000000000000000000000000015"),
                base_fee_l1: U256::from(0),
            },
            l2msg: bytes!("0x28fab3a5b69711cc64b09240d2694d9f0f07ebf60000000000000000000000000000000000000000000000000000000000002710"),
        };

        let msg2 = L1IncomingMessage {
            header: L1IncomingMessageHeader {
                kind: 12,
                sender: address!("0x3A0BB3a5B69711cc64b09240D2694d9f0F07fD07"),
                block_number: 3673391,
                timestamp: 1744666728,
                request_id: fixed_bytes!("0x0000000000000000000000000000000000000000000000000000000000000016"),
                base_fee_l1: U256::from(0),
            },
            l2msg: bytes!("0x28fab3a5b69711cc64b09240d2694d9f0f07ebf60000000000000000000000000000000000000000000000000de0b6b3a7640000"),
        };

        let msg3 = L1IncomingMessage {
            header: L1IncomingMessageHeader {
                kind: 12,
                sender: address!("0x3A0BB3a5B69711cc64b09240D2694d9f0F07fD07"),
                block_number: 3673425,
                timestamp: 1744667292,
                request_id: fixed_bytes!("0x0000000000000000000000000000000000000000000000000000000000000017"),
                base_fee_l1: U256::from(0),
            },
            l2msg: bytes!("0x28fab3a5b69711cc64b09240d2694d9f0f07ebf60000000000000000000000000000000000000000000000008ac7230489e80000"),
        };

        let messages = vec![msg1, msg2, msg3];

        // Calculate expected accumulator by applying messages one by one
        let expected =
            fixed_bytes!("0xdfc8a2a4675e75cf9321ccfdc5d3d0cd97d8754b2b2d3d693a180d2deb01330c");

        // Calculate using the function under test
        let actual = get_delayed_messages_accumulator(&messages, start_acc);

        assert_eq!(actual, expected);
    }
}
