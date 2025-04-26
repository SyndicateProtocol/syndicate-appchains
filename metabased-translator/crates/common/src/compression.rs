//! Compression utils

use shared::zlib_compression::{CM_BITS_MASK, ZLIB_CM15, ZLIB_CM8};

/// Defines the compression types supported by the metabased translator
#[derive(Debug, PartialEq, Eq)]
pub enum CompressionType {
    /// No compression
    None,
    /// Unknown compression
    Unknown,
    /// `ZLib` compression
    Zlib,
}
/// No compression is marked with the 0 byte
pub const NO_COMPRESSION: u8 = 0x0;

/// Gets the compression type based on a version byte
pub const fn get_compression_type(version_byte: u8) -> CompressionType {
    if version_byte == NO_COMPRESSION {
        CompressionType::None
    } else if version_byte & CM_BITS_MASK == ZLIB_CM8 || version_byte & CM_BITS_MASK == ZLIB_CM15 {
        CompressionType::Zlib
    } else {
        CompressionType::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_compression_type() {
        assert_eq!(get_compression_type(NO_COMPRESSION), CompressionType::None);
        assert_eq!(get_compression_type(0xFF), CompressionType::Unknown);
    }
}
