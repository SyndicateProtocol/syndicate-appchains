//! Compression benchmark suite for batch sequencing operations

use bench_utils::{
    generate_test_transactions, generate_transactions_with_pattern,
    generate_transactions_with_size, DataSize,
};
use synd_batch_sequencer::sequencing_batch::compress_batch;

fn main() {
    divan::main();
}

/// Benchmark single transaction compression performance using batch compression
#[divan::bench(args = [100, 500, 1000, 2000, 5000])]
fn compress_single_transaction(bencher: divan::Bencher, tx_size: usize) {
    bencher
        .with_inputs(|| {
            let transaction = bench_utils::generate_random_bytes(tx_size).to_vec();
            (transaction, "bench_tx".to_string())
        })
        .bench_values(|tx_with_id| {
            let result = compress_batch(&[], tx_with_id).expect("compression failed");
            divan::black_box(result);
        });
}

/// Benchmark transaction batch compression with varying transaction counts
#[divan::bench(args = [1, 10, 50, 100, 500])] // Reduced max for batch operations
fn compress_transaction_batch_by_count(bencher: divan::Bencher, n_transactions: usize) {
    bencher.with_inputs(|| generate_test_transactions(n_transactions)).bench_values(
        |transactions| {
            if let Some((first_tx, rest)) = transactions.split_first() {
                let mut batch = vec![first_tx.clone()];
                for tx in rest {
                    let result =
                        compress_batch(&batch, tx.clone()).expect("batch compression failed");
                    batch = result.into_owned_txs();
                }
                divan::black_box(batch);
            }
        },
    );
}

/// Benchmark compression performance by total data size
#[divan::bench(args = [DataSize::Small, DataSize::Medium, DataSize::Large])]
fn compress_by_total_size(bencher: divan::Bencher, size: DataSize) {
    bencher
        .with_inputs(|| {
            let transactions = generate_transactions_with_size(size.bytes());
            // Take only first 10 to keep benchmark reasonable
            transactions.into_iter().take(10).collect::<Vec<_>>()
        })
        .bench_values(|transactions| {
            if let Some((first_tx, rest)) = transactions.split_first() {
                let mut batch = vec![first_tx.clone()];
                for tx in rest {
                    let result =
                        compress_batch(&batch, tx.clone()).expect("compression by size failed");
                    batch = result.into_owned_txs();
                }
                divan::black_box(batch);
            }
        });
}

/// Benchmark batch compression performance with different transaction counts
#[divan::bench(args = [10, 50, 100])]
fn batch_compression_performance(bencher: divan::Bencher, n_transactions: usize) {
    bencher.with_inputs(|| generate_test_transactions(n_transactions)).bench_values(
        |transactions| {
            if let Some((first_tx, rest)) = transactions.split_first() {
                let mut current_batch = vec![first_tx.clone()];
                for tx in rest {
                    let result =
                        compress_batch(&current_batch, tx.clone()).expect("compression failed");
                    current_batch = result.into_owned_txs();
                }
                divan::black_box(current_batch);
            }
        },
    );
}

/// Benchmark incremental batch building
#[divan::bench(args = [5, 10, 25, 50])]
fn incremental_batch_building(bencher: divan::Bencher, n_transactions: usize) {
    bencher.with_inputs(|| generate_test_transactions(n_transactions)).bench_values(
        |transactions| {
            if let Some((first_tx, rest)) = transactions.split_first() {
                let mut current_batch = vec![first_tx.clone()];
                for tx in rest {
                    let result = compress_batch(&current_batch, tx.clone())
                        .expect("incremental compression failed");
                    current_batch = result.into_owned_txs();
                }
                divan::black_box(current_batch);
            }
        },
    );
}

/// Benchmark compression performance with realistic transaction patterns
#[divan::bench(args = [
    ("small_uniform", 20),
    ("large_uniform", 10),
    ("mixed_sizes", 25),
    ("tiny_many", 50)
])]
fn realistic_transaction_patterns(bencher: divan::Bencher, pattern: (&str, usize)) {
    let (pattern_name, count) = pattern;
    bencher.with_inputs(|| generate_transactions_with_pattern(pattern_name, count)).bench_values(
        |transactions| {
            if let Some((first_tx, rest)) = transactions.split_first() {
                let mut current_batch = vec![first_tx.clone()];
                for tx in rest {
                    let result =
                        compress_batch(&current_batch, tx.clone()).expect("compression failed");
                    current_batch = result.into_owned_txs();
                }
                divan::black_box(current_batch);
            }
        },
    );
}

/// Benchmark empty batch compression
#[divan::bench]
fn compress_empty_batch() {
    let tx = (vec![1, 2, 3], "single_tx".to_string());
    let result = compress_batch(&[], tx).expect("empty batch compression failed");
    divan::black_box(result);
}

/// Benchmark large single transaction compression
#[divan::bench(args = [10_000, 50_000, 100_000])]
fn compress_large_single_transaction(bencher: divan::Bencher, tx_size: usize) {
    bencher
        .with_inputs(|| {
            (bench_utils::generate_random_bytes(tx_size).to_vec(), "large_tx".to_string())
        })
        .bench_values(|large_tx| {
            let result =
                compress_batch(&[], large_tx).expect("large transaction compression failed");
            divan::black_box(result);
        });
}

/// Benchmark batch size growth impact on compression performance
#[divan::bench(args = [1, 5, 10, 20, 50])]
fn batch_size_impact(bencher: divan::Bencher, batch_size: usize) {
    bencher.with_inputs(|| generate_test_transactions(batch_size + 1)).bench_values(
        |transactions| {
            if let Some((last_tx, existing_txs)) = transactions.split_last() {
                let result = compress_batch(existing_txs, last_tx.clone())
                    .expect("batch size compression failed");
                divan::black_box(result);
            }
        },
    );
}

/// Benchmark repeated compression of same transaction into growing batch
#[divan::bench(args = [10, 25, 50])]
fn repeated_transaction_compression(bencher: divan::Bencher, iterations: usize) {
    bencher
        .with_inputs(|| {
            (
                (vec![1, 2, 3, 4, 5], "base_tx".to_string()),
                (vec![6, 7, 8, 9, 10], "repeated_tx".to_string()),
            )
        })
        .bench_values(|(base_tx, new_tx)| {
            let mut current_batch = vec![base_tx];

            for _ in 0..iterations {
                let result = compress_batch(&current_batch, new_tx.clone())
                    .expect("repeated compression failed");
                current_batch = result.into_owned_txs();
            }

            divan::black_box(current_batch);
        });
}
