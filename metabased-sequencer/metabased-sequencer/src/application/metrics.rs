//! A module concerned with the orchestration of measurements and properties regarding application
//! usage.
//!
//! The metrics module contains:
//! * A query for collecting [`metrics`].
//! * A service that defines taking [`Metrics`].
//! * A generic [`Stopwatch`].

use crate::presentation::json_rpc_errors::Error;
use std::{
    fmt::{Display, Write},
    time::Duration,
};

/// Queries all collected metrics into textual representation and returns as a string.
pub fn metrics(metrics: &impl Metrics) -> String {
    metrics.to_string()
}

/// A service for collecting measurements of properties describing the application usage.
pub trait Metrics: Display {
    /// Increases the count of calls to `eth_sendRawTransaction` with response latency measurement.
    fn append_send_raw_transaction_with_duration(&self, duration: Duration, error: Option<&Error>);

    /// Encodes all the collected metrics into textual representation and outputs using `writer`.
    fn encode(&self, writer: &mut impl Write) -> std::fmt::Result;
}

/// The `RunningStopwatch` trait is a monotonically non-decreasing clock that measures time since
/// started.
pub trait RunningStopwatch {
    /// Returns a [`Duration`] since starting this [`RunningStopwatch`].
    fn elapsed(&self) -> Duration;
}

/// The `Stopwatch` trait creates [`RunningStopwatch`] by calling [`Stopwatch::start`].
pub trait Stopwatch {
    /// The associated [`RunningStopwatch`] type.
    type Running: RunningStopwatch;

    /// Creates a running stopwatch that measures time since created using this function call.
    fn start(&self) -> Self::Running;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Formatter;

    struct DummyMetrics(&'static str);

    impl Display for DummyMetrics {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            self.encode(f)
        }
    }

    impl Metrics for DummyMetrics {
        fn append_send_raw_transaction_with_duration(
            &self,
            _duration: Duration,
            _error: Option<&Error>,
        ) {
        }

        fn encode(&self, writer: &mut impl Write) -> std::fmt::Result {
            writer.write_str(self.0)
        }
    }

    #[test]
    fn test_metrics_query_collects_output_from_service() {
        let data = "test";
        let actual_metrics = metrics(&DummyMetrics(data));
        let expected_metrics = data.to_owned();

        assert_eq!(actual_metrics, expected_metrics);
    }
}
