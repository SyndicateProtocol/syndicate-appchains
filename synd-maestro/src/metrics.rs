#![allow(missing_docs)]

use crate::valkey::valkey_metrics::ValkeyMetrics;
use prometheus_client::{metrics::gauge::Gauge, registry::Registry};
use std::time::Duration;

/// Struct holding metrics related to the `synd-maestro` service.
#[derive(Debug, Default, Clone)]
pub struct MaestroMetrics {
    transaction_requests: Gauge,
    transaction_requests_duration_ms: Gauge,
    enqueued_transactions: Gauge,
    waiting_transactions: Gauge,
    successful_transactions: Gauge,
    resubmitted_transactions: Gauge,
    pub valkey: ValkeyMetrics,
}

impl MaestroMetrics {
    /// Create a new `MaestroMetrics` struct
    pub fn new(registry: &mut Registry) -> Self {
        let metrics = Self {
            transaction_requests : Gauge::default(),
            transaction_requests_duration_ms: Gauge::default(),
            enqueued_transactions: Gauge::default(),
            waiting_transactions: Gauge::default(),
            successful_transactions: Gauge::default(),
            resubmitted_transactions: Gauge::default(),
            valkey: ValkeyMetrics::new(registry),
        };

        registry.register(
            "transaction_requests ",
            "Total number of requests received by maestro",
            metrics.transaction_requests .clone(),
        );

        registry.register(
            "transaction_requests_duration_ms",
            "Time taken for request to complete, in milliseconds",
            metrics.transaction_requests_duration_ms.clone(),
        );

        registry.register(
            "enqueued_transactions",
            "Total number of transactions enqueued for sequencing",
            metrics.enqueued_transactions.clone(),
        );

        registry.register(
            "waiting_transactions",
            "Total number of waiting transactions saved for later submission",
            metrics.waiting_transactions.clone(),
        );

        registry.register(
            "successful_transactions",
            "Total number of successful transactions with tx_hash returned to caller",
            metrics.successful_transactions.clone(),
        );

        registry.register(
            "resubmitted_transactions",
            "Total number of transactions re-submitted to the stream",
            metrics.resubmitted_transactions.clone(),
        );

        metrics
    }

    pub fn increment_transaction_requests (&self, count: usize) {
        self.transaction_requests .inc_by(count as i64);
    }

    pub fn record_transaction_requests_duration_ms(&self, duration: Duration) {
        self.transaction_requests_duration_ms.set(duration.as_millis() as i64);
    }

    pub fn increment_enqueued_transactions(&self, count: usize) {
        self.enqueued_transactions.inc_by(count as i64);
    }

    pub fn increment_waiting_transactions(&self, count: usize) {
        self.waiting_transactions.inc_by(count as i64);
    }

    pub fn increment_successful_transactions(&self, count: usize) {
        self.successful_transactions.inc_by(count as i64);
    }

    pub fn increment_resubmitted_transactions(&self, count: usize) {
        self.resubmitted_transactions.inc_by(count as i64);
    }
}
