#![allow(missing_docs)]

use prometheus_client::{metrics::gauge::Gauge, registry::Registry};
use std::time::Duration;

/// Struct holding metrics related to the `synd-maestro` service.
#[derive(Debug, Default, Clone)]
pub struct MaestroMetrics {
    maestro_requests_total: Gauge,
    maestro_requests_duration_ms: Gauge,
    maestro_enqueued_transactions_total: Gauge,
    maestro_waiting_transactions_total: Gauge,
    maestro_successful_transactions_total: Gauge,
    maestro_resubmitted_transactions_total: Gauge,
}

impl MaestroMetrics {
    /// Create a new `MaestroMetrics` struct
    pub fn new(registry: &mut Registry) -> Self {
        let metrics = Self {
            maestro_requests_total: Gauge::default(),
            maestro_requests_duration_ms: Gauge::default(),
            maestro_enqueued_transactions_total: Gauge::default(),
            maestro_waiting_transactions_total: Gauge::default(),
            maestro_successful_transactions_total: Gauge::default(),
            maestro_resubmitted_transactions_total: Gauge::default(),
        };

        registry.register(
            "maestro_requests_total",
            "Total number of requests received by maestro",
            metrics.maestro_requests_total.clone(),
        );

        registry.register(
            "maestro_requests_duration_ms",
            "Time taken for request to complete, in milliseconds",
            metrics.maestro_requests_duration_ms.clone(),
        );

        registry.register(
            "maestro_enqueued_transactions_total",
            "Total number of transactions enqueued for sequencing",
            metrics.maestro_enqueued_transactions_total.clone(),
        );

        registry.register(
            "maestro_waiting_transactions_total",
            "Total number of waiting transactions saved for later submission",
            metrics.maestro_waiting_transactions_total.clone(),
        );

        registry.register(
            "maestro_successful_transactions_total",
            "Total number of successful transactions with tx_hash returned to caller",
            metrics.maestro_successful_transactions_total.clone(),
        );

        registry.register(
            "maestro_resubmitted_transactions_total",
            "Total number of transactions re-submitted to the stream",
            metrics.maestro_resubmitted_transactions_total.clone(),
        );

        metrics
    }

    pub fn increment_maestro_requests_total(&self, count: usize) {
        self.maestro_requests_total.inc_by(count as i64);
    }

    pub fn record_maestro_requests_duration_ms(&self, duration: Duration) {
        self.maestro_requests_duration_ms.set(duration.as_millis() as i64);
    }

    pub fn increment_maestro_enqueued_transactions_total(&self, count: usize) {
        self.maestro_enqueued_transactions_total.inc_by(count as i64);
    }

    pub fn increment_maestro_waiting_transactions_total(&self, count: usize) {
        self.maestro_waiting_transactions_total.inc_by(count as i64);
    }

    pub fn increment_maestro_successful_transactions_total(&self, count: usize) {
        self.maestro_successful_transactions_total.inc_by(count as i64);
    }

    pub fn increment_maestro_resubmitted_transactions_total(&self, count: usize) {
        self.maestro_resubmitted_transactions_total.inc_by(count as i64);
    }
}
