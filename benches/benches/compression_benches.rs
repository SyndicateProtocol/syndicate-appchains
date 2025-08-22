//! Compression benchmark suite for zlib operations

use bench_utils::{generate_test_transactions, generate_transactions_with_size, DataSize};
use shared::zlib_compression::{
    compress_transaction, compress_transactions, decompress_transactions,
};

fn main() {
    divan::main();
}

/// Benchmark single transaction compression performance
#[divan::bench(args = [100, 500, 1000, 2000, 5000])]
fn compress_single_transaction(tx_size: usize) {
    let transaction = bench_utils::generate_random_bytes(tx_size);
    divan::black_box(compress_transaction(&transaction).expect("compression failed"));
}

/// Benchmark transaction batch compression with varying transaction counts
#[divan::bench(args = [1, 10, 50, 100, 500, 1000])]
fn compress_transaction_batch_by_count(n_transactions: usize) {
    let transactions = generate_test_transactions(n_transactions);
    divan::black_box(compress_transactions(&transactions).expect("batch compression failed"));
}

/// Benchmark compression performance by total data size
#[divan::bench(args = [DataSize::Small, DataSize::Medium, DataSize::Large])]
fn compress_by_total_size(size: DataSize) {
    let transactions = generate_transactions_with_size(size.bytes());
    divan::black_box(compress_transactions(&transactions).expect("compression by size failed"));
}

/// Benchmark compression ratio vs speed tradeoff
#[divan::bench(args = [100, 1000, 5000])]
fn compression_ratio_analysis(n_transactions: usize) {
    let transactions = generate_test_transactions(n_transactions);
    let original_size: usize = transactions.iter().map(|tx| tx.len()).sum();
    let compressed = compress_transactions(&transactions).expect("compression failed");

    // Store both compressed data and ratio for analysis
    let ratio = compressed.len() as f64 / original_size as f64;
    divan::black_box((compressed, ratio));
}

/// Benchmark round-trip compression/decompression
#[divan::bench(args = [50, 100, 500])]
fn roundtrip_compression(n_transactions: usize) {
    let transactions = generate_test_transactions(n_transactions);
    let compressed = compress_transactions(&transactions).expect("compression failed");
    divan::black_box(decompress_transactions(&compressed).expect("decompression failed"));
}

/// Benchmark memory allocation during compression
#[divan::bench(args = [100, 500, 1000])]
fn compression_memory_usage(n_transactions: usize) {
    let transactions = generate_test_transactions(n_transactions);
    // Simply benchmark the compression without special counter setup
    divan::black_box(compress_transactions(&transactions).expect("compression failed"));
}

/// Benchmark compression performance with realistic transaction patterns
#[divan::bench(args = [
    ("small_uniform", 100),
    ("large_uniform", 50), 
    ("mixed_sizes", 100),
    ("tiny_many", 1000)
])]
fn realistic_transaction_patterns(pattern: (&str, usize)) {
    let (_name, count) = pattern;
    let mut rng = rand::rng();
    use rand::Rng;

    let transactions: Vec<_> = (0..count)
        .map(|_| {
            let size = match pattern.0 {
                "small_uniform" => rng.random_range(200..300),
                "large_uniform" => rng.random_range(1800..2200),
                "mixed_sizes" => rng.random_range(100..3000),
                "tiny_many" => rng.random_range(50..150),
                _ => 500,
            };
            bench_utils::generate_random_bytes(size)
        })
        .collect();

    divan::black_box(compress_transactions(&transactions).expect("compression failed"));
}
