use crate::rollups::optimism::frame::Frame;
use alloy_primitives::{Address, Bytes, B256};
use alloy_rlp::{Buf, Decodable, Encodable, Error as RlpError};
use alloy_rpc_types::{TransactionInput, TransactionRequest};
use flate2::write::ZlibEncoder;
use flate2::Compression;

use std::error::Error;
use std::io::Write;
use std::ops::Div;
use tracing::info;

use eyre::Result;

// Constants
const BATCH_VERSION_BYTE: u8 = 0x00;
// Define the Batch struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Batch {
    /// Block hash of the previous L2 block
    pub parent_hash: B256,
    /// The batch epoch number. Same as the first L1 block number in the epoch.
    pub epoch_num: u64,
    /// The block hash of the first L1 block in the epoch
    pub epoch_hash: B256,
    /// The L2 block timestamp of this batch
    pub timestamp: u64,
    /// The L2 block transactions in this batch
    pub transactions: Vec<Vec<u8>>,
}

// Implementation for the Batch struct
impl Batch {
    /// Encode the `Batch` into RLP
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();

        // Step 1: Add the version byte
        out.push(BATCH_VERSION_BYTE);

        // Step 2: Encode fields as a list
        let header = alloy_rlp::Header {
            list: true,
            payload_length: self.parent_hash.length()
                + self.epoch_num.length()
                + self.epoch_hash.length()
                + self.timestamp.length()
                + self.transactions.length(),
        };
        header.encode(&mut out);

        // Step 3: Encode fields
        self.parent_hash.encode(&mut out);
        self.epoch_num.encode(&mut out);
        self.epoch_hash.encode(&mut out);
        self.timestamp.encode(&mut out);
        self.transactions.encode(&mut out);

        out
    }

    pub fn decode(encoded: &[u8]) -> Result<Self, RlpError> {
        let mut buf = encoded;

        // Step 1: Consume the version byte
        let version_byte = buf.get_u8();
        if version_byte != BATCH_VERSION_BYTE {
            // TODO: Use discrete error
            return Err(RlpError::Custom("Invalid version byte for Batch"));
        }

        // Step 2: Decode as a list
        let header = alloy_rlp::Header::decode(&mut buf)?;
        if !header.list {
            // TODO: Use discrete error
            return Err(RlpError::Custom("Batch must be an RLP list"));
        }

        // Step 3: Decode individual fields
        let parent_hash = B256::decode(&mut buf)?;
        let epoch_num = u64::decode(&mut buf)?;
        let epoch_hash = B256::decode(&mut buf)?;
        let timestamp = u64::decode(&mut buf)?;
        let transactions = Vec::<Vec<u8>>::decode(&mut buf)?;

        // Return the decoded Batch
        Ok(Batch {
            parent_hash,
            epoch_num,
            epoch_hash,
            timestamp,
            transactions,
        })
    }

    /// Splits the Batch into frames of a given size
    pub fn get_frames(&self, frame_size: usize) -> Result<Vec<Frame>, Box<dyn Error>> {
        // Step 1: Encode the Batch
        let encoded_batch = self.encode();
        info!("encoded_batch : {:?}", encoded_batch);

        // Step 2: Compress using zlib
        let compressed_channel = to_channel(&encoded_batch)?;
        info!("channel: {:?}", compressed_channel);

        // Step 3: Split into frames
        let frames = to_frames(&compressed_channel, frame_size, self.epoch_hash)?;
        Ok(frames)
    }
}

/// Compresses the batch data using zlib (no compression)
fn to_channel(batch: &[u8]) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut encoder = ZlibEncoder::new(&mut buf, Compression::none());
    encoder.write_all(batch)?;
    encoder.finish()?;
    Ok(buf)
}

fn to_frames(channel: &[u8], frame_size: usize, block_hash: B256) -> Result<Vec<Frame>> {
    let num_frames = (channel.len() + frame_size - 1).div(frame_size);
    let mut frames = Vec::with_capacity(num_frames);

    let id = block_hash;

    for (frame_num, chunk) in channel.chunks(frame_size).enumerate() {
        let is_last = frame_num == num_frames - 1;
        frames.push(Frame {
            id,
            frame_num: frame_num as u16,
            data: chunk.to_vec(),
            is_last,
        });
    }
    Ok(frames)
}

pub fn new_batcher_tx(from: Address, to: Address, data: Bytes) -> TransactionRequest {
    let input = TransactionInput::new(data);
    TransactionRequest::default().from(from).to(to).input(input)
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, Bytes};

    fn sample_batch() -> Batch {
        Batch {
            parent_hash: B256::repeat_byte(0x11),
            epoch_num: 42,
            epoch_hash: B256::repeat_byte(0x22),
            timestamp: 1712500000,
            transactions: vec![b"tx1".to_vec(), b"tx2".to_vec()],
        }
    }

    #[test]
    fn test_batch_encoding() {
        let batch = sample_batch();
        let encoded = batch.encode();

        // Ensure encoded batch starts with version byte
        assert_eq!(encoded[0], BATCH_VERSION_BYTE);
        assert!(
            encoded.len() > 1,
            "Encoded batch should have data beyond version byte"
        );
    }

    #[test]
    fn test_encode_decode() -> Result<(), RlpError> {
        let batch = sample_batch();

        // Encode and decode the batch
        let encoded = batch.encode();
        let decoded = Batch::decode(&encoded)?;

        // Verify batch matches original
        assert_eq!(batch, decoded);
        Ok(())
    }

    #[test]
    fn test_channel_compression() {
        let batch = sample_batch();
        let encoded = batch.encode();

        let compressed = to_channel(&encoded).expect("Compression failed");
        assert!(
            !compressed.is_empty(),
            "Compressed data should not be empty"
        );
    }

    #[test]
    fn test_frame_splitting() {
        let batch = sample_batch();
        let frames = batch.get_frames(16).expect("Frame splitting failed");

        assert!(!frames.is_empty(), "Frames should not be empty");
        assert!(
            frames.last().unwrap().is_last,
            "Last frame should be marked as is_last"
        );

        for (i, frame) in frames.iter().enumerate() {
            assert_eq!(
                frame.frame_num as usize, i,
                "Frame numbers should increment sequentially"
            );
        }
    }

    #[test]
    fn test_to_frames_small_chunk() {
        let data = vec![0xAA; 8]; // Small data
        let block_hash = B256::repeat_byte(0x55);
        let frames = to_frames(&data, 4, block_hash).expect("Frame creation failed");

        assert_eq!(frames.len(), 2, "Data should split into 2 frames");
        assert!(frames[1].is_last, "Second frame should be marked as last");
    }

    #[test]
    fn test_to_frames_exact_chunk() {
        let data = vec![0xBB; 16]; // Exact chunk size
        let block_hash = B256::repeat_byte(0x66);
        let frames = to_frames(&data, 16, block_hash).expect("Frame creation failed");

        assert_eq!(frames.len(), 1, "Data should fit exactly into one frame");
        assert!(frames[0].is_last, "Single frame should be marked as last");
    }

    #[test]
    fn test_new_batcher_tx() {
        let from = Address::repeat_byte(0x11);
        let to = Address::repeat_byte(0x22);
        let data = Bytes::from(b"batch-data".to_vec());

        let tx = new_batcher_tx(from, to, data.clone());
        assert_eq!(tx.from, Some(from), "From address should match");
        assert_eq!(tx.input, TransactionInput::new(data), "Data input mismatch");
    }

    #[test]
    fn test_invalid_version_decode() {
        let mut invalid_encoded = sample_batch().encode();
        invalid_encoded[0] = 0xFF; // Invalid version byte

        let result = Batch::decode(&invalid_encoded);
        assert!(
            result.is_err(),
            "Decoding should fail with invalid version byte"
        );
    }

    #[test]
    fn test_empty_frame_data() {
        let empty_data = vec![];
        let block_hash = B256::repeat_byte(0x33);

        let frames = to_frames(&empty_data, 16, block_hash).expect("Frame splitting failed");
        assert!(
            frames.is_empty(),
            "No frames should be generated for empty data"
        );
    }
}
