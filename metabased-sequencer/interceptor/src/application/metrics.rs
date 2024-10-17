pub use prometheus::PrometheusMetrics;

use std::fmt::Write;

pub fn metrics(metrics: &impl Metrics) -> String {
    let mut response = String::new();

    metrics
        .encode(&mut response)
        .expect("Formatting to string should be infallible");

    response
}

pub trait Metrics {
    fn inc_send_raw_transaction(&self);
    fn encode(&self, writer: &mut impl Write) -> std::fmt::Result;
}

mod noop {
    use super::*;

    impl Metrics for () {
        fn inc_send_raw_transaction(&self) {}

        fn encode(&self, _writer: &mut impl Write) -> std::fmt::Result {
            Ok(())
        }
    }
}

mod prometheus {
    use super::*;
    use prometheus_client::encoding::text::encode;
    use prometheus_client::encoding::EncodeLabelSet;
    use prometheus_client::metrics::counter::Counter;
    use prometheus_client::metrics::family::Family;
    use prometheus_client::registry::Registry;

    #[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
    pub struct Labels {
        rpc_method: &'static str,
    }

    #[derive(Debug)]
    pub struct PrometheusMetrics {
        registry: Registry,
        rpc_calls: Family<Labels, Counter>,
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

            Self {
                registry,
                rpc_calls,
            }
        }
    }

    impl Metrics for PrometheusMetrics {
        fn inc_send_raw_transaction(&self) {
            self.rpc_calls
                .get_or_create(&Labels {
                    rpc_method: "eth_sendRawTransaction",
                })
                .inc();
        }

        fn encode(&self, writer: &mut impl Write) -> std::fmt::Result {
            encode(writer, &self.registry)
        }
    }
}
