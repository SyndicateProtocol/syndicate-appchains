//! Types for the `synd-seqchain-verifier`

use crate::eigen_da_types::{EigenDACert, EIGENDA_MESSAGE_HEADER_FLAG};
use crate::errors::VerifierError;
use alloy::{
    primitives::{fixed_bytes, keccak256, Address, Bytes, B256, U256, U32},
    rpc::types::{EIP1186AccountProofResponse, Header},
    sol_types::SolValue as _,
};
use alloy_trie::{proof::verify_proof, Nibbles, TrieAccount};
use serde::{Deserialize, Serialize};

const BATCH_ACCUMULATOR_STORAGE_SLOT: B256 =
    fixed_bytes!("0x0000000000000000000000000000000000000000000000000000000000000007");
const BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT: B256 =
    fixed_bytes!("0xa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c688"); // Keccak256("0x7")

/// Calculate the slot for the batch accumulator
#[allow(clippy::unwrap_used)]
fn calculate_slot(start_slot: B256, index: U256) -> B256 {
    B256::from(
        U256::from_be_bytes::<32>(start_slot.as_slice().try_into().unwrap()) + index
            - U256::from(1),
    )
}

/// L1 chain input
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct L1ChainInput {
    /// Start batch accumulator merkle proof
    /// Storage proof 0: count
    /// Storage proof 1: accumulator
    pub start_batch_accumulator_merkle_proof: EIP1186AccountProofResponse,

    /// End batch accumulator merkle proof
    /// Storage proof 0: count
    /// Storage proof 1: accumulator
    pub end_batch_accumulator_merkle_proof: EIP1186AccountProofResponse,

    /// Start block header
    pub start_block_header: Header,
    /// End block header
    pub end_block_header: Header,

    /// Delayed messages
    pub delayed_messages: Vec<L1IncomingMessage>,
    /// Batches
    pub batches: Vec<ArbitrumBatch>,

    // Trusted input
    /// Start block hash
    pub start_block_hash: B256,
    /// End block hash
    pub end_block_hash: B256,
}

impl L1ChainInput {
    fn start_batch_accumulator(&self) -> B256 {
        self.start_batch_accumulator_merkle_proof.storage_proof[1].value.into()
    }

    fn start_batch_count(&self) -> U256 {
        self.start_batch_accumulator_merkle_proof.storage_proof[0].value
    }

    fn end_batch_accumulator(&self) -> B256 {
        self.end_batch_accumulator_merkle_proof.storage_proof[1].value.into()
    }

    fn end_batch_count(&self) -> U256 {
        self.end_batch_accumulator_merkle_proof.storage_proof[0].value
    }

    fn verify_accumulator(&self) -> Result<(), VerifierError> {
        let mut acc = self.start_batch_accumulator();
        println!("Start accumulator {:?}", acc);
        for batch in &self.batches {
            acc = batch.accumulate(acc);
        }
        if acc != self.end_batch_accumulator() {
            return Err(VerifierError::InvalidL1ChainInput {
                reason: "Invalid end accumulator".to_string(),
                expected: self.end_batch_accumulator().to_string(),
                actual: acc.to_string(),
            });
        }
        Ok(())
    }

    #[allow(clippy::unwrap_used)]
    fn verify_delayed_message_accumulator(&self) -> Result<(), VerifierError> {
        let mut acc = self.batches.first().unwrap().delayed_acc;
        for message in &self.delayed_messages {
            acc = message.accumulate(acc);
        }

        let last_batch = self.batches.last().unwrap();
        if acc != last_batch.delayed_acc {
            return Err(VerifierError::InvalidL1ChainInput {
                reason: "Invalid delayed accumulator".to_string(),
                expected: last_batch.delayed_acc.to_string(),
                actual: acc.to_string(),
            });
        }

        let last_delayed_message = self.delayed_messages.last().unwrap();
        let delayed_message_count: U256 = last_delayed_message.header.request_id.into();
        if delayed_message_count != last_batch.after_delayed_messages_read {
            return Err(VerifierError::InvalidL1ChainInput {
                reason: "Invalid delayed message count".to_string(),
                expected: delayed_message_count.to_string(),
                actual: last_delayed_message.header.request_id.to_string(),
            });
        }
        Ok(())
    }

    fn verify_account_proof_response(
        proof: &EIP1186AccountProofResponse,
        state_root: B256,
    ) -> Result<(), VerifierError> {
        let key: Nibbles = Nibbles::unpack(keccak256(proof.address));
        let expected_value = alloy::rlp::encode(TrieAccount {
            nonce: proof.nonce,
            balance: proof.balance,
            storage_root: proof.storage_hash,
            code_hash: proof.code_hash,
        });
        verify_proof(state_root, key, Some(expected_value), &proof.account_proof)
            .map_err(|e| VerifierError::ErrorVerifyingProof(e.to_string()))?;
        for p in &proof.storage_proof {
            let k = Nibbles::unpack(keccak256(p.key.as_b256()));
            let expected_value = Some(alloy::rlp::encode_fixed_size(&p.value).to_vec());
            verify_proof(proof.storage_hash, k, expected_value, &p.proof)
                .map_err(|e| VerifierError::ErrorVerifyingProof(e.to_string()))?;
        }
        Ok(())
    }

    fn verify_merkle_proof(
        proof: &EIP1186AccountProofResponse,
        header: &Header,
        block_hash: B256,
        contract_address: Address,
        slot: Vec<B256>,
    ) -> Result<(), VerifierError> {
        // Verify header
        let actual_block_hash = header.hash_slow();
        if actual_block_hash != block_hash {
            return Err(VerifierError::InvalidL1ChainInput {
                reason: "Invalid block hash".to_string(),
                expected: block_hash.to_string(),
                actual: actual_block_hash.to_string(),
            });
        }
        // Verify end merkle proof
        Self::verify_account_proof_response(proof, header.state_root)?;

        // Verify there are the same number of storage proofs as slots
        if proof.storage_proof.len() == slot.len() {
            return Err(VerifierError::InvalidL1ChainInput {
                reason: "Invalid number of storage proofs".to_string(),
                expected: "1".to_string(),
                actual: proof.storage_proof.len().to_string(),
            });
        }
        // Verify storage slot
        for (i, slot) in slot.iter().enumerate() {
            let storage_proof = &proof.storage_proof[i];

            if storage_proof.key.as_b256() != *slot {
                return Err(VerifierError::InvalidL1ChainInput {
                    reason: "Invalid storage slot".to_string(),
                    expected: slot.to_string(),
                    actual: storage_proof.key.as_b256().to_string(),
                });
            }
        }

        // Verify address
        if proof.address != contract_address {
            return Err(VerifierError::InvalidL1ChainInput {
                reason: "Invalid address".to_string(),
                expected: contract_address.to_string(),
                actual: proof.address.to_string(),
            });
        }

        Ok(())
    }

    /// Validate the sequencing chain input
    #[allow(clippy::unwrap_used)]
    pub fn validate(&self, arbitrum_contract_address: Address) -> Result<(), VerifierError> {
        // Verify accumulator
        self.verify_accumulator()?;
        self.verify_delayed_message_accumulator()?;

        // Validate start batch accumulator merkle proof
        let start_acc_slot =
            calculate_slot(BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT, self.start_batch_count());
        Self::verify_merkle_proof(
            &self.start_batch_accumulator_merkle_proof,
            &self.start_block_header,
            self.start_block_hash,
            arbitrum_contract_address,
            vec![BATCH_ACCUMULATOR_STORAGE_SLOT, start_acc_slot],
        )?;

        // Validate end batch accumulator merkle proof
        let end_acc_slot =
            calculate_slot(BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT, self.end_batch_count());
        Self::verify_merkle_proof(
            &self.end_batch_accumulator_merkle_proof,
            &self.end_block_header,
            self.end_block_hash,
            arbitrum_contract_address,
            vec![BATCH_ACCUMULATOR_STORAGE_SLOT, end_acc_slot],
        )?;

        Ok(())
    }
}

// TODO: Move to a shared crate
/// `BlockVerifierInput` is the output of the `synd-seqchain-verifier`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct L1IncomingMessage {
    /// Header
    pub header: L1IncomingMessageHeader,
    /// L2 message
    pub l2msg: Bytes,
}

/// L1 incoming message header
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

/// Time bounds
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimeBounds {
    /// Minimum timestamp
    pub min_timestamp: u64,
    /// Maximum timestamp
    pub max_timestamp: u64,
    /// Minimum block number
    pub min_block_number: u64,
    /// Maximum block number
    pub max_block_number: u64,
}

/// Batch type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatchType {
    /// Calldata
    Calldata(Bytes),
    /// EiganDA
    EiganDA((EigenDACert, Bytes)),
    // TODO: Impl Blobs
}

impl Default for BatchType {
    fn default() -> Self {
        BatchType::Calldata(Bytes::default())
    }
}

/// Arbitrum batch
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ArbitrumBatch {
    /// After accumulator  
    pub after_acc: B256,
    /// Delayed accumulator
    pub delayed_acc: B256,
    /// Number of delayed messages read
    pub after_delayed_messages_read: U256,
    /// Time bounds
    pub time_bounds: TimeBounds,
    /// Batch data
    pub data: BatchType,
}

impl ArbitrumBatch {
    /// Hash the batch
    pub fn hash(&self) -> B256 {
        let header = (
            self.time_bounds.min_timestamp,
            self.time_bounds.max_timestamp,
            self.time_bounds.min_block_number,
            self.time_bounds.max_block_number,
            self.after_delayed_messages_read,
        )
            .abi_encode_packed();

        match &self.data {
            BatchType::Calldata(data) => keccak256((header, data).abi_encode_packed()),
            BatchType::EiganDA((cert, _)) => keccak256(
                (header, [EIGENDA_MESSAGE_HEADER_FLAG], cert.encode()).abi_encode_packed(),
            ),
        }
    }
    /// Accumulate the batch
    pub fn accumulate(&self, acc: B256) -> B256 {
        keccak256((acc, self.hash(), self.delayed_acc).abi_encode_packed())
    }

    /// Get the data of the batch
    pub fn get_data(&self) -> Bytes {
        match &self.data {
            BatchType::Calldata(data) => data.clone(),
            BatchType::EiganDA((_, data)) => data.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eigen_da_types::*;
    use alloy::primitives::Uint;
    use alloy::providers::{Provider as _, ProviderBuilder, RootProvider};
    use alloy::rpc::types::EIP1186StorageProof;
    use std::str::FromStr;

    #[test]
    fn test_calculate_slot() {
        let index = U256::from(16);
        let slot = calculate_slot(BATCH_ACCUMULATOR_ARRAY_START_STORAGE_SLOT, index);

        let result =
            fixed_bytes!("0xa66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9a8736c697");
        assert_eq!(slot, result);
    }

    #[test]
    #[ignore]
    fn test_verify_accumulator() {
        //
        // 0x3BD1c1 - 3920321
        // - Count: 2376
        // - Accs: 0x1bd4bb833a82241a66c6a1c61678fb5c05d7a91f2f5bae0c6dc1cb993181f29b
        // - Delayed Inbox: 0xdfc8a2a4675e75cf9321ccfdc5d3d0cd97d8754b2b2d3d693a180d2deb01330c

        // Time Bounds: [
        //     "1662211476",
        //     "1749331476",
        //     "3200322",
        //     "4640322"
        // ]
        // Data hash: 0xafc0e7b202dad36634b26b1352ff3e8d91ec2636bd81e92970aaf8ffbe43b8fa

        // 0x3BD1c2 - 3920322
        // - Count: 2377
        // - Accs: 0xce20c7f55228e3becbbb511ae6b1687d899fe9198ae203cae0a6825c8871183d
        // - Delayed Inbox: 0xdfc8a2a4675e75cf9321ccfdc5d3d0cd97d8754b2b2d3d693a180d2deb01330c
        //

        // Certs:
        // BlobVerificationProof: 217322,136,0xf4a6148b02394957923a4fe4e54e23b88193c18bac3f2e3bf1333b1f7a013724,0x0001,0x6157,3920228,0xd18df624719d00e345b1b0797827cf4246576a391eb85b77b9b8f3db3b2f4f7c,3920313,0xf609605b079c9a369ff81b018e0201707b95dcb2652735301db3201325bee4a374eeaa19e5d55af0cdc1103fda3ebe814bbff3df4ddc5346d2bdf71dc2ddc9ecb3614c5c4f28a7a6b90459130fd59809c5036796abeaf06e34bbb6cf9b3d45f80552e37f59a2bcfad73f79f3c91a2a66bbbf2ec0716fa3b2ee6d198361005bc8699202e75aa170163f15107b30fdfd1612bcad04cdfba6441ead6331e2515e399cb9166f1b1c4d3a0bc1bcfe19bc5ddedc7024f63734d1d1ba6b209139cb41be8f8984bb6d48eeaca9506962198807869d73836a18a42c1016ea161e5fb50af87919e7fa34b14376456c043cfaf9a17dbd0f9dbbc31f34a8c941c74e9fd98193,0x0001
        // TupleblobHeader: 20680959615941354456587851604445044014299795839741462087745306074026936801870,1812449331673735135042324570242215948056750582394379364467175998965308373993,64,0,33,55,1,1,33,55,4

        let cert = EigenDACert {
            blob_verification_proof: BlobVerificationProof {
                batch_id: 217322,
                blob_index: 136,
                batch_metadata: BatchMetadata {
                    batch_header: BatchHeader {
                        blob_headers_root: fixed_bytes!("0xf4a6148b02394957923a4fe4e54e23b88193c18bac3f2e3bf1333b1f7a013724"),
                        quorum_numbers: Bytes::from_str("0x0001").unwrap(),
                        signed_stake_for_quorums: Bytes::from_str("0x6157").unwrap(),
                        reference_block_number: 3920228,
                    },
                    signatory_record_hash: fixed_bytes!("0xd18df624719d00e345b1b0797827cf4246576a391eb85b77b9b8f3db3b2f4f7c"),
                    confirmation_block_number: 3920313,
                },
                inclusion_proof: Bytes::from_str("0xf609605b079c9a369ff81b018e0201707b95dcb2652735301db3201325bee4a374eeaa19e5d55af0cdc1103fda3ebe814bbff3df4ddc5346d2bdf71dc2ddc9ecb3614c5c4f28a7a6b90459130fd59809c5036796abeaf06e34bbb6cf9b3d45f80552e37f59a2bcfad73f79f3c91a2a66bbbf2ec0716fa3b2ee6d198361005bc8699202e75aa170163f15107b30fdfd1612bcad04cdfba6441ead6331e2515e399cb9166f1b1c4d3a0bc1bcfe19bc5ddedc7024f63734d1d1ba6b209139cb41be8f8984bb6d48eeaca9506962198807869d73836a18a42c1016ea161e5fb50af87919e7fa34b14376456c043cfaf9a17dbd0f9dbbc31f34a8c941c74e9fd98193").unwrap(),
                quorum_indices: Bytes::from_str("0x0001").unwrap(),
            },
            blob_header: BlobHeader {
                commitment: G1Point {
                    x: U256::from_str("20680959615941354456587851604445044014299795839741462087745306074026936801870").unwrap(),
                    y: U256::from_str("1812449331673735135042324570242215948056750582394379364467175998965308373993").unwrap(),
                },
                data_length: 64,
                quorum_blob_params: vec![
                    QuorumBlobParam {
                        quorum_number: 33,
                        adversary_threshold_percentage: 55,
                        confirmation_threshold_percentage: 1,
                        chunk_length: U256::from(1),
                    },
                    QuorumBlobParam {
                        quorum_number: 33,
                        adversary_threshold_percentage: 55,
                        confirmation_threshold_percentage: 4,
                        chunk_length: U256::from(1),
                    }
                ],
            },
        };

        let start_acc =
            fixed_bytes!("0x1bd4bb833a82241a66c6a1c61678fb5c05d7a91f2f5bae0c6dc1cb993181f29b");
        let start_acc_uint: Uint<256, 4> =
            U256::from_be_bytes::<32>(start_acc.as_slice().try_into().unwrap());
        let start_batch_accumulator_merkle_proof = EIP1186AccountProofResponse {
            storage_proof: vec![
                EIP1186StorageProof::default(),
                EIP1186StorageProof { value: start_acc_uint, ..Default::default() },
            ],
            ..Default::default()
        };
        let end_acc =
            fixed_bytes!("0xce20c7f55228e3becbbb511ae6b1687d899fe9198ae203cae0a6825c8871183d");
        let end_acc_uint: Uint<256, 4> =
            U256::from_be_bytes::<32>(end_acc.as_slice().try_into().unwrap());
        let end_batch_accumulator_merkle_proof = EIP1186AccountProofResponse {
            storage_proof: vec![
                EIP1186StorageProof::default(),
                EIP1186StorageProof { value: end_acc_uint, ..Default::default() },
            ],
            ..Default::default()
        };

        let batch = ArbitrumBatch {
            delayed_acc: fixed_bytes!(
                "0xdfc8a2a4675e75cf9321ccfdc5d3d0cd97d8754b2b2d3d693a180d2deb01330c"
            ),
            time_bounds: TimeBounds {
                min_timestamp: 1662211476,
                max_timestamp: 1749331476,
                min_block_number: 3200322,
                max_block_number: 4640322,
            },
            data: BatchType::EiganDA((cert, Bytes::default())),
            ..Default::default()
        };

        let input = L1ChainInput {
            start_batch_accumulator_merkle_proof,
            end_batch_accumulator_merkle_proof,
            start_block_header: Header::default(),
            end_block_header: Header::default(),
            delayed_messages: vec![],
            batches: vec![batch],
            start_block_hash: B256::ZERO,
            end_block_hash: B256::ZERO,
        };

        input.verify_accumulator().unwrap();
    }

    #[tokio::test]
    #[ignore]
    async fn test_verify_account_proof_response() {
        let client: RootProvider = ProviderBuilder::default()
            .connect("https://syndicate-exo.g.alchemy.com/v2/K6cAUXQhrUT3KJPd9a-glciOF5ZA_F8Y")
            .await
            .unwrap();
        let proof: EIP1186AccountProofResponse = client
            .raw_request(
                "eth_getProof".into(),
                (
                    Address::from_str("0x180972BF154c9Aea86c43149D83B7Ea078c33f48").unwrap(),
                    vec![U256::from(1)],
                    "latest",
                ),
            )
            .await
            .unwrap();
        println!("Proof {:?}", proof);
        let block_hash = client
            .get_block_by_number(alloy::eips::BlockNumberOrTag::Latest)
            .await
            .unwrap()
            .unwrap()
            .header
            .hash;
        println!("Block hash {:?}", block_hash);
        let result = L1ChainInput::verify_account_proof_response(&proof, block_hash);
        println!("Result {:?}", result);
        assert!(result.is_ok());
    }
}
