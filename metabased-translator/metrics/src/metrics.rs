use prometheus_client::{
    encoding::EncodeLabelSet,
    metrics::{
        counter::Counter,
        family::Family,
        gauge::Gauge,
        histogram::{exponential_buckets, Histogram},
    },
    registry::Registry,
};
use std::{error::Error, time::Duration};

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    crate_name: String,
    method: &'static str,
    error_category: &'static str,
}

#[derive(Debug)]
pub struct Metrics {
    registry: Registry,
    rpc_calls: Family<Labels, Counter>,
    rpc_calls_duration: Family<Labels, Histogram>,
    last_block_fetched: Family<Labels, Gauge>,
}

impl Metrics {
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn new() -> Self {
        let mut registry = <Registry>::default();

        let rpc_calls = Family::<Labels, Counter>::default();
        registry.register("rpc_calls", "Number of RPC method calls done", rpc_calls.clone());

        let rpc_calls_duration = Family::<Labels, Histogram>::new_with_constructor(|| {
            Histogram::new(exponential_buckets(0.01, 2.0, 10))
        });
        registry.register(
            "rpc_calls_latency",
            "Latency of RPC method call responses",
            rpc_calls_duration.clone(),
        );

        let last_block_fetched = Family::<Labels, Gauge>::default();
        registry.register(
            "last_block_fetched",
            "Tracks the last block number fetched for a specific RPC URL",
            last_block_fetched.clone(),
        );

        Self { registry, rpc_calls, rpc_calls_duration, last_block_fetched }
    }

    pub fn record_last_block_fetched(&self, crate_name: String, block_number: u64) {
        self.last_block_fetched
            .get_or_create(&Labels {
                crate_name,
                method: "last_block_fetched",
                error_category: "none",
            })
            .set(block_number as i64);
    }

    pub fn record_rpc_call(
        &self,
        crate_name: String,
        method: &'static str,
        duration: Duration,
        error: Option<&dyn Error>,
    ) {
        // Convert the error into a category string
        let error_category: &'static str = match error {
            Some(_) => "error", // Replace this with specific error categorization if needed
            None => "none",
        };

        let name = crate_name.clone();
        // Increment the counter for the RPC method call
        self.rpc_calls.get_or_create(&Labels { crate_name: name, method, error_category }).inc();

        // Observe the latency of the RPC method call
        self.rpc_calls_duration
            .get_or_create(&Labels { crate_name, method, error_category })
            .observe(duration.as_secs_f64());
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
