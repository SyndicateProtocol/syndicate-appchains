//! A singleton that manages port allocation for the test suite

use std::{
    net::TcpListener,
    sync::{
        atomic::{AtomicU16, Ordering},
        OnceLock,
    },
};

/// Check if a port is available by attempting to bind to it
fn is_port_available(port: u16) -> bool {
    let addr = format!("127.0.0.1:{}", port);
    TcpListener::bind(addr).is_ok()
}

/// A singleton that manages port allocation for the test suite
#[derive(Debug)]
pub struct PortManager {
    next_port: AtomicU16,
}

/// A singleton that manages port allocation for the test suite
impl PortManager {
    /// Returns the singleton instance of the port manager
    pub fn instance() -> &'static Self {
        static MANAGER: OnceLock<PortManager> = OnceLock::new();
        MANAGER.get_or_init(|| Self { next_port: AtomicU16::new(8200) })
    }

    /// Acquires a range of ports
    pub fn acquire_port_range(&self, count: u16) -> Result<PortRange, PortError> {
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 100;

        while attempts < MAX_ATTEMPTS {
            let current = self.next_port.load(Ordering::Relaxed);
            let end = current.checked_add(count).ok_or(PortError::PortExhausted)?;

            // Check if any port in range is already in use
            let ports_available = (current..end).all(is_port_available);

            if ports_available {
                // Try to atomically claim the range
                if self
                    .next_port
                    .compare_exchange(current, end, Ordering::SeqCst, Ordering::Relaxed)
                    .is_ok()
                {
                    return Ok(PortRange { start: current, end });
                }
            }

            attempts += 1;
        }

        Err(PortError::NoAvailablePorts)
    }
}

/// A range of ports
#[derive(Debug)]
pub struct PortRange {
    start: u16,
    end: u16,
}

/// A range of ports
impl PortRange {
    /// Returns the next port in the range
    pub fn next_port(&mut self) -> Option<u16> {
        (self.start < self.end).then(|| {
            let port = self.start;
            self.start += 1;
            port
        })
    }
}

/// An error that occurs when managing ports
#[derive(Debug, thiserror::Error)]
pub enum PortError {
    /// No available ports after maximum attempts
    #[error("No available ports after maximum attempts")]
    NoAvailablePorts,
    /// Port range exhausted
    #[error("Port range exhausted")]
    PortExhausted,
}
