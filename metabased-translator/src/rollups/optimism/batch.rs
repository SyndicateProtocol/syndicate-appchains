use alloy_primitives::B256; // 32-byte hash
use alloy_rlp::Encodable;
use flate2::{write::ZlibEncoder, Compression}; // Zlib compression
use std::error::Error;
use std::io::Write;

// Constants
const BATCH_VERSION_BYTE: u8 = 0x01;

// Define the Batch struct
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Batch {
    pub parent_hash: B256,
    pub epoch_number: u64,
    pub epoch_hash: B256,
    pub timestamp: u64,
    pub transaction_list: Vec<Vec<u8>>,
}

// Frame struct to represent the framed data
#[derive(Debug, PartialEq, Eq)]
pub struct Frame {
    pub id: B256,
    pub frame_num: u16,
    pub data: Vec<u8>,
    pub is_last: bool,
}

// Implementation for the Batch struct
impl Batch {
    /// Encodes the Batch to RLP bytes
    pub fn encode(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = Vec::new();

        // Write the version byte
        buf.push(BATCH_VERSION_BYTE);

        // RLP encode fields
        self.parent_hash.encode(&mut buf);
        self.epoch_number.encode(&mut buf);
        self.epoch_hash.encode(&mut buf);
        self.timestamp.encode(&mut buf);
        self.transaction_list.encode(&mut buf);

        Ok(buf)
    }

    /// Splits the Batch into frames of a given size
    pub fn get_frames(&self, frame_size: usize) -> Result<Vec<Frame>, Box<dyn Error>> {
        // Step 1: Encode the Batch
        let encoded_batch = self.encode()?;

        // Step 2: Compress using zlib
        let compressed_channel = to_channel(&encoded_batch)?;

        // Step 3: Split into frames
        let frames = to_frames(&compressed_channel, frame_size, self.epoch_hash)?;
        Ok(frames)
    }
}

/// Compresses the batch data using zlib (no compression)
pub fn to_channel(batch: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buf = Vec::new();
    let mut encoder = ZlibEncoder::new(&mut buf, Compression::none());
    encoder.write_all(batch)?;
    encoder.finish()?;
    Ok(buf)
}

/// Splits the compressed channel into frames
pub fn to_frames(
    channel: &[u8],
    frame_size: usize,
    block_hash: B256,
) -> Result<Vec<Frame>, Box<dyn Error>> {
    let num_frames = (channel.len() + frame_size - 1) / frame_size;
    let mut frames = Vec::with_capacity(num_frames);

    // Generate ChannelID (here it's the block hash)
    let id = block_hash;

    let mut frame_num: u16 = 0;
    for chunk in channel.chunks(frame_size) {
        let is_last = frame_num as usize == num_frames - 1;
        frames.push(Frame {
            id,
            frame_num,
            data: chunk.to_vec(),
            is_last,
        });
        frame_num += 1;
    }
    Ok(frames)
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::hex::FromHex;

    fn sample_batch() -> Batch {
        let parent_hash =
            B256::from_hex("1111111111111111111111111111111111111111111111111111111111111111")
                .unwrap();
        let epoch_hash =
            B256::from_hex("2222222222222222222222222222222222222222222222222222222222222222")
                .unwrap();
        let transaction_list = vec![b"transaction_1".to_vec(), b"transaction_2".to_vec()];

        Batch {
            parent_hash,
            epoch_number: 42,
            epoch_hash,
            timestamp: 1712500000,
            transaction_list,
        }
    }

    #[test]
    fn test_batch_encoding() {
        let batch = sample_batch();
        let encoded = batch.encode().expect("Batch encoding failed");

        // Ensure encoded batch starts with version byte
        assert_eq!(encoded[0], BATCH_VERSION_BYTE);
        assert!(
            encoded.len() > 1,
            "Encoded batch should have data beyond version byte"
        );
    }

    #[test]
    fn test_channel_compression() {
        let batch = sample_batch();
        let encoded = batch.encode().expect("Batch encoding failed");

        let compressed = to_channel(&encoded).expect("Compression failed");
        assert!(compressed.len() > 0, "Compressed data should not be empty");
    }

    #[test]
    fn test_frame_splitting() {
        let batch = sample_batch();
        let frames = batch.get_frames(16).expect("Frame splitting failed");

        assert!(!frames.is_empty(), "Frames should not be empty");
        assert_eq!(
            frames.last().unwrap().is_last,
            true,
            "Last frame should be marked as is_last"
        );

        // Validate frame content
        for (i, frame) in frames.iter().enumerate() {
            assert_eq!(
                frame.frame_num as usize, i,
                "Frame numbers should increment sequentially"
            );
        }
    }
}
