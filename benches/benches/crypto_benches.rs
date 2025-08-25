//! Cryptographic operations benchmark suite

use alloy::{
    consensus::transaction::SignerRecoverable,
    primitives::{keccak256, FixedBytes, U256},
    sol_types::SolValue,
};
use bench_utils::{generate_hash_data, generate_random_address, generate_random_hash};
use shared::tx_validation::{check_signature, decode_transaction, validate_transaction};

fn main() {
    divan::main();
}

/// Benchmark keccak256 hashing performance with varying input sizes
#[divan::bench(args = [32, 64, 128, 256, 512, 1024, 2048, 4096])]
fn keccak256_hashing_single(bencher: divan::Bencher, input_size: usize) {
    bencher.with_inputs(|| bench_utils::generate_random_bytes(input_size)).bench_values(|data| {
        let result = keccak256(&data);
        divan::black_box(result);
    });
}

/// Benchmark batch keccak256 hashing (simulating accumulator updates)
#[divan::bench(args = [1, 10, 50, 100, 500, 1000])]
fn keccak256_batch_hashing(bencher: divan::Bencher, num_hashes: usize) {
    bencher.with_inputs(|| generate_hash_data(num_hashes)).bench_values(|data_sets| {
        let results: Vec<_> = data_sets.iter().map(keccak256).collect();
        divan::black_box(results);
    });
}

/// Benchmark accumulator-style hashing (chained keccak operations)
#[divan::bench(args = [10, 50, 100, 500, 1000])]
fn accumulator_hashing_chain(bencher: divan::Bencher, chain_length: usize) {
    bencher.with_inputs(generate_random_hash).bench_values(|message_hash| {
        let mut acc = FixedBytes::ZERO;
        for _ in 0..chain_length {
            acc = keccak256((acc, message_hash).abi_encode_packed());
        }
        divan::black_box(acc);
    });
}

/// Benchmark complex message hashing (simulating inbox message processing)
#[divan::bench(args = [1, 10, 50, 100, 500])]
fn complex_message_hashing(bencher: divan::Bencher, num_messages: usize) {
    bencher
        .with_inputs(|| {
            let sender = generate_random_address();
            let seq_block_number = 12345u64;
            let timestamp = 1700000000u64;
            let base_fee_l1 = U256::from(1000000000u64);
            (sender, seq_block_number, timestamp, base_fee_l1)
        })
        .bench_values(|(sender, seq_block_number, timestamp, base_fee_l1)| {
            let results: Vec<_> = (0..num_messages)
                .map(|i| {
                    let message_data = bench_utils::generate_random_bytes(256);
                    let message_count = U256::from(i as u64);

                    keccak256(
                        (
                            [1u8], // msg.kind
                            sender,
                            seq_block_number,
                            timestamp,
                            message_count,
                            base_fee_l1,
                            keccak256(&message_data),
                        )
                            .abi_encode_packed(),
                    )
                })
                .collect();
            divan::black_box(results);
        });
}

/// Benchmark transaction signature verification
#[divan::bench(args = [1, 10, 50, 100])]
fn signature_verification_batch(bencher: divan::Bencher, num_signatures: usize) {
    bencher
        .with_inputs(|| {
            let tx_data = "0xf86d8202b28477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ca05924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562a001b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772";
            let tx_bytes = alloy::primitives::hex::decode(tx_data).expect("valid hex");
            alloy::primitives::Bytes::from(tx_bytes)
        })
        .bench_values(|raw_tx| {
            let results: Vec<_> = (0..num_signatures)
                .map(|_| {
                    let tx = decode_transaction(&raw_tx).expect("valid transaction");
                    check_signature(&tx)
                })
                .collect::<Vec<_>>();
            divan::black_box(results);
        });
}

/// Benchmark full transaction validation pipeline
#[divan::bench(args = [1, 10, 50, 100])]
fn transaction_validation_pipeline(bencher: divan::Bencher, num_transactions: usize) {
    bencher
        .with_inputs(|| {
            let tx_data = "0xf86d8202b28477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ca05924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562a001b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772";
            let tx_bytes = alloy::primitives::hex::decode(tx_data).expect("valid hex");
            alloy::primitives::Bytes::from(tx_bytes)
        })
        .bench_values(|raw_tx| {
            let results: Vec<_> =
                (0..num_transactions).map(|_| validate_transaction(&raw_tx)).collect::<Vec<_>>();
            divan::black_box(results);
        });
}

/// Benchmark address recovery performance
#[divan::bench(args = [10, 50, 100, 500])]
fn address_recovery_performance(bencher: divan::Bencher, num_recoveries: usize) {
    bencher
        .with_inputs(|| {
            let tx_data = "0xf86d8202b28477359400825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ca05924bde7ef10aa88db9c66dd4f5fb16b46dff2319b9968be983118b57bb50562a001b24b31010004f13d9a26b320845257a6cfc2bf819a3d55e3fc86263c5f0772";
            let tx_bytes = alloy::primitives::hex::decode(tx_data).expect("valid hex");
            let raw_tx = alloy::primitives::Bytes::from(tx_bytes);
            decode_transaction(&raw_tx).expect("valid transaction")
        })
        .bench_values(|tx| {
            let addresses: Vec<_> =
                (0..num_recoveries).map(|_| tx.recover_signer().expect("valid signature")).collect();
            divan::black_box(addresses);
        });
}

/// Benchmark ABI encoding performance for complex structures
#[divan::bench(args = [10, 50, 100, 500])]
fn abi_encoding_performance(bencher: divan::Bencher, num_encodings: usize) {
    bencher
        .with_inputs(|| {
            let sender = generate_random_address();
            let seq_block_number = 12345u64;
            let timestamp = 1700000000u64;
            let base_fee_l1 = U256::from(1000000000u64);
            let message_data = bench_utils::generate_random_bytes(1024);
            (sender, seq_block_number, timestamp, base_fee_l1, message_data)
        })
        .bench_values(|(sender, seq_block_number, timestamp, base_fee_l1, message_data)| {
            let results: Vec<_> = (0..num_encodings)
                .map(|i| {
                    (
                        [1u8], // msg.kind
                        sender,
                        seq_block_number,
                        timestamp,
                        U256::from(i as u64),
                        base_fee_l1,
                        keccak256(&message_data),
                    )
                        .abi_encode_packed()
                })
                .collect();
            divan::black_box(results);
        });
}

/// Benchmark memory allocation patterns during crypto operations
#[divan::bench(args = [100, 500, 1000])]
fn crypto_memory_allocation(bencher: divan::Bencher, num_operations: usize) {
    let data_size = 512; // Fixed size for consistent memory measurements
    bencher.bench(|| {
        let results: Vec<_> = (0..num_operations)
            .map(|_| {
                let data = bench_utils::generate_random_bytes(data_size);
                keccak256(&data)
            })
            .collect();
        divan::black_box(results);
    })
}
