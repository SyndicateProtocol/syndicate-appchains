//! Compression utils

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
/// `ZLib` compression with CM8   
pub const ZLIB_CM8: u8 = 0x08;
/// `ZLib` compression with CM15
pub const ZLIB_CM15: u8 = 0x0F;

/// Gets the compression type based on a version byte
pub const fn get_compression_type(version_byte: u8) -> CompressionType {
    match version_byte {
        NO_COMPRESSION => CompressionType::None,
        ZLIB_CM8 | ZLIB_CM15 => CompressionType::Zlib,
        _ => CompressionType::Unknown,
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
