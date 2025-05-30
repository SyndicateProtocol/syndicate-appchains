#![allow(missing_docs)]

use prometheus_client::{metrics::gauge::Gauge, registry::Registry};
use std::time::Duration;

#[allow(missing_docs)]
#[derive(Debug, Default, Clone)]
pub struct BatcherMetrics {
    pub batch_transactions_count: Gauge,
    pub batch_size_bytes: Gauge,
    pub batch_compression_ratio: Gauge,
    pub batch_processing_time_ms: Gauge,
    pub batch_submission_latency_ms: Gauge,
    pub total_txs_processed: Gauge,
    pub wallet_balance: Gauge,
}

impl BatcherMetrics {
    pub fn new(registry: &mut Registry) -> Self {
        let metrics = Self {
            batch_transactions_count: Gauge::default(),
            batch_size_bytes: Gauge::default(),
            batch_compression_ratio: Gauge::default(),
            batch_processing_time_ms: Gauge::default(),
            batch_submission_latency_ms: Gauge::default(),
            total_txs_processed: Gauge::default(),
            wallet_balance: Gauge::default(),
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
            "total_txs_processed",
            "Total number of transactions processed",
            metrics.total_txs_processed.clone(),
        );

        registry.register(
            "wallet_balance",
            "Current wallet balance in wei",
            metrics.wallet_balance.clone(),
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

    pub fn record_compression_ratio(&self, original_size: usize, compressed_size: usize) {
        if original_size > 0 {
            let ratio = (original_size as f64 / compressed_size as f64 * 100.0) as i64;
            self.batch_compression_ratio.set(ratio);
            self.record_batch_size(compressed_size);
        }
    }

    pub fn record_processing_time(&self, duration: Duration) {
        self.batch_processing_time_ms.set(duration.as_millis() as i64);
    }

    pub fn record_submission_latency(&self, duration: Duration) {
        self.batch_submission_latency_ms.set(duration.as_millis() as i64);
    }

    pub fn increment_total_txs_processed(&self, count: usize) {
        self.total_txs_processed.inc_by(count as i64);
    }

    pub fn record_wallet_balance(&self, balance: u128) {
        self.wallet_balance.set(balance as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prometheus_client::registry::Registry;

    #[test]
    fn test_new_metrics_initialization() {
        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        assert_eq!(metrics.batch_transactions_count.get(), 0);
    }

    #[test]
    fn test_record_batch_transactions() {
        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        metrics.record_batch_transactions(10);
        assert_eq!(metrics.batch_transactions_count.get(), 10);
    }

    #[test]
    fn test_record_batch_size() {
        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        metrics.record_batch_size(1000);
        assert_eq!(metrics.batch_size_bytes.get(), 1000);
    }

    #[test]
    fn test_record_compression_ratio() {
        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        metrics.record_compression_ratio(1000, 500);
        assert_eq!(metrics.batch_compression_ratio.get(), 200);
    }

    #[test]
    fn test_record_processing_time() {
        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        metrics.record_processing_time(Duration::from_millis(100));
        assert_eq!(metrics.batch_processing_time_ms.get(), 100);
    }

    #[test]
    fn test_record_submission_latency() {
        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        metrics.record_submission_latency(Duration::from_millis(100));
        assert_eq!(metrics.batch_submission_latency_ms.get(), 100);
    }

    #[test]
    fn test_increment_total_txs_processed() {
        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        metrics.increment_total_txs_processed(10);
        assert_eq!(metrics.total_txs_processed.get(), 10);
    }

    #[test]
    fn test_record_wallet_balance() {
        let mut registry = Registry::default();
        let metrics = BatcherMetrics::new(&mut registry);

        metrics.record_wallet_balance(1000000000000000000);
        assert_eq!(metrics.wallet_balance.get(), 1000000000000000000);
    }
}
