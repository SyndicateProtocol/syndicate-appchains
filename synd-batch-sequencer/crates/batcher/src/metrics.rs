use prometheus_client::{metrics::gauge::Gauge, registry::Registry};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Default, Clone)]
pub struct BatcherMetrics {
    pub batch_transactions_count: Gauge,
    pub batch_size_bytes: Gauge,
    pub batch_timestamp: Gauge,
    pub batch_compression_ratio: Gauge,
    pub batch_processing_time_ms: Gauge,
    pub batch_submission_latency_ms: Gauge,
    pub batch_errors: Gauge,
    pub redis_read_latency_ms: Gauge,
    pub total_txs_processed: Gauge,
}

impl BatcherMetrics {
    pub fn new(registry: &mut Registry) -> Self {
        let metrics = Self {
            batch_transactions_count: Gauge::default(),
            batch_size_bytes: Gauge::default(),
            batch_timestamp: Gauge::default(),
            batch_compression_ratio: Gauge::default(),
            batch_processing_time_ms: Gauge::default(),
            batch_submission_latency_ms: Gauge::default(),
            batch_errors: Gauge::default(),
            redis_read_latency_ms: Gauge::default(),
            total_txs_processed: Gauge::default(),
        };

        registry.register(
            "batch_transactions_count",
            "Number of transactions in the current batch",
            metrics.batch_transactions_count.clone(),
        );

        registry.register(
            "batch_size_bytes",
            "Size of the current batch in bytes",
            metrics.batch_size_bytes.clone(),
        );

        registry.register(
            "batch_timestamp",
            "Timestamp of the last batch submission",
            metrics.batch_timestamp.clone(),
        );

        registry.register(
            "batch_compression_ratio",
            "Compression ratio of the current batch",
            metrics.batch_compression_ratio.clone(),
        );

        registry.register(
            "batch_processing_time_ms",
            "Time taken to process the current batch in milliseconds",
            metrics.batch_processing_time_ms.clone(),
        );

        registry.register(
            "batch_submission_latency_ms",
            "Time taken to submit the batch in milliseconds",
            metrics.batch_submission_latency_ms.clone(),
        );

        registry.register(
            "batch_errors",
            "Number of batch processing errors",
            metrics.batch_errors.clone(),
        );

        registry.register(
            "redis_read_latency_ms",
            "Time taken to read from Redis in milliseconds",
            metrics.redis_read_latency_ms.clone(),
        );

        registry.register(
            "total_txs_processed",
            "Total number of transactions processed",
            metrics.total_txs_processed.clone(),
        );

        metrics
    }

    pub fn record_batch_transactions(&self, count: usize) {
        self.batch_transactions_count.set(count as i64);
        self.increment_total_txs_processed(count);
    }

    pub fn record_batch_size(&self, size: usize) {
        self.batch_size_bytes.set(size as i64);
    }

    pub fn record_batch_timestamp(&self) {
        self.batch_timestamp.set(
            SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0),
        );
    }

    pub fn record_compression_ratio(&self, original_size: usize, compressed_size: usize) {
        if original_size > 0 {
            let ratio = (original_size as f64 / compressed_size as f64 * 100.0) as i64;
            self.batch_compression_ratio.set(ratio);
        }
    }

    pub fn record_processing_time(&self, duration: Duration) {
        self.batch_processing_time_ms.set(duration.as_millis() as i64);
    }

    pub fn record_submission_latency(&self, duration: Duration) {
        self.batch_submission_latency_ms.set(duration.as_millis() as i64);
    }

    pub fn record_redis_latency(&self, duration: Duration) {
        self.redis_read_latency_ms.set(duration.as_millis() as i64);
    }

    pub fn increment_errors(&self) {
        self.batch_errors.inc();
    }

    pub fn increment_total_txs_processed(&self, count: usize) {
        self.total_txs_processed.inc_by(count as i64);
    }
}
