use crate::application::Metrics;
use prometheus_client::encoding::text::encode;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::histogram::{exponential_buckets, Histogram};
use prometheus_client::registry::Registry;
use std::fmt::{Display, Formatter, Write};
use std::time::Duration;
use crate::presentation::json_rpc_errors::{Error, InvalidInputError, InvalidParamsError, Rejection};

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    rpc_method: &'static str,
    error: &'static str,
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

    // // mapping for Prometheus
    // pub fn error_to_static_str(error: Option<&Error>) -> &'static str {
    //     match error {
    //         None => "none",
    //         Some(error) => match error {
    //             Error::InvalidRequest => "invalid_request",
    //             Error::MethodNotFound(ref _method) => "method_not_found",  // Note: We lose the specific method name here
    //             Error::Internal => "internal_error",
    //             Error::Parse => "parse_error",
    //             Error::ResourceNotFound => "resource_not_found",
    //             Error::ResourceUnavailable => "resource_unavailable",
    //             Error::MethodNotSupported => "method_not_supported",
    //             Error::LimitExceeded => "limit_exceeded",
    //             Error::Server => "server_error",
    //
    //             Error::Contract(ref _contract_error) => "contract_error",
    //
    //             Error::InvalidParams(param_error) => match param_error {
    //                 InvalidParamsError::BadSignature => "invalid_params.bad_signature",
    //                 InvalidParamsError::NonceTooLow => "invalid_params.nonce_too_low",
    //                 InvalidParamsError::InvalidHex => "invalid_params.invalid_hex",
    //                 InvalidParamsError::NotAnArray => "invalid_params.not_array",
    //                 InvalidParamsError::WrongParamCount(_) => "invalid_params.wrong_count",
    //                 InvalidParamsError::MissingParam => "invalid_params.missing_param",
    //                 InvalidParamsError::NotHexEncoded => "invalid_params.not_hex_encoded",
    //             },
    //
    //             Error::InvalidInput(input_error) => match input_error {
    //                 InvalidInputError::InvalidJson => "invalid_input.invalid_json",
    //                 InvalidInputError::InvalidUint => "invalid_input.invalid_uint",
    //                 InvalidInputError::InvalidTransactionSignature => "invalid_input.invalid_tx_signature",
    //                 InvalidInputError::MissingGasPrice => "invalid_input.missing_gas_price",
    //                 InvalidInputError::UnableToRLPDecode => "invalid_input.rlp_decode_error",
    //             },
    //
    //             Error::TransactionRejected(rejection) => match rejection {
    //                 Rejection::FeeTooHigh => "tx_rejected.fee_too_high",
    //             },
    //         }
    //     }
    // }
}
pub fn error_to_static_str(error: Option<&Error>) -> &'static str {
    match error {
        None => "none",
        Some(error) => match error {
            Error::InvalidRequest => "invalid_request",
            Error::MethodNotFound(ref _method) => "method_not_found",  // Note: We lose the specific method name here
            Error::Internal => "internal_error",
            Error::Parse => "parse_error",
            Error::ResourceNotFound => "resource_not_found",
            Error::ResourceUnavailable => "resource_unavailable",
            Error::MethodNotSupported => "method_not_supported",
            Error::LimitExceeded => "limit_exceeded",
            Error::Server => "server_error",

            Error::Contract(ref _contract_error) => "contract_error",

            Error::InvalidParams(param_error) => match param_error {
                InvalidParamsError::BadSignature => "invalid_params.bad_signature",
                InvalidParamsError::NonceTooLow => "invalid_params.nonce_too_low",
                InvalidParamsError::InvalidHex => "invalid_params.invalid_hex",
                InvalidParamsError::NotAnArray => "invalid_params.not_array",
                InvalidParamsError::WrongParamCount(_) => "invalid_params.wrong_count",
                InvalidParamsError::MissingParam => "invalid_params.missing_param",
                InvalidParamsError::NotHexEncoded => "invalid_params.not_hex_encoded",
            },

            Error::InvalidInput(input_error) => match input_error {
                InvalidInputError::InvalidJson => "invalid_input.invalid_json",
                InvalidInputError::InvalidUint => "invalid_input.invalid_uint",
                InvalidInputError::InvalidTransactionSignature => "invalid_input.invalid_tx_signature",
                InvalidInputError::MissingGasPrice => "invalid_input.missing_gas_price",
                InvalidInputError::UnableToRLPDecode => "invalid_input.rlp_decode_error",
            },

            Error::TransactionRejected(rejection) => match rejection {
                Rejection::FeeTooHigh => "tx_rejected.fee_too_high",
            },
        }
    }
}

impl Metrics for PrometheusMetrics {
    fn append_send_raw_transaction_with_duration(&self, duration: Duration, error: Option<&Error>) {
        // Map the error string to a static Prometheus label
        let error_label = error_to_static_str(error);

        self.rpc_calls
            .get_or_create(&Labels {
                rpc_method: "eth_sendRawTransaction",
                error: error_label,
            })
            .inc();

        self.rpc_calls_duration
            .get_or_create(&Labels {
                rpc_method: "eth_sendRawTransaction",
                error: error_label,
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
