//! Types for the `EigenDA`

use alloy::sol;
use serde::{Deserialize, Serialize};

// From `EigenDA` contract:
// https://github.com/Layr-Labs/eigenda/blob/4d1994e318ff6898cfd500270d8881e18dc021bf/contracts/src/core/libraries/v1/EigenDATypesV1.sol
sol! {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct G1Point {
        uint256 X;
        uint256 Y;
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct VersionedBlobParams {
        uint32 maxNumOperators;
        uint32 numChunks;
        uint8 codingRate;
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct SecurityThresholds {
        uint8 confirmationThreshold;
        uint8 adversaryThreshold;
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct QuorumBlobParam {
        uint8 quorumNumber;
        uint8 adversaryThresholdPercentage;
        uint8 confirmationThresholdPercentage;
        uint32 chunkLength;
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct BlobHeader {
        G1Point commitment;
        uint32 dataLength;
        QuorumBlobParam[] quorumBlobParams;
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct ReducedBatchHeader {
        bytes32 blobHeadersRoot;
        uint32 referenceBlockNumber;
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct BatchHeader {
        bytes32 blobHeadersRoot;
        bytes quorumNumbers;
        bytes signedStakeForQuorums;
        uint32 referenceBlockNumber;
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct BatchMetadata {
        BatchHeader batchHeader;
        bytes32 signatoryRecordHash;
        uint32 confirmationBlockNumber;
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct BlobVerificationProof {
        uint32 batchId;
        uint32 blobIndex;
        BatchMetadata batchMetadata;
        bytes inclusionProof;
        bytes quorumIndices;
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct EigenDACert {
        BlobVerificationProof blobVerificationProof;
        BlobHeader blobHeader;
    }
}

/// `EigenDA` message header flag
/// <https://github.com/Layr-Labs/nitro-contracts/blob/278fdbc39089fa86330f0c23f0a05aee61972c84/src/bridge/SequencerInbox.sol#L133>
pub const EIGENDA_MESSAGE_HEADER_FLAG: u8 = 0xed;

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        primitives::{bytes, fixed_bytes, Bytes, U256},
        sol_types::SolValue,
    };
    use std::str::FromStr;

    #[test]
    fn test_eigen_da_cert_encode() {
        // Using data from real batch from Exo:
        // <https://holesky.etherscan.io/inputdatadecoder?tx=0xab80c93423f60da385f0b3f4ca1a162aafd922aa0aa3ce031d4e37d0f729c1a9>

        let cert = EigenDACert {
            blobVerificationProof: BlobVerificationProof {
                batchId: 217322,
                blobIndex: 136,
                batchMetadata: BatchMetadata {
                    batchHeader: BatchHeader {
                        blobHeadersRoot: fixed_bytes!("0xf4a6148b02394957923a4fe4e54e23b88193c18bac3f2e3bf1333b1f7a013724"),
                        quorumNumbers: bytes!("0x0001"),
                        signedStakeForQuorums: bytes!("0x6157"),
                        referenceBlockNumber: 3920228,
                    },
                    signatoryRecordHash: fixed_bytes!("0xd18df624719d00e345b1b0797827cf4246576a391eb85b77b9b8f3db3b2f4f7c"),
                    confirmationBlockNumber: 3920313,
                },
                inclusionProof: bytes!("0xf609605b079c9a369ff81b018e0201707b95dcb2652735301db3201325bee4a374eeaa19e5d55af0cdc1103fda3ebe814bbff3df4ddc5346d2bdf71dc2ddc9ecb3614c5c4f28a7a6b90459130fd59809c5036796abeaf06e34bbb6cf9b3d45f80552e37f59a2bcfad73f79f3c91a2a66bbbf2ec0716fa3b2ee6d198361005bc8699202e75aa170163f15107b30fdfd1612bcad04cdfba6441ead6331e2515e399cb9166f1b1c4d3a0bc1bcfe19bc5ddedc7024f63734d1d1ba6b209139cb41be8f8984bb6d48eeaca9506962198807869d73836a18a42c1016ea161e5fb50af87919e7fa34b14376456c043cfaf9a17dbd0f9dbbc31f34a8c941c74e9fd98193"),
                quorumIndices: bytes!("0x0001"),
            },
            blobHeader: BlobHeader {
                commitment: G1Point {
                    X: U256::from_str("20680959615941354456587851604445044014299795839741462087745306074026936801870").unwrap(),
                    Y: U256::from_str("1812449331673735135042324570242215948056750582394379364467175998965308373993").unwrap(),
                },
                dataLength: 64,
                quorumBlobParams: vec![
                    QuorumBlobParam {
                        quorumNumber: 0,
                        adversaryThresholdPercentage: 33,
                        confirmationThresholdPercentage: 55,
                        chunkLength: 1,
                    },
                    QuorumBlobParam {
                        quorumNumber: 1,
                        adversaryThresholdPercentage: 33,
                        confirmationThresholdPercentage: 55,
                        chunkLength: 4,
                    }
                ],
            },
        };

        let correct_encoded = bytes!("0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000003a000000000000000000000000000000000000000000000000000000000000350ea000000000000000000000000000000000000000000000000000000000000008800000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000003200000000000000000000000000000000000000000000000000000000000000060d18df624719d00e345b1b0797827cf4246576a391eb85b77b9b8f3db3b2f4f7c00000000000000000000000000000000000000000000000000000000003bd1b9f4a6148b02394957923a4fe4e54e23b88193c18bac3f2e3bf1333b1f7a013724000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000003bd16400000000000000000000000000000000000000000000000000000000000000020001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000261570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100f609605b079c9a369ff81b018e0201707b95dcb2652735301db3201325bee4a374eeaa19e5d55af0cdc1103fda3ebe814bbff3df4ddc5346d2bdf71dc2ddc9ecb3614c5c4f28a7a6b90459130fd59809c5036796abeaf06e34bbb6cf9b3d45f80552e37f59a2bcfad73f79f3c91a2a66bbbf2ec0716fa3b2ee6d198361005bc8699202e75aa170163f15107b30fdfd1612bcad04cdfba6441ead6331e2515e399cb9166f1b1c4d3a0bc1bcfe19bc5ddedc7024f63734d1d1ba6b209139cb41be8f8984bb6d48eeaca9506962198807869d73836a18a42c1016ea161e5fb50af87919e7fa34b14376456c043cfaf9a17dbd0f9dbbc31f34a8c941c74e9fd98193000000000000000000000000000000000000000000000000000000000000000200010000000000000000000000000000000000000000000000000000000000002db90222187e61d6bea8fadb67459b5869f5beb80836fa419bf86e5f52ee9a4e0401cf5a16aa3ef193c3e4f736ee4472f08e1e9cfe04d90287e0da1731f6bbe900000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000021000000000000000000000000000000000000000000000000000000000000003700000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002100000000000000000000000000000000000000000000000000000000000000370000000000000000000000000000000000000000000000000000000000000004");
        let encoded = cert.abi_encode();

        assert_eq!(Bytes::from(encoded), correct_encoded);
    }
}
