//! The `metrics` module for the `Proposer`

use prometheus_client::{metrics::gauge::Gauge, registry::Registry};

/// Structure holding metrics related to the `Proposer`.
#[derive(Debug, Clone)]
pub struct ProposerMetrics {
    /// Last posted block number
    pub last_posted_block_number: Gauge,
    /// Current wallet balance in wei
    pub wallet_balance: Gauge,
}

impl ProposerMetrics {
    /// Creates a new `ProposerMetrics` instance and registers metrics in the provided registry.
    pub fn new(registry: &mut Registry) -> Self {
        let last_posted_block_number = Gauge::default();
        let wallet_balance = Gauge::default();

        registry.register(
            "last_posted_block_number",
            "Tracks the last posted block number",
            last_posted_block_number.clone(),
        );

        registry.register(
            "wallet_balance",
            "Current wallet balance in wei",
            wallet_balance.clone(),
        );

        Self { last_posted_block_number, wallet_balance }
    }

    /// Updates the last posted block number.
    pub fn record_last_block_posted(&self, number: u64) {
        self.last_posted_block_number.set(number as i64);
    }

    /// Updates the current wallet balance.
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
        let metrics = ProposerMetrics::new(&mut registry);

        assert_eq!(metrics.last_posted_block_number.get(), 0);
    }

    #[test]
    fn test_record_last_block_posted() {
        let mut registry = Registry::default();
        let metrics = ProposerMetrics::new(&mut registry);

        metrics.record_last_block_posted(10);
        assert_eq!(metrics.last_posted_block_number.get(), 10);
    }

    #[test]
    fn test_record_wallet_balance() {
        let mut registry = Registry::default();
        let metrics = ProposerMetrics::new(&mut registry);

        metrics.record_wallet_balance(1000000000000000000);
        assert_eq!(metrics.wallet_balance.get(), 1000000000000000000);
    }
}
