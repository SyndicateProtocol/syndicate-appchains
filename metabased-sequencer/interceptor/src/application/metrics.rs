pub use prometheus::PrometheusMetrics;

use std::fmt::Write;
use std::time::Duration;

pub fn metrics(metrics: &impl Metrics) -> String {
    let mut response = String::new();

    metrics
        .encode(&mut response)
        .expect("Formatting to string should be infallible");

    response
}

pub trait Metrics {
    /// Increases the count of calls to `eth_sendRawTransaction` with response latency measurement.
    fn append_send_raw_transaction_with_duration(&self, duration: Duration);
    fn encode(&self, writer: &mut impl Write) -> std::fmt::Result;
}

pub trait RunningStopwatch {
    fn elapsed(&self) -> Duration;
}

pub trait Stopwatch {
    type Running: RunningStopwatch;

    fn start(&self) -> Self::Running;
}

#[cfg(test)]
mod noop {
    use super::*;

    pub type NoopMetrics = ();

    impl Metrics for NoopMetrics {
        fn append_send_raw_transaction_with_duration(&self, _duration: Duration) {}

        fn encode(&self, _writer: &mut impl Write) -> std::fmt::Result {
            Ok(())
        }
    }

    pub type NoopStopwatch = ();

    impl RunningStopwatch for NoopStopwatch {
        fn elapsed(&self) -> Duration {
            Duration::ZERO
        }
    }

    impl Stopwatch for NoopStopwatch {
        type Running = ();

        fn start(&self) -> Self::Running {}
    }
}

mod prometheus {
    use super::*;
    use prometheus_client::encoding::text::encode;
    use prometheus_client::encoding::EncodeLabelSet;
    use prometheus_client::metrics::counter::Counter;
    use prometheus_client::metrics::family::Family;
    use prometheus_client::metrics::histogram::{exponential_buckets, Histogram};
    use prometheus_client::registry::Registry;

    #[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
    pub struct Labels {
        rpc_method: &'static str,
    }

    #[derive(Debug)]
    pub struct PrometheusMetrics {
        registry: Registry,
        rpc_calls: Family<Labels, Counter>,
        rpc_calls_duration: Family<Labels, Histogram>,
    }

    impl Default for PrometheusMetrics {
        fn default() -> Self {
            Self::new()
        }
    }

    impl PrometheusMetrics {
        pub fn new() -> Self {
            let mut registry = <Registry>::default();
            let rpc_calls = Family::<Labels, Counter>::default();
            registry.register(
                "rpc_calls",
                "Number of RPC method calls received",
                rpc_calls.clone(),
            );
            let rpc_calls_duration = Family::<Labels, Histogram>::new_with_constructor(|| {
                Histogram::new(exponential_buckets(1.0, 2.0, 10))
            });
            registry.register(
                "rpc_calls_latency",
                "Latency of RPC method calls responses",
                rpc_calls_duration.clone(),
            );

            Self {
                registry,
                rpc_calls,
                rpc_calls_duration,
            }
        }
    }

    impl Metrics for PrometheusMetrics {
        fn append_send_raw_transaction_with_duration(&self, duration: Duration) {
            self.rpc_calls
                .get_or_create(&Labels {
                    rpc_method: "eth_sendRawTransaction",
                })
                .inc();

            self.rpc_calls_duration
                .get_or_create(&Labels {
                    rpc_method: "eth_sendRawTransaction",
                })
                .observe(duration.as_secs_f64());
        }

        fn encode(&self, writer: &mut impl Write) -> std::fmt::Result {
            encode(writer, &self.registry)
        }
    }
}
