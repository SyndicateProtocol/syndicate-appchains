//! Integration tests for Redis Streams performance

use eyre::{eyre, Result};
use maestro::{
    errors::Error,
    redis_manager::{StreamConsumer, StreamManager, StreamProducer, TransactionRequest},
};
use rand::{distributions::Alphanumeric, Rng};
use redis::AsyncCommands;
use std::{
    fmt,
    sync::Arc,
    time::{Duration, Instant},
};
use test_utils::{port_manager::PortManager, wait_until};
use tokio::{
    process::{Child, Command},
    runtime::Handle,
    sync::{Barrier, Mutex},
    task,
    time::timeout,
};
use tracing::{debug, error, info};

const TX_STREAM_KEY: &str = "maestro:transactions";

/// Struct to manage Docker container lifecycle
#[derive(Debug)]
pub struct Docker(Child);

impl Drop for Docker {
    fn drop(&mut self) {
        if let Some(x) = self.0.id() {
            let _ = std::process::Command::new("kill").arg(x.to_string()).output();
            task::block_in_place(move || {
                Handle::current().block_on(async move {
                    let _ = self.0.wait().await;
                })
            })
        }
    }
}

/// Configuration for the benchmark
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Number of producer tasks
    pub producer_count: usize,
    /// Number of consumer tasks
    pub consumer_count: usize,
    /// Total number of transactions to produce
    pub transaction_count: usize,
    /// Size of each transaction in bytes (approximate)
    pub transaction_size: usize,
    /// How long to run the benchmark for (in seconds)
    pub duration_seconds: u64,
    /// Redis port
    pub redis_port: u16,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            producer_count: 4,
            consumer_count: 4,
            transaction_count: 10_000,
            transaction_size: 1024, // 1KB
            duration_seconds: 10,
            redis_port: 6379,
        }
    }
}

/// Results from the benchmark
#[derive(Debug)]
pub struct BenchmarkResults {
    /// Total number of transactions produced
    pub transactions_produced: usize,
    /// Total number of transactions consumed
    pub transactions_consumed: usize,
    /// Average enqueue latency in microseconds
    pub avg_enqueue_latency_us: f64,
    /// Average dequeue latency in microseconds
    pub avg_dequeue_latency_us: f64,
    /// Enqueue throughput (transactions per second)
    pub enqueue_tps: f64,
    /// Dequeue throughput (transactions per second)
    pub dequeue_tps: f64,
    /// Total runtime in seconds
    pub runtime_seconds: f64,
}

impl fmt::Display for BenchmarkResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        writeln!(f, "=== BENCHMARK RESULTS ===")?;
        writeln!(f, "Total Runtime: {:.2} seconds", self.runtime_seconds)?;
        writeln!(f, "Transactions Produced: {}", self.transactions_produced)?;
        writeln!(f, "Transactions Consumed: {}", self.transactions_consumed)?;
        writeln!(f, "Enqueue Throughput: {:.2} TPS", self.enqueue_tps)?;
        writeln!(f, "Dequeue Throughput: {:.2} TPS", self.dequeue_tps)?;
        writeln!(f, "Avg Enqueue Latency: {:.2} μs", self.avg_enqueue_latency_us)?;
        writeln!(f, "Avg Dequeue Latency: {:.2} μs", self.avg_dequeue_latency_us)?;
        writeln!(f, "=======================")
    }
}

/// Statistics collected during benchmark
#[derive(Debug, Default)]
struct BenchmarkStats {
    transactions_produced: usize,
    transactions_consumed: usize,
    enqueue_latency_us_sum: u128,
    dequeue_latency_us_sum: u128,
}

/// Start a Redis container
pub async fn start_redis(port: u16) -> Result<Docker> {
    info!("Starting Redis container on port {}", port);

    // Use a unique name based on the port
    let container_name = format!("redis-benchmark-{}", port);

    // Stop any existing container with the same name to avoid conflicts
    let _ = Command::new("docker").args(["stop", &container_name]).output().await;

    let redis = Command::new("docker")
        .args([
            "run",
            "--init",
            "--rm",
            "--name",
            &container_name,
            "-p",
            &format!("{}:6379", port),
            "redis:alpine",
            "redis-server",
            "--save",
            "", // Disable persistence
            "--appendonly",
            "no", // Disable AOF
            "--maxmemory",
            "256mb",
            "--maxmemory-policy",
            "allkeys-lru",
        ])
        .spawn()?;

    // Wait for Redis to be ready
    timeout(Duration::from_secs(30), async {
        let mut retries = 0;
        while retries < 50 {
            let result = Command::new("docker")
                .args(["exec", &container_name, "redis-cli", "ping"])
                .output()
                .await;

            match result {
                Ok(output) if output.status.success() && output.stdout.starts_with(b"PONG") => {
                    info!("Redis is ready on port {}", port);
                    return Ok::<_, eyre::Error>(());
                }
                _ => {
                    retries += 1;
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        }

        Err(eyre!("Redis failed to start"))
    })
    .await??;

    Ok(Docker(redis))
}

/// Run the Redis Streams benchmark with a test Redis instance
pub async fn run_benchmark_with_redis() -> Result<BenchmarkResults> {
    // Get a port for Redis
    let port = PortManager::instance().next_port();

    // Create benchmark config
    let config = BenchmarkConfig { redis_port: port, ..Default::default() };

    // Start Redis
    let _redis = start_redis(port).await?;

    // Run the benchmark
    run_benchmark(config).await
}

/// Run the Redis Streams benchmark
pub async fn run_benchmark(config: BenchmarkConfig) -> Result<BenchmarkResults> {
    info!("Starting Redis Streams benchmark with config: {:?}", config);

    let redis_url = format!("redis://127.0.0.1:{}", config.redis_port);

    // Create a shared Redis client
    let client = redis::Client::open(redis_url.clone())?;

    // Create shared statistics
    let stats = Arc::new(Mutex::new(BenchmarkStats::default()));

    // Create a barrier to synchronize all tasks
    let start_barrier = Arc::new(Barrier::new(config.producer_count + config.consumer_count + 1));

    // Create a flag to signal when to stop
    let stop = Arc::new(Mutex::new(false));

    // Calculate transactions per producer
    let txs_per_producer = config.transaction_count / config.producer_count;

    // Create a stream manager to initialize the consumer group
    let conn = client.get_multiplexed_async_connection().await?;
    let mut stream_manager = StreamManager::new(conn.clone());
    let consumer_group_name = stream_manager.init_consumer_group().await?;
    let consumer_group_name_arc = Arc::new(consumer_group_name.to_string());

    // Start the producer tasks
    let producer_handles = (0..config.producer_count)
        .map(|id| {
            let client_clone = client.clone();
            let barrier_clone = start_barrier.clone();
            let stats_clone = stats.clone();
            let stop_clone = stop.clone();
            let config_clone = config.clone();

            tokio::spawn(async move {
                let conn = client_clone.get_multiplexed_async_connection().await?;
                let stream_manager = StreamManager::new(conn);
                let mut producer = stream_manager.create_producer()?;

                // Wait for all tasks to be ready
                barrier_clone.wait().await;

                producer_task(
                    id,
                    &mut producer,
                    txs_per_producer,
                    config_clone.transaction_size,
                    stats_clone,
                    stop_clone,
                )
                .await
            })
        })
        .collect::<Vec<_>>();

    // Start the consumer tasks
    let consumer_handles = (0..config.consumer_count)
        .map(|id| {
            let client_clone = client.clone();
            let barrier_clone = start_barrier.clone();
            let stats_clone = stats.clone();
            let stop_clone = stop.clone();

            let group_name_clone = Arc::clone(&consumer_group_name_arc);
            tokio::spawn(async move {
                let conn = client_clone.get_multiplexed_async_connection().await?;
                let stream_manager = StreamManager::new(conn);
                let mut consumer =
                    stream_manager.create_consumer_with_group(group_name_clone.as_str(), id)?;

                // Wait for all tasks to be ready
                barrier_clone.wait().await;

                consumer_task(id, &mut consumer, stats_clone, stop_clone).await
            })
        })
        .collect::<Vec<_>>();

    // Start the benchmark
    let start_time = Instant::now();
    start_barrier.wait().await;
    info!("Benchmark started!");

    // Run for the specified duration
    tokio::time::sleep(Duration::from_secs(config.duration_seconds)).await;

    // Signal all tasks to stop
    let mut stop_guard = stop.lock().await;
    *stop_guard = true;
    drop(stop_guard);
    info!("Stopping benchmark...");

    // Wait for all tasks to complete
    let (producer_results, consumer_results) = tokio::join!(
        async {
            let mut results = Vec::new();
            for handle in producer_handles {
                results.push(handle.await);
            }
            results
        },
        async {
            let mut results = Vec::new();
            for handle in consumer_handles {
                results.push(handle.await);
            }
            results
        }
    );

    // Check for any errors
    for result in producer_results {
        if let Err(e) = result {
            error!("Producer task panicked: {}", e);
        }
    }

    for result in consumer_results {
        if let Err(e) = result {
            error!("Consumer task panicked: {}", e);
        }
    }

    // Calculate final stats
    let runtime_seconds = start_time.elapsed().as_secs_f64();
    let stats_guard = stats.lock().await;

    let avg_enqueue_latency_us = if stats_guard.transactions_produced > 0 {
        stats_guard.enqueue_latency_us_sum as f64 / stats_guard.transactions_produced as f64
    } else {
        0.0
    };

    let avg_dequeue_latency_us = if stats_guard.transactions_consumed > 0 {
        stats_guard.dequeue_latency_us_sum as f64 / stats_guard.transactions_consumed as f64
    } else {
        0.0
    };

    let results = BenchmarkResults {
        transactions_produced: stats_guard.transactions_produced,
        transactions_consumed: stats_guard.transactions_consumed,
        avg_enqueue_latency_us,
        avg_dequeue_latency_us,
        enqueue_tps: stats_guard.transactions_produced as f64 / runtime_seconds,
        dequeue_tps: stats_guard.transactions_consumed as f64 / runtime_seconds,
        runtime_seconds,
    };

    #[allow(clippy::expect_used)]
    let stream_info: redis::Value =
        conn.clone().xinfo_stream(TX_STREAM_KEY).await.expect("Failed to get stream info");
    info!("Stream info: {:?}", stream_info);

    info!("Benchmark complete! Results: {:?}", results);
    Ok(results)
}

/// Producer task that generates and enqueues transactions
async fn producer_task(
    id: usize,
    producer: &mut StreamProducer,
    transaction_count: usize,
    transaction_size: usize,
    stats: Arc<Mutex<BenchmarkStats>>,
    stop: Arc<Mutex<bool>>,
) -> Result<(), Error> {
    info!("Producer {id} started");

    let mut count = 0;
    loop {
        // Check if we should stop
        if *stop.lock().await || count >= transaction_count {
            break;
        }

        // Generate a random transaction
        let tx = generate_random_transaction(transaction_size);

        // Measure enqueue latency
        let start = Instant::now();
        let result = producer.enqueue_transaction(tx).await;
        let latency_us = start.elapsed().as_micros();

        match result {
            Ok(_) => {
                // Update stats
                let mut stats_guard = stats.lock().await;
                stats_guard.transactions_produced += 1;
                stats_guard.enqueue_latency_us_sum += latency_us;

                count += 1;
                if count % 1000 == 0 {
                    debug!("Producer {id} enqueued {count} transactions");
                }
            }
            Err(e) => {
                error!("Producer {id} failed to enqueue transaction: {e}");
                // Short backoff on error
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }

    info!("Producer {id} finished, enqueued {count} transactions");
    Ok(())
}

/// Consumer task that dequeues and processes transactions
async fn consumer_task(
    id: usize,
    consumer: &mut StreamConsumer,
    stats: Arc<Mutex<BenchmarkStats>>,
    stop: Arc<Mutex<bool>>,
) -> Result<(), Error> {
    info!("Consumer {id} started");

    let mut count = 0;
    loop {
        // Check if we should stop
        if *stop.lock().await {
            break;
        }

        // Try to process a batch with timeout
        let start = Instant::now();
        let result = timeout(Duration::from_secs(1), consumer.process_batch()).await;

        match result {
            Ok(Ok(processed)) => {
                if processed > 0 {
                    let latency_us = start.elapsed().as_micros() / processed as u128;

                    // Update stats
                    let mut stats_guard = stats.lock().await;
                    stats_guard.transactions_consumed += processed;
                    stats_guard.dequeue_latency_us_sum += latency_us * processed as u128;

                    count += processed;
                    debug!("Consumer {id} processed {processed} transactions (total: {count})");
                } else {
                    // No messages, small sleep to avoid CPU spin
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }
            Ok(Err(e)) => {
                error!("Consumer {id} failed to process batch: {e}");
                // Short backoff on error
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            Err(_) => {
                // Timeout occurred, continue
                debug!("Consumer {id} timed out waiting for messages");
            }
        }
    }

    info!("Consumer {id} finished, processed {count} transactions");
    Ok(())
}

/// Generate a random transaction for benchmarking
fn generate_random_transaction(size: usize) -> TransactionRequest {
    let mut rng = rand::thread_rng();

    // Generate random hex string for tx_hash (fixed size)
    let tx_hash =
        format!("0x{}", (0..64).map(|_| format!("{:x}", rng.gen::<u8>())).collect::<String>());

    // Generate random address for sender (fixed size)
    let sender =
        format!("0x{}", (0..40).map(|_| format!("{:x}", rng.gen::<u8>())).collect::<String>());

    // Generate random raw tx with remaining size
    let remaining_size = size.saturating_sub(tx_hash.len() + sender.len() + 20);
    let raw_tx = format!(
        "0x{}",
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(remaining_size)
            .map(char::from)
            .collect::<String>()
    );

    #[allow(clippy::unwrap_used)]
    TransactionRequest {
        tx_hash,
        sender,
        raw_tx,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test is ignored by default since it's a benchmark
    // Run with: cargo test --package maestro --test integration -- redis_benchmark --ignored
    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn test_redis_benchmark() {
        // Initialize logging if needed
        let _ = tracing_subscriber::fmt().with_env_filter("info").try_init();

        let results = run_benchmark_with_redis().await.expect("Benchmark failed");

        println!("{}", results);

        // Add assertions to verify minimum performance
        assert!(results.enqueue_tps > 100.0, "Enqueue throughput too low");
        assert!(results.dequeue_tps > 100.0, "Dequeue throughput too low");
        assert!(results.avg_enqueue_latency_us < 5000.0, "Enqueue latency too high");
        assert!(results.avg_dequeue_latency_us < 5000.0, "Dequeue latency too high");
    }

    // Test with high producer count, low consumer count (to simulate backpressure)
    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn test_redis_benchmark_backpressure() {
        // Initialize logging if needed
        let _ = tracing_subscriber::fmt().with_env_filter("info").try_init();

        let port = PortManager::instance().next_port();
        let config = BenchmarkConfig {
            producer_count: 8,
            consumer_count: 2,
            redis_port: port,
            ..Default::default()
        };

        let _redis = start_redis(port).await.expect("Failed to start Redis");
        let results = run_benchmark(config).await.expect("Benchmark failed");

        println!("{}", results);

        // Verify we produced more than we consumed (backpressure)
        assert!(
            results.transactions_produced > results.transactions_consumed,
            "Expected producer to outpace consumer in backpressure test"
        );
    }

    // Test with low producer count, high consumer count (to ensure complete processing)
    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn test_redis_benchmark_complete_processing() {
        // Initialize logging if needed
        let _ = tracing_subscriber::fmt().with_env_filter("info").try_init();

        let port = PortManager::instance().next_port();

        // Configure with more consumers than producers
        let config = BenchmarkConfig {
            producer_count: 2,
            consumer_count: 8,
            transaction_count: 5000, // Reduced count for faster test completion
            duration_seconds: 15,    // Give enough time for complete processing
            redis_port: port,
            ..Default::default()
        };

        let _redis = start_redis(port).await.expect("Failed to start Redis");
        let results = run_benchmark(config).await.expect("Benchmark failed");

        println!("{}", results);

        // Verify that all or most transactions were consumed
        let consumption_ratio =
            results.transactions_consumed as f64 / results.transactions_produced as f64;

        println!("Consumption ratio: {:.2}", consumption_ratio);

        // We expect at least 95% of the produced transactions to be consumed
        assert!(
            consumption_ratio > 0.95,
            "Expected almost all transactions to be consumed, but ratio was only {:.2}",
            consumption_ratio
        );

        // Verify that the dequeue throughput is higher than the enqueue throughput
        assert!(
            results.dequeue_tps > results.enqueue_tps,
            "Expected dequeue throughput ({:.2}) to exceed enqueue throughput ({:.2})",
            results.dequeue_tps,
            results.enqueue_tps
        );

        // Verify that dequeue latency is acceptable
        assert!(
            results.avg_dequeue_latency_us < 2000.0,
            "Expected dequeue latency to be under 2ms, but was {:.2} μs",
            results.avg_dequeue_latency_us
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    #[ignore]
    async fn test_redis_stream_correctness() {
        // Initialize detailed logging for debugging
        let _ = tracing_subscriber::fmt().with_env_filter("debug").try_init();

        // Get a port for Redis
        let port = PortManager::instance().next_port();

        // Create a simple configuration with minimal tasks
        let config = BenchmarkConfig {
            producer_count: 1,
            consumer_count: 2,
            transaction_count: 10, // Just 10 transactions
            transaction_size: 128, // Smaller transactions for quicker testing
            duration_seconds: 5,   // Short duration
            redis_port: port,
        };

        // Start Redis
        let _redis = start_redis(port).await.expect("Failed to start Redis");
        let results = run_benchmark(config).await.expect("Benchmark failed");

        println!("{}", results);

        // Assert correctness
        assert_eq!(results.transactions_produced, 10, "Should have produced 10 messages");
        assert_eq!(results.transactions_consumed, 10, "Should have consumed 10 messages");
    }
}
