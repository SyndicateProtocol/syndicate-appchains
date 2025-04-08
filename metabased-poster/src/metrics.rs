//! The `metrics` module for the `Poster`

use prometheus_client::{metrics::gauge::Gauge, registry::Registry};

/// Structure holding metrics related to the `Poster`.
#[derive(Debug, Clone)]
pub struct PosterMetrics {
    /// Last posted block number
    pub last_posted_block_number: Gauge,
}

impl PosterMetrics {
    /// Creates a new `PosterMetrics` instance and registers metrics in the provided registry.
    pub fn new(registry: &mut Registry) -> Self {
        let last_posted_block_number = Gauge::default();

        registry.register(
            "last_posted_block_number",
            "Tracks the last posted block number",
            last_posted_block_number.clone(),
        );

        Self { last_posted_block_number }
    }

    /// Updates the last posted block number.
    pub fn record_last_block_posted(&self, number: u64) {
        self.last_posted_block_number.set(number as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prometheus_client::registry::Registry;

    #[test]
    fn test_new_metrics_initialization() {
        let mut registry = Registry::default();
        let metrics = PosterMetrics::new(&mut registry);

        assert_eq!(metrics.last_posted_block_number.get(), 0);
    }

    #[test]
    fn test_record_last_block_posted() {
        let mut registry = Registry::default();
        let metrics = PosterMetrics::new(&mut registry);

        metrics.record_last_block_posted(10);
        assert_eq!(metrics.last_posted_block_number.get(), 10);
    }
}
