//! Frame encoding for Optimism batcher transactions
//!
//! This module provides functionality for encoding batches of transactions into frames
//! that can be submitted by the batcher. The frames are encoded with:
//! - A 16-byte frame ID derived from the block hash
//! - A frame number to order frames within a batch
//! - The frame payload data
//! - A flag indicating if this is the last frame
//!
//! The encoded frames are prefixed with a version byte and can be submitted as batcher
//! transactions.
use alloy::primitives::B128;
use std::io::{self, Write};

const BATCHER_TRANSACTION_VERSION_BYTE: u8 = 0x00;

/// Frame struct to represent the framed data
#[derive(Debug, PartialEq, Eq)]
pub struct Frame {
    /// 16-byte frame identifier derived from block hash
    pub id: B128,
    /// Frame sequence number within a batch
    pub frame_num: u16,
    /// Frame payload data
    pub data: Vec<u8>,
    /// Indicates if this is the last frame in the batch
    pub is_last: bool,
}

impl Frame {
    /// Marshal the Frame into a binary buffer
    pub fn marshal_binary<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(self.id.as_ref())?; // Write 32-byte id
        writer.write_all(&self.frame_num.to_be_bytes())?; // Write frame_num in big-endian
        writer.write_all(&(self.data.len() as u32).to_be_bytes())?; // Convert to u32 and write in big-endian
        writer.write_all(&self.data)?; // Write the data payload
        writer.write_all(&[self.is_last as u8])?; // Write is_last as a single byte (0 or 1)
        Ok(())
    }
}

/// Function to serialize frames into a byte buffer
pub fn to_data(frames: &[Frame]) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    buf.push(BATCHER_TRANSACTION_VERSION_BYTE); // Add version byte at the beginning

    for frame in frames {
        frame.marshal_binary(&mut buf)?; // Serialize each frame and append to buffer
    }

    Ok(buf)
}
