//! Data generators for benchmark tests

use alloy::primitives::{Address, Bytes, TxHash};
use rand::Rng;

/// Different sizes for generated test data
#[derive(Debug, Clone, Copy)]
pub enum DataSize {
    Small,  // ~1KB
    Medium, // ~10KB
    Large,  // ~100KB
    XLarge, // ~1MB
}

impl DataSize {
    pub const fn bytes(self) -> usize {
        match self {
            Self::Small => 1024,
            Self::Medium => 10 * 1024,
            Self::Large => 100 * 1024,
            Self::XLarge => 1024 * 1024,
        }
    }
}

/// Generate test transactions of varying sizes
pub fn generate_test_transactions(count: usize) -> Vec<Bytes> {
    let mut rng = rand::rng();
    (0..count)
        .map(|_| {
            let size = rng.random_range(100..2000); // Realistic transaction sizes
            generate_random_bytes(size)
        })
        .collect()
}

/// Generate transactions with specific total data size
pub fn generate_transactions_with_size(total_size: usize) -> Vec<Bytes> {
    let avg_tx_size = 500; // Average transaction size
    let count = total_size / avg_tx_size;
    generate_test_transactions(count)
}

/// Generate random bytes of specified length
pub fn generate_random_bytes(length: usize) -> Bytes {
    let mut rng = rand::rng();
    let data: Vec<u8> = (0..length).map(|_| rng.random()).collect();
    Bytes::from(data)
}

/// Generate test data for hashing operations
pub fn generate_hash_data(count: usize) -> Vec<Vec<u8>> {
    let mut rng = rand::rng();
    (0..count)
        .map(|_| {
            let size = rng.random_range(32..1024);
            (0..size).map(|_| rng.random::<u8>()).collect()
        })
        .collect()
}

/// Generate a realistic block for testing
pub fn generate_test_block(size: DataSize) -> TestBlock {
    let mut rng = rand::rng();
    let tx_count = size.bytes() / 500; // Approximate transactions per block

    TestBlock {
        hash: generate_random_hash(),
        parent_hash: generate_random_hash(),
        number: rng.random_range(1..1_000_000),
        timestamp: rng.random_range(1_600_000_000..1_700_000_000),
        transactions: generate_test_transactions(tx_count),
        state_root: generate_random_hash(),
        receipts_root: generate_random_hash(),
    }
}

/// Generate a random transaction hash
pub fn generate_random_hash() -> TxHash {
    let mut rng = rand::rng();
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);
    TxHash::from(bytes)
}

/// Generate a random address
pub fn generate_random_address() -> Address {
    let mut rng = rand::rng();
    let mut bytes = [0u8; 20];
    rng.fill(&mut bytes);
    Address::from(bytes)
}

/// Test block structure for benchmarking
#[derive(Debug, Clone)]
pub struct TestBlock {
    pub hash: TxHash,
    pub parent_hash: TxHash,
    pub number: u64,
    pub timestamp: u64,
    pub transactions: Vec<Bytes>,
    pub state_root: TxHash,
    pub receipts_root: TxHash,
}

/// Generate test signature data
pub fn generate_test_signature() -> (Vec<u8>, Vec<u8>) {
    let signature = generate_random_bytes(65).to_vec(); // ECDSA signature
    let message = generate_random_bytes(32).to_vec(); // Message hash
    (signature, message)
}

/// Generate test batch data for database operations
pub fn generate_test_batch_data(size: usize) -> TestBatchData {
    let mut rng = rand::rng();

    TestBatchData {
        transactions: generate_test_transactions(size),
        block_hashes: (0..size).map(|_| generate_random_hash()).collect(),
        timestamps: (0..size).map(|_| rng.random_range(1_600_000_000..1_700_000_000)).collect(),
        state_data: (0..size).map(|_| generate_random_bytes(rng.random_range(100..1000))).collect(),
    }
}

/// Test batch data structure
#[derive(Debug, Clone)]
pub struct TestBatchData {
    pub transactions: Vec<Bytes>,
    pub block_hashes: Vec<TxHash>,
    pub timestamps: Vec<u64>,
    pub state_data: Vec<Bytes>,
}
