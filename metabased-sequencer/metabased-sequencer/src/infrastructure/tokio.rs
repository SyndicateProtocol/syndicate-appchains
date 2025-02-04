use crate::application::{RunningStopwatch, Stopwatch};
use std::time::Duration;
use tokio::time::Instant;

#[derive(Debug)]
pub struct TokioStopwatch;

impl RunningStopwatch for Instant {
    fn elapsed(&self) -> Duration {
        Instant::elapsed(self)
    }
}

impl Stopwatch for TokioStopwatch {
    type Running = Instant;

    fn start(&self) -> Self::Running {
        Instant::now()
    }
}
