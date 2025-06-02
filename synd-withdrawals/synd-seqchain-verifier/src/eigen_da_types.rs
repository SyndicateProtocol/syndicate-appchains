use alloy::{
    primitives::{Bytes, B256, U256},
    sol_types::SolValue,
};
use serde::{Deserialize, Serialize};

/// EigenDA message header flag
pub const EIGENDA_MESSAGE_HEADER_FLAG: u8 = 0xed;

/// G1 point
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct G1Point {
    /// The x coordinate of the G1 point
    pub x: U256,
    /// The y coordinate of the G1 point
    pub y: U256,
}

impl G1Point {
    /// Encode the G1 point
    pub fn encode(&self) -> Vec<u8> {
        (self.x, self.y).abi_encode_packed()
    }
}

/// Quorum blob param

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuorumBlobParam {
    /// The quorum number
    pub quorum_number: u8,
    /// The adversary threshold percentage
    pub adversary_threshold_percentage: u8,
    /// The confirmation threshold percentage
    pub confirmation_threshold_percentage: u8,
    /// The chunk length
    pub chunk_length: U256,
}

impl QuorumBlobParam {
    /// Pack the quorum blob param
    pub fn pack(&self) -> (U256, U256, U256, U256) {
        (
            U256::from(self.quorum_number),
            U256::from(self.adversary_threshold_percentage),
            U256::from(self.confirmation_threshold_percentage),
            self.chunk_length,
        )
    }
}

/// Blob header

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlobHeader {
    /// The commitment G1 point
    pub commitment: G1Point,
    /// The length of the data
    pub data_length: u32,
    /// The quorum blob parameters
    pub quorum_blob_params: Vec<QuorumBlobParam>,
}

impl BlobHeader {
    /// Encode the blob header
    pub fn encode(&self) -> Vec<u8> {
        let params = self.quorum_blob_params.iter().map(|p| p.pack()).collect::<Vec<_>>();
        (self.commitment.encode(), self.data_length, params).abi_encode()
    }
}

/// Batch header

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchHeader {
    /// The root of blob headers
    pub blob_headers_root: B256,
    /// The quorum numbers
    pub quorum_numbers: Bytes,
    /// The signed stake for quorums
    pub signed_stake_for_quorums: Bytes,
    /// The reference block number
    pub reference_block_number: u32,
}

impl BatchHeader {
    /// Encode the batch header
    pub fn encode(&self) -> Vec<u8> {
        (
            self.blob_headers_root,
            self.quorum_numbers.clone(),
            self.signed_stake_for_quorums.clone(),
            self.reference_block_number,
        )
            .abi_encode()
    }
}

/// Batch metadata

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchMetadata {
    /// The batch header
    pub batch_header: BatchHeader,
    /// The signatory record hash
    pub signatory_record_hash: B256,
    /// The confirmation block number
    pub confirmation_block_number: u32,
}

impl BatchMetadata {
    /// Encode the batch metadata
    pub fn encode(&self) -> Vec<u8> {
        (self.batch_header.encode(), self.signatory_record_hash, self.confirmation_block_number)
            .abi_encode()
    }
}

/// Blob verification proof
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BlobVerificationProof {
    /// The batch ID
    pub batch_id: u32,
    /// The blob index
    pub blob_index: u32,
    /// The batch metadata
    pub batch_metadata: BatchMetadata,
    /// The inclusion proof
    pub inclusion_proof: Bytes,
    /// The quorum indices
    pub quorum_indices: Bytes,
}

impl BlobVerificationProof {
    /// Encode the blob verification proof
    pub fn encode(&self) -> Vec<u8> {
        (
            self.batch_id,
            self.blob_index,
            self.batch_metadata.encode(),
            self.inclusion_proof.clone(),
            self.quorum_indices.clone(),
        )
            .abi_encode()
    }
}

/// EigenDA cert
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EigenDACert {
    /// The blob verification proof
    pub blob_verification_proof: BlobVerificationProof,
    /// The blob header
    pub blob_header: BlobHeader,
}

impl EigenDACert {
    /// Encode the EigenDA cert
    pub fn encode(&self) -> Vec<u8> {
        (self.blob_verification_proof.encode(), self.blob_header.encode()).abi_encode()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use alloy::primitives::B256;

//     #[test]
//     fn test_eigen_da_cert_encode() {
//         let cert = EigenDACert {
//             blob_verification_proof: BlobVerificationProof {
//                 batch_id: 1,
//                 blob_index: 2,
//                 batch_metadata: BatchMetadata {
//                     batch_header: BatchHeader {
//                         blob_headers_root: B256::ZERO,
//                         quorum_numbers: Bytes::from(vec![1, 2, 3]),
//                         signed_stake_for_quorums: Bytes::from(vec![4, 5, 6]),
//                         reference_block_number: 100,
//                     },
//                     signatory_record_hash: B256::ZERO,
//                     confirmation_block_number: 300,
//                 },
//                 inclusion_proof: Bytes::from(vec![1, 2, 3]),
//                 quorum_indices: Bytes::from(vec![4, 5, 6]),
//             },
//             blob_header: BlobHeader {
//                 commitment: G1Point { x: U256::ZERO, y: U256::ZERO },
//                 data_length: 100,
//                 quorum_blob_params: vec![QuorumBlobParam {
//                     quorum_number: 1,
//                     adversary_threshold_percentage: 2,
//                     confirmation_threshold_percentage: 3,
//                     chunk_length: U256::ZERO,
//                 }],
//             },
//         };

//         let encoded = cert.encode();
//         assert!(!encoded.is_empty());

//         // Verify the encoded bytes contain the expected data
//         assert_eq!(encoded.len(), 100);

//         // check the blob verification proof
//         assert_eq!(encoded[0..32], B256::ZERO);
//     }
// }

// BlobVerificationProof: 217322,136,0xf4a6148b02394957923a4fe4e54e23b88193c18bac3f2e3bf1333b1f7a013724,0x0001,0x6157,3920228,0xd18df624719d00e345b1b0797827cf4246576a391eb85b77b9b8f3db3b2f4f7c,3920313,0xf609605b079c9a369ff81b018e0201707b95dcb2652735301db3201325bee4a374eeaa19e5d55af0cdc1103fda3ebe814bbff3df4ddc5346d2bdf71dc2ddc9ecb3614c5c4f28a7a6b90459130fd59809c5036796abeaf06e34bbb6cf9b3d45f80552e37f59a2bcfad73f79f3c91a2a66bbbf2ec0716fa3b2ee6d198361005bc8699202e75aa170163f15107b30fdfd1612bcad04cdfba6441ead6331e2515e399cb9166f1b1c4d3a0bc1bcfe19bc5ddedc7024f63734d1d1ba6b209139cb41be8f8984bb6d48eeaca9506962198807869d73836a18a42c1016ea161e5fb50af87919e7fa34b14376456c043cfaf9a17dbd0f9dbbc31f34a8c941c74e9fd98193,0x0001
// TupleblobHeader: 20680959615941354456587851604445044014299795839741462087745306074026936801870,1812449331673735135042324570242215948056750582394379364467175998965308373993,64,0,33,55,1,1,33,55,4
