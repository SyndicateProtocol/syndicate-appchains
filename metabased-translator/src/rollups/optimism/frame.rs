use crate::rollups::optimism::batch::BATCH_VERSION_BYTE;
use alloy_primitives::B256;
use std::io::{self, Write};

// Frame struct to represent the framed data
#[derive(Debug, PartialEq, Eq)]
pub struct Frame {
    pub id: B256,
    pub frame_num: u16,
    pub data: Vec<u8>,
    pub is_last: bool,
}

impl Frame {
    // Marshal the Frame into a binary buffer
    pub fn marshal_binary<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.id.as_ref())?; // Write 32-byte id
        writer.write_all(&self.frame_num.to_be_bytes())?; // Write frame_num in big-endian
        writer.write_all(&[self.is_last as u8])?; // Write is_last as a single byte (0 or 1)
        writer.write_all(&self.data)?; // Write the data payload
        Ok(())
    }
}

// Function to serialize frames into a byte buffer
pub fn to_data(frames: &[Frame]) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    buf.push(BATCH_VERSION_BYTE); // Add version byte at the beginning

    for frame in frames {
        frame.marshal_binary(&mut buf)?; // Serialize each frame and append to buffer
    }

    Ok(buf)
}
