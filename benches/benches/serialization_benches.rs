//! Serialization and data structure benchmark suite

use alloy::primitives::{FixedBytes, U256};
use bench_utils::{generate_random_address, generate_random_hash, LoadScenario};
use synd_mchain::db::{ArbitrumBatch, Block, DelayedMessage, MBlock, Slot, State};

fn main() {
    divan::main();
}

/// Create test data structures for benchmarking
fn create_test_mblock(batch_size: usize, with_payload: bool) -> MBlock {
    let mut rng = rand::rng();
    use rand::Rng;

    let slot = Slot {
        seq_block_number: rng.random_range(1..1_000_000),
        seq_block_hash: generate_random_hash(),
        set_block_number: rng.random_range(1..1_000_000),
        set_block_hash: generate_random_hash(),
    };

    let payload = if with_payload {
        let delayed_messages: Vec<DelayedMessage> = (0..batch_size)
            .map(|_| DelayedMessage {
                kind: rng.random_range(0..10),
                sender: generate_random_address(),
                data: bench_utils::generate_random_bytes(rng.random_range(100..1000)),
                base_fee_l1: U256::from(rng.random_range(1_000_000..10_000_000u64)),
            })
            .collect();

        Some(ArbitrumBatch::new(
            bench_utils::generate_random_bytes(batch_size * 200),
            delayed_messages,
        ))
    } else {
        None
    };

    MBlock { timestamp: rng.random_range(1_600_000_000..1_700_000_000), slot, payload }
}

/// Benchmark `MBlock` serialization with bincode
#[divan::bench(args = [1, 5, 10, 25, 50, 100, 250, 500])]
fn mblock_serialization(bencher: divan::Bencher, batch_size: usize) {
    bencher.with_inputs(|| create_test_mblock(batch_size, true)).bench_values(|mblock| {
        let serialized = bincode::serde::encode_to_vec(&mblock, bincode::config::standard())
            .expect("serialization failed");
        divan::black_box(serialized);
    });
}

/// Benchmark `MBlock` deserialization with bincode
#[divan::bench(args = [10, 50, 100, 500])]
fn mblock_deserialization(bencher: divan::Bencher, batch_size: usize) {
    bencher
        .with_inputs(|| {
            let mblock = create_test_mblock(batch_size, true);
            bincode::serde::encode_to_vec(&mblock, bincode::config::standard())
                .expect("serialization failed")
        })
        .bench_values(|serialized_data| {
            let deserialized: MBlock =
                bincode::serde::decode_from_slice(&serialized_data, bincode::config::standard())
                    .expect("deserialization failed")
                    .0;
            divan::black_box(deserialized);
        });
}

/// Benchmark `ArbitrumBatch` serialization
#[divan::bench(args = [LoadScenario::Light, LoadScenario::Medium, LoadScenario::Heavy])]
fn arbitrum_batch_serialization(bencher: divan::Bencher, scenario: LoadScenario) {
    bencher
        .with_inputs(|| {
            let batch_size = scenario.batch_size();
            let mut rng = rand::rng();
            use rand::Rng;

            let delayed_messages: Vec<DelayedMessage> = (0..batch_size)
                .map(|_| DelayedMessage {
                    kind: rng.random_range(0..10),
                    sender: generate_random_address(),
                    data: bench_utils::generate_random_bytes(rng.random_range(100..1000)),
                    base_fee_l1: U256::from(rng.random_range(1_000_000..10_000_000u64)),
                })
                .collect();

            ArbitrumBatch::new(
                bench_utils::generate_random_bytes(batch_size * 200),
                delayed_messages,
            )
        })
        .bench_values(|batch| {
            let serialized = bincode::serde::encode_to_vec(&batch, bincode::config::standard())
                .expect("serialization failed");
            divan::black_box(serialized);
        });
}

/// Benchmark State structure serialization
#[divan::bench]
fn state_serialization(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| State {
            batch_count: 12345,
            batch_acc: generate_random_hash(),
            message_count: 98765,
            message_acc: generate_random_hash(),
            timestamp: 1700000000,
            slot: Slot {
                seq_block_number: 1000,
                seq_block_hash: generate_random_hash(),
                set_block_number: 2000,
                set_block_hash: generate_random_hash(),
            },
        })
        .bench_values(|state| {
            let serialized = bincode::serde::encode_to_vec(&state, bincode::config::standard())
                .expect("serialization failed");
            divan::black_box(serialized);
        });
}

/// Benchmark `Block` structure operations
#[divan::bench(args = [10, 50, 100, 500])]
fn block_serialization(bencher: divan::Bencher, num_messages: usize) {
    bencher
        .with_inputs(|| {
            let mut rng = rand::rng();
            use rand::Rng;

            let messages: Vec<(DelayedMessage, FixedBytes<32>)> = (0..num_messages)
                .map(|_| {
                    let message = DelayedMessage {
                        kind: rng.random_range(0..10),
                        sender: generate_random_address(),
                        data: bench_utils::generate_random_bytes(rng.random_range(100..500)),
                        base_fee_l1: U256::from(rng.random_range(1_000_000..10_000_000u64)),
                    };
                    (message, generate_random_hash())
                })
                .collect();

            Block {
                timestamp: 1700000000,
                batch: bench_utils::generate_random_bytes(1000),
                after_batch_acc: generate_random_hash(),
                messages,
                before_message_count: 100,
                before_message_acc: generate_random_hash(),
                before_batch_acc: generate_random_hash(),
                slot: Slot {
                    seq_block_number: 1000,
                    seq_block_hash: generate_random_hash(),
                    set_block_number: 2000,
                    set_block_hash: generate_random_hash(),
                },
            }
        })
        .bench_values(|block| {
            let serialized = bincode::serde::encode_to_vec(&block, bincode::config::standard())
                .expect("serialization failed");
            divan::black_box(serialized);
        });
}

/// Benchmark memory allocation during serialization
#[divan::bench(args = [50, 100, 500, 1000])]
fn serialization_memory_usage(bencher: divan::Bencher, batch_size: usize) {
    bencher.with_inputs(|| create_test_mblock(batch_size, true)).bench_values(|mblock| {
        let serialized = bincode::serde::encode_to_vec(&mblock, bincode::config::standard())
            .expect("serialization failed");
        divan::black_box(serialized);
    });
}

/// Benchmark round-trip serialization performance
#[divan::bench(args = [25, 50, 100, 250])]
fn roundtrip_serialization(bencher: divan::Bencher, batch_size: usize) {
    bencher.with_inputs(|| create_test_mblock(batch_size, true)).bench_values(|original| {
        let serialized = bincode::serde::encode_to_vec(&original, bincode::config::standard())
            .expect("serialization failed");
        let deserialized: MBlock =
            bincode::serde::decode_from_slice(&serialized, bincode::config::standard())
                .expect("deserialization failed")
                .0;
        divan::black_box(deserialized);
    });
}

/// Benchmark `DelayedMessage` creation and serialization
#[divan::bench(args = [100, 500, 1000, 5000])]
fn delayed_message_batch_serialization(bencher: divan::Bencher, num_messages: usize) {
    bencher
        .with_inputs(|| {
            let mut rng = rand::rng();
            use rand::Rng;

            (0..num_messages)
                .map(|_| DelayedMessage {
                    kind: rng.random_range(0..10),
                    sender: generate_random_address(),
                    data: bench_utils::generate_random_bytes(rng.random_range(50..500)),
                    base_fee_l1: U256::from(rng.random_range(1_000_000..10_000_000u64)),
                })
                .collect::<Vec<_>>()
        })
        .bench_values(|messages| {
            let serialized = bincode::serde::encode_to_vec(&messages, bincode::config::standard())
                .expect("serialization failed");
            divan::black_box(serialized);
        });
}
