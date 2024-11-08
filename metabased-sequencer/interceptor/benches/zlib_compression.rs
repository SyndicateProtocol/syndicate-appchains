use alloy_primitives::hex_literal::hex;
use alloy::primitives::Bytes;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Instant;
use alloy::signers::k256::elliptic_curve::rand_core::RngCore;
use alloy_primitives::{Address, B256, U256};
use alloy_rlp::{Encodable, RlpDecodable, RlpEncodable};
use rand::Rng;
use interceptor::infrastructure::{compress_transaction, compress_transactions, decompress_transaction, decompress_transactions};

// Sample transactions used in benchmarks
const SAMPLE_TX_1: [u8; 110] = hex!("02f86b83014a3407830f4240830f443e825208944e527486594696a7607ff3379e21746689a3fd6d1480c080a0502ec1e72aa5d8e52f2547c3dcb973d6129364828ea54cfd166ea74350a60cd4a02db70ba79cfb18a45d6b415e58aed8947bb66efc1156c2067e59d4ea5c69cfcb");
const SAMPLE_TX_2: [u8; 132] = hex!("cdb554ea000000000000000000000000b8b904c73d2fb4d8c173298a51c27fab70222c320000000000000000000000000000000000000000000000000000000000568936000000000000000000000000b8b904c73d2fb4d8c173298a51c27fab70222c32000000000000000000000000000000000000000000000000000000000059bd0d");
const SAMPLE_TX_3: [u8; 68] = hex!("39509351000000000000000000000000dd2da9ba748722faea8629a215ea47dd15e852f90000000000000000000000000000000000000000000000000429d069189e0000");
const SAMPLE_TX_4: [u8; 132] = hex!("81813c8b0000000000000000000000000000000000000000000000000000000001026afc0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000016345785d8a000000000000000000000000000000000000000000000000000000000000671834d8");

fn print_compression_stats(name: &str, original_size: usize, compressed_size: usize,
                           decompressed_size: usize, compress_time: std::time::Duration,
                           decompress_time: std::time::Duration) {
    let ratio = (1.0 - (compressed_size as f64 / original_size as f64)) * 100.0;
    println!("\n{}", name);
    println!("Compression ratio: {:.2}%", ratio);
    println!("Original size: {} bytes", original_size);
    println!("Compressed size: {} bytes", compressed_size);
    println!("Decompressed size: {} bytes", decompressed_size);
    println!("Compression time: {:?}", compress_time);
    println!("Decompression time: {:?}", decompress_time);
}

fn bench_single_tx(c: &mut Criterion) {
    let input = &SAMPLE_TX_1;

    // Pre-calculate sizes
    let compressed = compress_transaction(input).unwrap();
    let decompressed = decompress_transaction(&compressed).unwrap();
    let compressed_size = compressed.len();
    let decompressed_size = decompressed.len();

    // Run the benchmark
    c.bench_function("single_tx_compression", |b| {
        b.iter(|| {
            let start = Instant::now();
            let compressed = compress_transaction(input).unwrap();
            let compress_time = start.elapsed();

            let start = Instant::now();
            let decompressed = decompress_transaction(&compressed).unwrap();
            let decompression_time = start.elapsed();

            assert_eq!(input[..], decompressed[..]);
            (compress_time, decompression_time)
        })
    });

    // Get example timings for stats (outside the benchmark)
    let (example_compress_time, example_decompress_time) = {
        let start = Instant::now();
        let compressed = compress_transaction(input).unwrap();
        let compress_time = start.elapsed();

        let start = Instant::now();
        let _decompressed = decompress_transaction(&compressed).unwrap();
        let decompress_time = start.elapsed();

        (compress_time, decompress_time)
    };

    // Print stats once after everything
    print_compression_stats(
        "Single TX compression",
        input.len(),
        compressed_size,
        decompressed_size,
        example_compress_time,
        example_decompress_time
    );
}

#[derive(RlpEncodable, RlpDecodable)]
struct Transaction {
    chain_id: u64,
    nonce: u64,
    max_priority_fee_per_gas: U256,
    gas_price: U256,
    gas_limit: U256,
    to: Address,
    value: U256,
    data: Bytes,
    access_list: Vec<u8>,
    v: u64,
    r: B256,
    s: B256,
}

/// Generates a random raw Ethereum transaction in hexadecimal format
pub fn generate_random_raw_transaction_rlp() -> String {
    let mut rng = rand::thread_rng();

    // Generate random recipient address
    let mut recipient_bytes = [0u8; 20];
    rng.fill_bytes(&mut recipient_bytes);

    // Generate random signature values
    let mut r_bytes = [0u8; 32];
    let mut s_bytes = [0u8; 32];
    rng.fill_bytes(&mut r_bytes);
    rng.fill_bytes(&mut s_bytes);

    // Convert Gwei to Wei for gas prices (1 Gwei = 1e9 Wei)
    let min_gas_price_gwei: u64 = 1;
    let max_gas_price_gwei: u64 = 100;
    let gas_price_gwei = rng.gen_range(min_gas_price_gwei..=max_gas_price_gwei);
    let gas_price = U256::from(gas_price_gwei) * U256::from(1_000_000_000u64);

    let min_priority_fee_gwei: u64 = 1;
    let max_priority_fee_gwei: u64 = 50;
    let priority_fee_gwei = rng.gen_range(min_priority_fee_gwei..=max_priority_fee_gwei);
    let priority_fee = U256::from(priority_fee_gwei) * U256::from(1_000_000_000u64);

    let tx = Transaction {
        chain_id: 1, // mainnet
        nonce: rng.gen_range(0..1_000_000),
        max_priority_fee_per_gas: priority_fee,
        gas_price,
        gas_limit: U256::from(rng.gen_range(21_000..1_000_000)),
        to: Address::from_slice(&recipient_bytes),
        // Random value between 0 and 2 ETH (in Wei)
        value: U256::from(rng.gen_range(0..=2_000_000_000_000_000_000u64)),
        data: Bytes::default(),
        access_list: vec![1],
        v: 2 + 35 + rng.gen_range(0..2), // EIP-155 encoding
        r: B256::from_slice(&r_bytes),
        s: B256::from_slice(&s_bytes),
    };

    // Encode the transaction using RLP
    let mut buf = Vec::new();
    tx.encode(&mut buf);

    // Convert to hex string with "0x" prefix
    format!("0x02{}", hex::encode(&buf))
}

#[test]
fn test_generate_transaction() {
    let tx = generate_random_raw_transaction_rlp();
    // Transaction should start with "0x02" and be a valid hex string
    assert!(tx.starts_with("0x02"));
    // Verify we can decode the hex string
    assert!(hex::decode(&tx[4..]).is_ok());
}

fn bench_batch_multiple_tx(c: &mut Criterion) {
    let txs = vec![
        Bytes::copy_from_slice(&SAMPLE_TX_1),
        Bytes::copy_from_slice(&SAMPLE_TX_2[0..50]), // Partial TX
        Bytes::copy_from_slice(&[1, 2, 3, 4, 5]),    // Small TX
        Bytes::copy_from_slice(&SAMPLE_TX_3),
        Bytes::copy_from_slice(&SAMPLE_TX_4),
    ];

    // Get initial statistics once before benchmarking
    let compressed = compress_transactions(&txs).unwrap();
    let original_size: usize = txs.iter().map(|tx| tx.len()).sum();
    let decompressed = decompress_transactions(&compressed).unwrap();
    let compressed_size = compressed.len();
    let decompressed_size = decompressed.iter().map(|tx| tx.len()).sum();

    // Run the actual benchmark
    c.bench_function("batch_multiple_tx", |b| {
        b.iter(|| {
            let start = Instant::now();
            let compressed = compress_transactions(&txs).unwrap();
            let compress_time = start.elapsed();

            let start = Instant::now();
            let decompressed = decompress_transactions(&compressed).unwrap();
            let decompression_time = start.elapsed();

            assert_eq!(txs, decompressed);

            (compress_time, decompression_time)
        })
    });

    // Print stats once after benchmarking
    // Using the pre-calculated sizes and example timing from one run
    let (example_compress_time, example_decompress_time) = {
        let start = Instant::now();
        let compressed = compress_transactions(&txs).unwrap();
        let compress_time = start.elapsed();

        let start = Instant::now();
        let _decompressed = decompress_transactions(&compressed).unwrap();
        let decompress_time = start.elapsed();

        (compress_time, decompress_time)
    };

    print_compression_stats(
        "Multiple TX compression",
        original_size,
        compressed_size,
        decompressed_size,
        example_compress_time,
        example_decompress_time
    );
}

fn bench_batch_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_sizes");

    for size in [100, 1000].iter() {
        let mut txs = Vec::new();
        txs.push(Bytes::copy_from_slice(&SAMPLE_TX_1));
        for _ in 0..*size {
            txs.push(Bytes::copy_from_slice(generate_random_raw_transaction_rlp().as_ref()));
        }

        // Pre-calculate sizes
        let compressed = compress_transactions(&txs).unwrap();
        let original_size: usize = txs.iter().map(|tx| tx.len()).sum();
        let decompressed = decompress_transactions(&compressed).unwrap();
        let compressed_size = compressed.len();
        let decompressed_size = decompressed.iter().map(|tx| tx.len()).sum();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
            b.iter(|| {
                let start = Instant::now();
                let compressed = compress_transactions(&txs).unwrap();
                let compress_time = start.elapsed();

                let start = Instant::now();
                let decompressed = decompress_transactions(&compressed).unwrap();
                let decompression_time = start.elapsed();

                assert_eq!(txs, decompressed);
                (compress_time, decompression_time)
            })
        });

        // Print stats once after each batch size benchmark
        let (example_compress_time, example_decompress_time) = {
            let start = Instant::now();
            let compressed = compress_transactions(&txs).unwrap();
            let compress_time = start.elapsed();

            let start = Instant::now();
            let _decompressed = decompress_transactions(&compressed).unwrap();
            let decompress_time = start.elapsed();

            (compress_time, decompress_time)
        };

        print_compression_stats(
            &format!("Batch compression (n={})", size),
            original_size,
            compressed_size,
            decompressed_size,
            example_compress_time,
            example_decompress_time
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_single_tx,
    bench_batch_multiple_tx,
    bench_batch_sizes
);
criterion_main!(benches);