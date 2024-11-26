use crate::application::Metrics;
use prometheus_client::encoding::text::encode;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::histogram::{exponential_buckets, Histogram};
use prometheus_client::registry::Registry;
use std::fmt::{Display, Formatter, Write};
use std::time::Duration;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    rpc_method: &'static str,
}

#[derive(Debug)]
pub struct PrometheusMetrics {
    registry: Registry,
    rpc_calls: Family<Labels, Counter>,
    rpc_calls_duration: Family<Labels, Histogram>,
    rpc_calls_errors: Family<Labels, Counter>,
    rpc_calls_errors_duration: Family<Labels, Histogram>,
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
            Histogram::new(exponential_buckets(0.01, 2.0, 10))
        });
        registry.register(
            "rpc_calls_latency",
            "Latency of RPC method calls responses",
            rpc_calls_duration.clone(),
        );

        let rpc_calls_errors = Family::<Labels, Counter>::default();
        registry.register(
            "rpc_calls_errors",
            "Number of RPC method calls that failed",
            rpc_calls_errors.clone(),
        );
        let rpc_calls_errors_duration = Family::<Labels, Histogram>::new_with_constructor(|| {
            Histogram::new(exponential_buckets(0.01, 2.0, 10))
        });
        registry.register(
            "rpc_calls_errors_latency",
            "Latency of RPC method calls responses that failed",
            rpc_calls_errors_duration.clone(),
        );

        Self {
            registry,
            rpc_calls,
            rpc_calls_duration,
            rpc_calls_errors,
            rpc_calls_errors_duration,
        }
    }
}

impl Metrics for PrometheusMetrics {
    fn append_send_raw_transaction_with_duration(&self, duration: Duration, success: bool) {
        if success {
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
            return;
        }

        self.rpc_calls_errors
            .get_or_create(&Labels {
                rpc_method: "eth_sendRawTransaction",
            })
            .inc();

        self.rpc_calls_errors_duration
            .get_or_create(&Labels {
                rpc_method: "eth_sendRawTransaction",
            })
            .observe(duration.as_secs_f64());
    }

    fn encode(&self, writer: &mut impl Write) -> std::fmt::Result {
        encode(writer, &self.registry)
    }
}

impl Display for PrometheusMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.encode(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case([400], r#"# HELP rpc_calls Number of RPC method calls received.
# TYPE rpc_calls counter
rpc_calls_total{rpc_method="eth_sendRawTransaction"} 1
# HELP rpc_calls_latency Latency of RPC method calls responses.
# TYPE rpc_calls_latency histogram
rpc_calls_latency_sum{rpc_method="eth_sendRawTransaction"} 0.4
rpc_calls_latency_count{rpc_method="eth_sendRawTransaction"} 1
rpc_calls_latency_bucket{le="0.01",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.02",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.04",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.08",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.16",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.32",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.64",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_latency_bucket{le="1.28",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_latency_bucket{le="2.56",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_latency_bucket{le="5.12",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_latency_bucket{le="+Inf",rpc_method="eth_sendRawTransaction"} 1
# HELP rpc_calls_errors Number of RPC method calls that failed.
# TYPE rpc_calls_errors counter
rpc_calls_errors_total{rpc_method="eth_sendRawTransaction"} 1
# HELP rpc_calls_errors_latency Latency of RPC method calls responses that failed.
# TYPE rpc_calls_errors_latency histogram
rpc_calls_errors_latency_sum{rpc_method="eth_sendRawTransaction"} 0.4
rpc_calls_errors_latency_count{rpc_method="eth_sendRawTransaction"} 1
rpc_calls_errors_latency_bucket{le="0.01",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.02",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.04",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.08",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.16",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.32",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.64",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_errors_latency_bucket{le="1.28",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_errors_latency_bucket{le="2.56",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_errors_latency_bucket{le="5.12",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_errors_latency_bucket{le="+Inf",rpc_method="eth_sendRawTransaction"} 1
# EOF
"#; "One in middle bucket")]
    #[test_case([400, 600], r#"# HELP rpc_calls Number of RPC method calls received.
# TYPE rpc_calls counter
rpc_calls_total{rpc_method="eth_sendRawTransaction"} 2
# HELP rpc_calls_latency Latency of RPC method calls responses.
# TYPE rpc_calls_latency histogram
rpc_calls_latency_sum{rpc_method="eth_sendRawTransaction"} 1.0
rpc_calls_latency_count{rpc_method="eth_sendRawTransaction"} 2
rpc_calls_latency_bucket{le="0.01",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.02",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.04",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.08",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.16",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.32",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.64",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_latency_bucket{le="1.28",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_latency_bucket{le="2.56",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_latency_bucket{le="5.12",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_latency_bucket{le="+Inf",rpc_method="eth_sendRawTransaction"} 2
# HELP rpc_calls_errors Number of RPC method calls that failed.
# TYPE rpc_calls_errors counter
rpc_calls_errors_total{rpc_method="eth_sendRawTransaction"} 2
# HELP rpc_calls_errors_latency Latency of RPC method calls responses that failed.
# TYPE rpc_calls_errors_latency histogram
rpc_calls_errors_latency_sum{rpc_method="eth_sendRawTransaction"} 1.0
rpc_calls_errors_latency_count{rpc_method="eth_sendRawTransaction"} 2
rpc_calls_errors_latency_bucket{le="0.01",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.02",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.04",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.08",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.16",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.32",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.64",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_errors_latency_bucket{le="1.28",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_errors_latency_bucket{le="2.56",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_errors_latency_bucket{le="5.12",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_errors_latency_bucket{le="+Inf",rpc_method="eth_sendRawTransaction"} 2
# EOF
"#; "Multiple in same bucket")]
    #[test_case([1, 200], r#"# HELP rpc_calls Number of RPC method calls received.
# TYPE rpc_calls counter
rpc_calls_total{rpc_method="eth_sendRawTransaction"} 2
# HELP rpc_calls_latency Latency of RPC method calls responses.
# TYPE rpc_calls_latency histogram
rpc_calls_latency_sum{rpc_method="eth_sendRawTransaction"} 0.201
rpc_calls_latency_count{rpc_method="eth_sendRawTransaction"} 2
rpc_calls_latency_bucket{le="0.01",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_latency_bucket{le="0.02",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_latency_bucket{le="0.04",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_latency_bucket{le="0.08",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_latency_bucket{le="0.16",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_latency_bucket{le="0.32",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_latency_bucket{le="0.64",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_latency_bucket{le="1.28",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_latency_bucket{le="2.56",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_latency_bucket{le="5.12",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_latency_bucket{le="+Inf",rpc_method="eth_sendRawTransaction"} 2
# HELP rpc_calls_errors Number of RPC method calls that failed.
# TYPE rpc_calls_errors counter
rpc_calls_errors_total{rpc_method="eth_sendRawTransaction"} 2
# HELP rpc_calls_errors_latency Latency of RPC method calls responses that failed.
# TYPE rpc_calls_errors_latency histogram
rpc_calls_errors_latency_sum{rpc_method="eth_sendRawTransaction"} 0.201
rpc_calls_errors_latency_count{rpc_method="eth_sendRawTransaction"} 2
rpc_calls_errors_latency_bucket{le="0.01",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_errors_latency_bucket{le="0.02",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_errors_latency_bucket{le="0.04",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_errors_latency_bucket{le="0.08",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_errors_latency_bucket{le="0.16",rpc_method="eth_sendRawTransaction"} 1
rpc_calls_errors_latency_bucket{le="0.32",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_errors_latency_bucket{le="0.64",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_errors_latency_bucket{le="1.28",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_errors_latency_bucket{le="2.56",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_errors_latency_bucket{le="5.12",rpc_method="eth_sendRawTransaction"} 2
rpc_calls_errors_latency_bucket{le="+Inf",rpc_method="eth_sendRawTransaction"} 2
# EOF
"#; "Multiple in different buckets")]
    #[test_case([9999], r#"# HELP rpc_calls Number of RPC method calls received.
# TYPE rpc_calls counter
rpc_calls_total{rpc_method="eth_sendRawTransaction"} 1
# HELP rpc_calls_latency Latency of RPC method calls responses.
# TYPE rpc_calls_latency histogram
rpc_calls_latency_sum{rpc_method="eth_sendRawTransaction"} 9.999
rpc_calls_latency_count{rpc_method="eth_sendRawTransaction"} 1
rpc_calls_latency_bucket{le="0.01",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.02",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.04",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.08",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.16",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.32",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="0.64",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="1.28",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="2.56",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="5.12",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_latency_bucket{le="+Inf",rpc_method="eth_sendRawTransaction"} 1
# HELP rpc_calls_errors Number of RPC method calls that failed.
# TYPE rpc_calls_errors counter
rpc_calls_errors_total{rpc_method="eth_sendRawTransaction"} 1
# HELP rpc_calls_errors_latency Latency of RPC method calls responses that failed.
# TYPE rpc_calls_errors_latency histogram
rpc_calls_errors_latency_sum{rpc_method="eth_sendRawTransaction"} 9.999
rpc_calls_errors_latency_count{rpc_method="eth_sendRawTransaction"} 1
rpc_calls_errors_latency_bucket{le="0.01",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.02",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.04",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.08",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.16",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.32",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="0.64",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="1.28",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="2.56",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="5.12",rpc_method="eth_sendRawTransaction"} 0
rpc_calls_errors_latency_bucket{le="+Inf",rpc_method="eth_sendRawTransaction"} 1
# EOF
"#; "One in bucket beyond all")]
    fn test_collecting_metrics_records_all_durations(
        durations: impl IntoIterator<Item = u64>,
        expected_output: impl Into<String>,
    ) {
        let metrics = PrometheusMetrics::default();

        for millis in durations {
            metrics.append_send_raw_transaction_with_duration(Duration::from_millis(millis), true);
            metrics.append_send_raw_transaction_with_duration(Duration::from_millis(millis), false);
        }

        let actual_output = metrics.to_string();
        let expected_output = expected_output.into();

        assert_eq!(actual_output, expected_output);
    }
}
