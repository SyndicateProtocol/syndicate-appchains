use std::{
    io::{stderr, Write},
    time::{Duration, SystemTime},
};

#[derive(Debug)]
pub(super) struct TestTimer(pub(crate) SystemTime, pub(crate) Duration);

impl Drop for TestTimer {
    fn drop(&mut self) {
        let thread = std::thread::current();
        let elapsed = format!(
            "---- SYN ---- Test {:?} took: {:.2?} (setup: {:.2?})\n",
            thread.name().unwrap_or_default(),
            self.0.elapsed().unwrap_or_default(),
            self.1,
        );
        // Write directly to stderr (bypasses test harness output capture)
        let _ = stderr().write_all(elapsed.as_bytes());
    }
}
