use crate::application::Metrics;
use crate::presentation::json_rpc_errors::Error;
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
    error_category: &'static str,
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
            Histogram::new(exponential_buckets(0.01, 2.0, 10))
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

// Grouping errors into categories to reduce Prometheus metric cardinality
pub fn error_to_metric_category(error: Option<&Error>) -> &'static str {
    match error {
        None => "none",
        Some(error) => match error {
            Error::InvalidRequest | Error::Parse | Error::InvalidInput(_) => "validation_error",
            Error::MethodNotFound(_) | Error::MethodNotSupported => "method_error",
            Error::ResourceNotFound | Error::ResourceUnavailable => "resource_error",
            Error::Internal | Error::Server => "server_error",
            Error::Contract(_) => "contract_error",
            Error::InvalidParams(_) => "params_error",
            Error::TransactionRejected(_) => "tx_error",
            Error::LimitExceeded => "limit_error",
        },
    }
}

impl Metrics for PrometheusMetrics {
    fn append_send_raw_transaction_with_duration(&self, duration: Duration, error: Option<&Error>) {
        // Map the error string to a static Prometheus label
        let error_category = error_to_metric_category(error);

        // Log full details of error
        if let Some(err) = error {
            tracing::error!(
                method = "eth_sendRawTransaction",
                error_category = error_category,
                detailed_error = ?err,  // Debug format for full error details
                "Transaction error occurred"
            );
        }

        self.rpc_calls
            .get_or_create(&Labels {
                rpc_method: "eth_sendRawTransaction",
                error_category,
            })
            .inc();

        self.rpc_calls_duration
            .get_or_create(&Labels {
                rpc_method: "eth_sendRawTransaction",
                error_category,
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
rpc_calls_total{rpc_method="eth_sendRawTransaction",error_category="none"} 1
# HELP rpc_calls_latency Latency of RPC method calls responses.
# TYPE rpc_calls_latency histogram
rpc_calls_latency_sum{rpc_method="eth_sendRawTransaction",error_category="none"} 0.4
rpc_calls_latency_count{rpc_method="eth_sendRawTransaction",error_category="none"} 1
rpc_calls_latency_bucket{le="0.01",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.02",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.04",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.08",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.16",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.32",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.64",rpc_method="eth_sendRawTransaction",error_category="none"} 1
rpc_calls_latency_bucket{le="1.28",rpc_method="eth_sendRawTransaction",error_category="none"} 1
rpc_calls_latency_bucket{le="2.56",rpc_method="eth_sendRawTransaction",error_category="none"} 1
rpc_calls_latency_bucket{le="5.12",rpc_method="eth_sendRawTransaction",error_category="none"} 1
rpc_calls_latency_bucket{le="+Inf",rpc_method="eth_sendRawTransaction",error_category="none"} 1
# EOF
"#; "One in middle bucket")]
    #[test_case([400, 600], r#"# HELP rpc_calls Number of RPC method calls received.
# TYPE rpc_calls counter
rpc_calls_total{rpc_method="eth_sendRawTransaction",error_category="none"} 2
# HELP rpc_calls_latency Latency of RPC method calls responses.
# TYPE rpc_calls_latency histogram
rpc_calls_latency_sum{rpc_method="eth_sendRawTransaction",error_category="none"} 1.0
rpc_calls_latency_count{rpc_method="eth_sendRawTransaction",error_category="none"} 2
rpc_calls_latency_bucket{le="0.01",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.02",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.04",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.08",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.16",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.32",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.64",rpc_method="eth_sendRawTransaction",error_category="none"} 2
rpc_calls_latency_bucket{le="1.28",rpc_method="eth_sendRawTransaction",error_category="none"} 2
rpc_calls_latency_bucket{le="2.56",rpc_method="eth_sendRawTransaction",error_category="none"} 2
rpc_calls_latency_bucket{le="5.12",rpc_method="eth_sendRawTransaction",error_category="none"} 2
rpc_calls_latency_bucket{le="+Inf",rpc_method="eth_sendRawTransaction",error_category="none"} 2
# EOF
"#; "Multiple in same bucket")]
    #[test_case([1, 200], r#"# HELP rpc_calls Number of RPC method calls received.
# TYPE rpc_calls counter
rpc_calls_total{rpc_method="eth_sendRawTransaction",error_category="none"} 2
# HELP rpc_calls_latency Latency of RPC method calls responses.
# TYPE rpc_calls_latency histogram
rpc_calls_latency_sum{rpc_method="eth_sendRawTransaction",error_category="none"} 0.201
rpc_calls_latency_count{rpc_method="eth_sendRawTransaction",error_category="none"} 2
rpc_calls_latency_bucket{le="0.01",rpc_method="eth_sendRawTransaction",error_category="none"} 1
rpc_calls_latency_bucket{le="0.02",rpc_method="eth_sendRawTransaction",error_category="none"} 1
rpc_calls_latency_bucket{le="0.04",rpc_method="eth_sendRawTransaction",error_category="none"} 1
rpc_calls_latency_bucket{le="0.08",rpc_method="eth_sendRawTransaction",error_category="none"} 1
rpc_calls_latency_bucket{le="0.16",rpc_method="eth_sendRawTransaction",error_category="none"} 1
rpc_calls_latency_bucket{le="0.32",rpc_method="eth_sendRawTransaction",error_category="none"} 2
rpc_calls_latency_bucket{le="0.64",rpc_method="eth_sendRawTransaction",error_category="none"} 2
rpc_calls_latency_bucket{le="1.28",rpc_method="eth_sendRawTransaction",error_category="none"} 2
rpc_calls_latency_bucket{le="2.56",rpc_method="eth_sendRawTransaction",error_category="none"} 2
rpc_calls_latency_bucket{le="5.12",rpc_method="eth_sendRawTransaction",error_category="none"} 2
rpc_calls_latency_bucket{le="+Inf",rpc_method="eth_sendRawTransaction",error_category="none"} 2
# EOF
"#; "Multiple in different buckets")]
    #[test_case([9999], r#"# HELP rpc_calls Number of RPC method calls received.
# TYPE rpc_calls counter
rpc_calls_total{rpc_method="eth_sendRawTransaction",error_category="none"} 1
# HELP rpc_calls_latency Latency of RPC method calls responses.
# TYPE rpc_calls_latency histogram
rpc_calls_latency_sum{rpc_method="eth_sendRawTransaction",error_category="none"} 9.999
rpc_calls_latency_count{rpc_method="eth_sendRawTransaction",error_category="none"} 1
rpc_calls_latency_bucket{le="0.01",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.02",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.04",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.08",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.16",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.32",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="0.64",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="1.28",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="2.56",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="5.12",rpc_method="eth_sendRawTransaction",error_category="none"} 0
rpc_calls_latency_bucket{le="+Inf",rpc_method="eth_sendRawTransaction",error_category="none"} 1
# EOF
"#; "One in bucket beyond all")]
    fn test_collecting_metrics_records_all_durations(
        durations: impl IntoIterator<Item = u64>,
        expected_output: impl Into<String>,
    ) {
        let metrics = PrometheusMetrics::default();

        for millis in durations {
            metrics.append_send_raw_transaction_with_duration(Duration::from_millis(millis), None);
        }

        let actual_output = metrics.to_string();
        let expected_output = expected_output.into();

        assert_eq!(actual_output, expected_output);
    }
}
