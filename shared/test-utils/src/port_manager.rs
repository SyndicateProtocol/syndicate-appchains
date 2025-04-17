//! A singleton that manages port allocation for the test suite

use std::{
    net::TcpListener,
    sync::{
        atomic::{AtomicU16, Ordering},
        OnceLock,
    },
};

const MAX_PORTS: u16 = 65534;

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

    /// Returns the next available port
    pub fn next_port(&self) -> u16 {
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 100;

        while attempts < MAX_ATTEMPTS {
            let next = self.next_port.fetch_add(1, Ordering::SeqCst);
            assert!(next <= MAX_PORTS, "Port range exhausted");

            if is_port_available(next) {
                return next;
            }

            attempts += 1;
        }

        panic!("Failed to acquire port after {} attempts", MAX_ATTEMPTS);
    }
}
