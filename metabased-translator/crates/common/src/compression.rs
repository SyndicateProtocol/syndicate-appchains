//! Compression utils

/// Defines the compression types supported by the metabased translator
#[derive(Debug, PartialEq, Eq)]
pub enum CompressionType {
    /// No compression
    None,
    /// Unknown compression
    Unknown,
}
/// No compression is marked with the 0 byte
pub const NO_COMPRESSION: u8 = 0x0;

/// Gets the compression type based on a version byte
pub const fn get_compression_type(version_byte: u8) -> CompressionType {
    match version_byte {
        NO_COMPRESSION => CompressionType::None,
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
