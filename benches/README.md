# Syndicate Appchains Benchmark Suite

This directory contains comprehensive benchmark tests for the Syndicate Appchains project using the [`divan`](https://github.com/nvzqz/divan) benchmarking library.

## Overview

The benchmark suite is organized into focused test categories that measure performance-critical operations across the appchain infrastructure:

### 📦 Compression Benchmarks (`compression_benches.rs`)
- **Transaction compression** - Single transaction zlib compression performance
- **Batch compression** - Multi-transaction batch compression throughput  
- **Compression ratios** - Analysis of compression efficiency vs speed tradeoffs
- **Round-trip testing** - Full compression/decompression cycles
- **Memory allocation** - Memory usage patterns during compression
- **Realistic patterns** - Performance with varied transaction size distributions

**Key Metrics**: Throughput (ops/sec), compression ratio, memory usage

### 🔐 Cryptographic Benchmarks (`crypto_benches.rs`)
- **Hash operations** - keccak256 performance across different input sizes
- **Accumulator hashing** - Chained hash operations for state accumulators
- **Message hashing** - Complex multi-field message hash computations
- **Signature verification** - Transaction signature validation performance
- **Address recovery** - ECDSA public key recovery operations
- **ABI encoding** - Structured data encoding performance

**Key Metrics**: Hash throughput, signature verification rate, memory allocation

### 🗄️ Database Benchmarks (`database_benches.rs`)  
- **Batch operations** - RocksDB write performance for various batch sizes
- **Serialization** - Bincode encode/decode overhead measurement
- **State updates** - Chain state persistence performance
- **Concurrent operations** - Multi-threaded database access patterns
- **Memory usage** - Memory footprint during database operations
- **Accumulator updates** - Hash computation load during batch processing

**Key Metrics**: Write throughput, serialization overhead, concurrent performance

## Running Benchmarks

### Prerequisites

Ensure you have the required system dependencies:

```bash
# Ubuntu/Debian
sudo apt-get install librocksdb-dev

# macOS
brew install rocksdb
```

### Individual Benchmark Suites

```bash
# Run compression benchmarks
cargo bench --bench compression_benches

# Run crypto benchmarks  
cargo bench --bench crypto_benches

# Run database benchmarks
cargo bench --bench database_benches
```

### All Benchmarks

```bash
# Run all benchmark suites
cargo bench
```

### Filtering Specific Tests

```bash
# Run only keccak256 benchmarks
cargo bench --bench crypto_benches -- keccak256

# Run only large batch tests
cargo bench --bench database_benches -- "batch_size.*1000"

# Run with specific argument values
cargo bench --bench compression_benches -- "compress_transaction_batch.*100"
```

### Output Options

```bash
# JSON output for CI/analysis
cargo bench --bench compression_benches -- --output-format json > results.json

# Quiet mode (less verbose)
cargo bench -- --quiet

# Sample size control
cargo bench -- --sample-size 100
```

## Benchmark Configuration

The benchmarks are configured with realistic parameter ranges based on production workloads:

- **Transaction sizes**: 100 bytes - 5KB (typical range)
- **Batch sizes**: 1-1000 transactions 
- **Hash data**: 32 bytes - 4KB inputs
- **Database operations**: 1-500 batch sizes
- **Concurrency**: 1-8 threads

## CI Integration

Benchmarks run automatically on:
- **Push** to main branch or feature branches affecting performance-critical code
- **Pull requests** with performance comparison against main branch
- **Manual trigger** via GitHub Actions workflow dispatch

Results are uploaded as artifacts and PR comments show performance summaries.

## Performance Profiling Integration

For deeper analysis, combine benchmarks with profiling tools:

```bash
# CPU profiling with perf
cargo bench --bench crypto_benches -- --profile

# Memory profiling with valgrind  
cargo bench --bench database_benches -- --memory-profiler

# Flamegraph generation
cargo flamegraph --bench compression_benches
```

## Interpreting Results

### Understanding Output

Divan provides comprehensive metrics for each benchmark:

```
compress_transaction_batch_by_count/100  478.9 µs   495.8 µs   513.4 µs
├─ fastest        478.9 µs  (2 samples)
├─ slowest        513.4 µs  (2 samples) 
├─ median         495.8 µs  (2 samples)
├─ mean           495.8 µs  (2 samples)
├─ sample count   2
└─ iterations     208 (104/sample)
```

- **fastest/slowest**: Performance range across all samples
- **median/mean**: Central tendency measures
- **sample count**: Number of independent measurement samples
- **iterations**: Total function calls (iterations per sample)

### Performance Targets

Establish baseline performance expectations:

- **Compression**: >1000 transactions/sec for typical batches
- **Hashing**: >10,000 keccak256 ops/sec
- **Database writes**: >500 batch writes/sec
- **Signature verification**: >100 verifications/sec

### Regression Detection

Monitor for performance regressions:
- **>10% degradation** in core operations requires investigation
- **Memory usage increases** may indicate leaks or inefficient allocation
- **Concurrency scaling** should show near-linear improvements

## Contributing New Benchmarks

When adding new benchmarks:

1. **Focus on hot paths** - Profile code to identify bottlenecks
2. **Realistic data** - Use representative input sizes and patterns
3. **Multiple scenarios** - Test various load conditions
4. **Memory tracking** - Include memory allocation measurements where relevant
5. **Documentation** - Explain what the benchmark measures and why

### Example Benchmark Structure

```rust
#[divan::bench(args = [10, 100, 1000])]
fn my_operation_benchmark(input_size: usize) -> divan::Bencher {
    divan::Bencher::new()
        .with_inputs(|| {
            // Setup test data (not timed)
            generate_test_data(input_size)
        })
        .counter(divan::counter::BytesCount::new(input_size))
        .bench_values(|data| {
            // Timed operation
            divan::black_box(my_operation(data));
        })
}
```

## Troubleshooting

### Common Issues

**RocksDB linking errors**:
```bash
export ROCKSDB_LIB_DIR=/usr/lib/x86_64-linux-gnu
export ROCKSDB_INCLUDE_DIR=/usr/include
```

**Benchmark timeouts**:
```bash
cargo bench -- --max-time 60  # Extend timeout to 60 seconds
```

**Memory allocation tracking**:
```bash
# Ensure sufficient system memory for large benchmarks
ulimit -m unlimited
```

### Performance Debugging

For unexpected results:
1. Run benchmarks multiple times to ensure consistency
2. Check system load during benchmark execution
3. Verify input data generation doesn't dominate runtime
4. Use `divan::black_box()` to prevent compiler optimizations
5. Profile with external tools for detailed analysis

## Future Enhancements

Planned improvements:
- **Historical tracking** - Performance trend analysis over time
- **Regression alerts** - Automated performance regression detection  
- **Cross-platform testing** - Windows and macOS benchmark validation
- **Load testing** - End-to-end system performance under sustained load
- **Memory profiling** - Detailed allocation pattern analysis