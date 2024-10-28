use std::convert::Infallible;
use bytes::{BytesMut};
use domain::primitives::Bytes;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{self, Write};
use crate::domain;
use flate2::read::ZlibDecoder;
use std::io::Read;

// Valid Zlib CM bits
const ZLIB_CM8: u8 = 0x08;
const ZLIB_CM15: u8 = 0x0F;
const CM_BITS_MASK: u8 = 0x0F;

/// Compresses a single Ethereum transaction using zlib compression
/// Ensures the CM bits are set to 8 (default for zlib)
pub fn compress_transaction(transaction: &[u8]) -> Result<Vec<u8>, IoError> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(transaction)?;
    let compressed = encoder.finish()?;

    // Verify the CM bits are correct (should be 8 by default with flate2)
    is_valid_cm_bits_8_only(&compressed)?;

    Ok(compressed)
}

/// Decompresses a single zlib compressed Ethereum transaction
pub fn decompress_transaction(compressed: &[u8]) -> Result<Vec<u8>, IoError> {
    is_valid_cm_bits_8_or_15(compressed)?;

    let mut decoder = ZlibDecoder::new(compressed);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;

    Ok(buffer)
}

#[derive(Debug)]
pub struct IoError(io::Error);

impl From<io::Error> for IoError {
    fn from(e: io::Error) -> Self {
        IoError(e)
    }
}

impl From<IoError> for Infallible {
    fn from(_: IoError) -> Self {
        unreachable!("Cannot instantiate infallible")
    }
}

/// Compresses a slice of Ethereum transactions using zlib compression
/// Each transaction is prefixed with its length to enable proper decompression
pub fn compress_transactions(transactions: &[Bytes]) -> Result<Bytes, IoError> {
    let mut buffer = BytesMut::new();

    // Write number of transactions (4 bytes)
    buffer.extend_from_slice(&(transactions.len() as u32).to_be_bytes());

    // Write each transaction with its length prefix
    for tx in transactions {
        buffer.extend_from_slice(&(tx.len() as u32).to_be_bytes());
        buffer.extend_from_slice(tx);
    }

    // Compress using zlib
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&buffer)?;
    let compressed = encoder.finish()?;

    is_valid_cm_bits_8_only(&compressed)?;

    Ok(Bytes::from(compressed))
}

/// Decompresses zlib compressed Ethereum transactions back into their original form
pub fn decompress_transactions(compressed: &Bytes) -> Result<Vec<Bytes>, IoError>{
    is_valid_cm_bits_8_or_15(compressed)?;

    let mut decoder = ZlibDecoder::new(&compressed[..]);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;

    let mut transactions = Vec::new();
    let mut pos = 0;

    // Read number of transactions
    if buffer.len() < 4 {
        return Err(IoError(io::Error::new(io::ErrorKind::InvalidData, "Invalid compressed data")));
    }
    let num_transactions = u32::from_be_bytes(buffer[0..4].try_into().unwrap());
    pos += 4;

    // Read each transaction
    for _ in 0..num_transactions {
        if pos + 4 > buffer.len() {
            return Err(IoError(io::Error::new(io::ErrorKind::InvalidData,  "Truncated data")));
        }

        let tx_len = u32::from_be_bytes(buffer[pos..pos+4].try_into().unwrap()) as usize;
        pos += 4;

        if pos + tx_len > buffer.len() {
            return Err(IoError(io::Error::new(io::ErrorKind::InvalidData,  "Truncated transaction")));
        }

        transactions.push(Bytes::copy_from_slice(&buffer[pos..pos+tx_len]));
        pos += tx_len;
    }

    Ok(transactions)
}

/// Validates CM bits in the zlib header against allowed values
fn validate_cm_bits<T: AsRef<[u8]>>(compressed: T, allowed_values: &[u8], error_msg: &str) -> Result<(), IoError> {
    let compressed = compressed.as_ref();

    // Check for empty data first
    if compressed.is_empty() {
        return Err(IoError(io::Error::new(io::ErrorKind::InvalidData, "Empty compressed data")))
    }

    // Extract CM bits and check against allowed values
    let cm_bits = compressed[0] & CM_BITS_MASK;
    if !allowed_values.contains(&cm_bits) {
        return Err(IoError(io::Error::new(io::ErrorKind::InvalidData, error_msg)))
    }

    Ok(())
}

/// Validates that CM bits are exactly 8
fn is_valid_cm_bits_8_only<T: AsRef<[u8]>>(compressed: T) -> Result<(), IoError> {
    validate_cm_bits(
        compressed,
        &[ZLIB_CM8],
        "Invalid CM bits in compressed data, expected ZLIB_CM8"
    )
}

/// Validates that CM bits are either 8 or 15
fn is_valid_cm_bits_8_or_15<T: AsRef<[u8]>>(compressed: T) -> Result<(), IoError>  {
    validate_cm_bits(
        compressed,
        &[ZLIB_CM8, ZLIB_CM15],
        "Invalid CM bits in compressed data, expected ZLIB_CM8 or ZLIB_CM15"
    )
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;
    use alloy_primitives::hex_literal::hex;

    #[test]
    fn test_valid_cm_bits() {
        // Test CM8
        let valid_cm8 = vec![0x08];  // CM bits = 8
        assert!(is_valid_cm_bits_8_only(&valid_cm8).is_ok());
        assert!(is_valid_cm_bits_8_or_15(valid_cm8).is_ok());

        // Test CM15
        let valid_cm15 = vec![0x0F];  // CM bits = 15
        assert!(is_valid_cm_bits_8_only(&valid_cm15).is_err());
        assert!(is_valid_cm_bits_8_or_15(valid_cm15).is_ok());

        // Test with compression info bits set
        let valid_with_info = vec![0x78];  // CM bits = 8, info bits = 7
        assert!(is_valid_cm_bits_8_only(valid_with_info).is_ok());
    }

    #[test]
    fn test_invalid_cm_bits() {
        // Test empty data
        let empty: Vec<u8> = vec![];
        assert!(is_valid_cm_bits_8_only(empty).is_err());

        // Test invalid CM bits
        let invalid = vec![0x01];  // CM bits = 1
        assert!(is_valid_cm_bits_8_only(&invalid).is_err());
        assert!(is_valid_cm_bits_8_or_15(invalid).is_err());
    }

    // Sample txn - from send_raw_transaction() on Latitude
    const SAMPLE_TX_1: [u8; 110] = hex!("02f86b83014a3407830f4240830f443e825208944e527486594696a7607ff3379e21746689a3fd6d1480c080a0502ec1e72aa5d8e52f2547c3dcb973d6129364828ea54cfd166ea74350a60cd4a02db70ba79cfb18a45d6b415e58aed8947bb66efc1156c2067e59d4ea5c69cfcb");

    // Pulled random txns from Base Sepolia explorer
    // https://sepolia.basescan.org/tx/0x517f3cda3ec255651839794d633c54843cb07ee54d18dfd6a7797a1d96ec4ffe
    const SAMPLE_TX_2: [u8; 132] = hex!("cdb554ea000000000000000000000000b8b904c73d2fb4d8c173298a51c27fab70222c320000000000000000000000000000000000000000000000000000000000568936000000000000000000000000b8b904c73d2fb4d8c173298a51c27fab70222c32000000000000000000000000000000000000000000000000000000000059bd0d");
    // https://sepolia.basescan.org/tx/0xdcef8686e15d1e163482d27658b4f1260a8dc07a7e5f6bce81df27a8f2127b81
    const SAMPLE_TX_3: [u8; 68] = hex!("39509351000000000000000000000000dd2da9ba748722faea8629a215ea47dd15e852f90000000000000000000000000000000000000000000000000429d069189e0000");
    // https://sepolia.basescan.org/tx/0x5de957de7b67999cc14099b7b40919afb0592de64c20a658c6cd296624b34ba9
    const SAMPLE_TX_4: [u8; 132] = hex!("81813c8b0000000000000000000000000000000000000000000000000000000001026afc0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000016345785d8a000000000000000000000000000000000000000000000000000000000000671834d8");

    #[test]
    fn test_single_tx_compression() {
        // Test valid single transaction compression and decompression
        let start = Instant::now();
        let input = &SAMPLE_TX_1;
        let compressed = compress_transaction(input).unwrap();
        let compress_time = start.elapsed();
        let start = Instant::now();
        let decompressed = decompress_transaction(&compressed).unwrap();
        let decompression_time = start.elapsed();

        assert_eq!(input[..], decompressed[..]);

        // Verify CM bits
        assert_eq!(compressed[0] & CM_BITS_MASK, ZLIB_CM8);

        // Check compression ratio
        let ratio = (1.0 - (compressed.len() as f64 / input.len() as f64)) * 100.0;
        println!("Single TX (n=1) compression ratio: {:.2}%", ratio);
        println!("Original size: {} bytes", input.len());
        println!("Compressed size: {} bytes", compressed.len());
        println!("Decompression size: {} bytes", decompressed.len());
        println!("Compression time: {:?}", compress_time);
        println!("Decompression time: {:?}", decompression_time);
        println!();
    }

    #[test]
    fn test_single_tx_empty() {
        // Test compression of empty transaction
        let empty_tx: [u8; 0] = [];
        let compressed = compress_transaction(&empty_tx).unwrap();
        let decompressed = decompress_transaction(&compressed).unwrap();

        assert_eq!(&empty_tx[..], &decompressed[..]);
        assert_eq!(compressed[0] & CM_BITS_MASK, ZLIB_CM8); // Verify CM bits
    }

    #[test]
    fn test_single_tx_invalid_compressed() {
        // Test various invalid compressed data scenarios

        // Test 1: Empty compressed data
        let empty_compressed: Vec<u8> = vec![];
        assert!(decompress_transaction(&empty_compressed).is_err());

        // Test 2: Invalid CM bits
        let mut invalid_cm = compress_transaction(&SAMPLE_TX_1).unwrap();
        invalid_cm[0] = invalid_cm[0] & 0xF0 | 0x01; // Set invalid CM bits
        assert!(decompress_transaction(&invalid_cm).is_err());

        // Test 3: Truncated compressed data
        let truncated = &compress_transaction(&SAMPLE_TX_1).unwrap()[0..5];
        assert!(decompress_transaction(truncated).is_err());

        // Test 4: Random invalid data
        let random_data = vec![1, 2, 3, 4, 5];
        assert!(decompress_transaction(&random_data).is_err());
    }

    #[test]
    fn test_batch_empty_tx() {
        // Test empty transaction list
        let empty_txs: Vec<Bytes> = vec![];
        let compressed = compress_transactions(&empty_txs).unwrap();
        let decompressed = decompress_transactions(&compressed).unwrap();

        assert_eq!(empty_txs, decompressed);
        assert_eq!(compressed[0] & CM_BITS_MASK, ZLIB_CM8); // Verify CM bits
    }

    #[test]
    fn test_batch_single_tx() {
        // Test batch compression with single transaction
        let txs = vec![Bytes::copy_from_slice(&SAMPLE_TX_1)];

        let compressed = compress_transactions(&txs).unwrap();
        let decompressed = decompress_transactions(&compressed).unwrap();

        assert_eq!(txs, decompressed);
        assert_eq!(compressed[0] & CM_BITS_MASK, ZLIB_CM8); // Verify CM bits
    }

    #[test]
    fn test_batch_multiple_tx() {
        // Test multiple transactions of varying sizes
        let txs = vec![
            Bytes::copy_from_slice(&SAMPLE_TX_1),
            Bytes::copy_from_slice(&SAMPLE_TX_2[0..50]), // Partial TX
            Bytes::copy_from_slice(&[1, 2, 3, 4, 5]), // Small TX
            Bytes::copy_from_slice(&SAMPLE_TX_3),
            Bytes::copy_from_slice(&SAMPLE_TX_4),
        ];

        let start = Instant::now();
        let compressed = compress_transactions(&txs).unwrap();
        let compress_time = start.elapsed();
        let start = Instant::now();
        let decompressed = decompress_transactions(&compressed).unwrap();
        let decompression_time = start.elapsed();

        assert_eq!(txs, decompressed);
        assert_eq!(compressed[0] & CM_BITS_MASK, ZLIB_CM8); // Verify CM bits


        // Check compression ratio
        let original_size: usize = txs.iter().map(|tx| tx.len()).sum();
        let compressed_size: usize = compressed.len();
        let decompressed_size: usize = decompressed.iter().map(|tx| tx.len()).sum();
        let ratio = (1.0 - (compressed_size as f64 / original_size as f64)) * 100.0;
        println!("Multiple TX (n={}) compression ratio: {:.2}%", txs.len(), ratio);
        println!("Original size: {} bytes", original_size);
        println!("Compressed size: {} bytes", compressed_size);
        println!("Decompressed size: {} bytes", decompressed_size);
        println!("Compression time: {:?}", compress_time);
        println!("Decompression time: {:?}", decompression_time);
        println!();
    }

    #[test]
    fn test_batch_invalid_compressed() {
        // Test various invalid batch compressed data scenarios

        // Test 1: Empty compressed data
        let empty_compressed = Bytes::new();
        assert!(decompress_transactions(&empty_compressed).is_err());

        // Test 2: Invalid CM bits
        let invalid_cm = compress_transactions(&[Bytes::copy_from_slice(&SAMPLE_TX_1)]).unwrap();
        let mut bytes = BytesMut::from(&invalid_cm[..]);
        bytes[0] = bytes[0] & 0xF0 | 0x01; // Set invalid CM bits
        let bytes_immutable = Bytes(bytes.freeze());
        assert!(decompress_transactions(&bytes_immutable).is_err());

        // Test 3: Truncated compressed data
        let truncated = Bytes::copy_from_slice(&compress_transactions(&[Bytes::copy_from_slice(&SAMPLE_TX_1)]).unwrap()[0..5]);
        assert!(decompress_transactions(&truncated).is_err());

        // Test 4: Valid zlib header but invalid content
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&[1, 2, 3]).unwrap(); // Write invalid length prefix
        let invalid_content = Bytes::from(encoder.finish().unwrap());
        assert!(decompress_transactions(&invalid_content).is_err());
    }

    #[test]
    fn test_large_batch() {
        // Test with a larger number of transactions
        let mut txs = Vec::new();
        // TODO (SEQ-240): improve test with many unique transactions, not 4 repeated ones
        for _ in 0..25 {
            txs.push(Bytes::copy_from_slice(&SAMPLE_TX_1));
            txs.push(Bytes::copy_from_slice(&SAMPLE_TX_2));
            txs.push(Bytes::copy_from_slice(&SAMPLE_TX_3));
            txs.push(Bytes::copy_from_slice(&SAMPLE_TX_4));
        }

        let start = Instant::now();
        let compressed = compress_transactions(&txs).unwrap();
        let compress_time = start.elapsed();
        let start = Instant::now();
        let decompressed = decompress_transactions(&compressed).unwrap();
        let decompression_time = start.elapsed();

        assert_eq!(txs, decompressed);
        assert_eq!(compressed[0] & CM_BITS_MASK, ZLIB_CM8); // Verify CM bits

        // Check compression ratio
        let original_size: usize = txs.iter().map(|tx| tx.len()).sum();
        let compressed_size: usize = compressed.len();
        let decompressed_size: usize = decompressed.iter().map(|tx| tx.len()).sum();
        let ratio = (1.0 - (compressed.len() as f64 / original_size as f64)) * 100.0;

        println!("Large batch (n={}) compression ratio: {:.2}%", txs.len(), ratio);
        println!("Original size: {} bytes", original_size);
        println!("Compressed size: {} bytes", compressed_size);
        println!("Decompressed size: {} bytes", decompressed_size);
        println!("Compression time: {:?}", compress_time);
        println!("Decompression time: {:?}", decompression_time);
        println!();
    }
}