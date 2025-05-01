//! A singleton that manages port allocation for the test suite

use crate::wait_until;
use std::{
    fs::{self, OpenOptions},
    io::{self},
    net::TcpListener,
    path::PathBuf,
    sync::OnceLock,
    time::Duration,
};

const MAX_PORTS: u16 = 65534;
const DEFAULT_START_PORT: u16 = 8200;
const PORT_FILE_NAME: &str = ".test_ports";
const LOCK_FILE_NAME: &str = ".test_ports.lock";

/// Check if a port is available by attempting to bind to it
fn is_port_available(port: u16) -> bool {
    let addr = format!("127.0.0.1:{}", port);
    TcpListener::bind(addr).is_ok()
}

/// A singleton that manages port allocation for the test suite
#[derive(Debug)]
pub struct PortManager {
    port_file: PathBuf,
    lock_file: PathBuf,
}

/// A singleton that manages port allocation for the test suite
impl PortManager {
    /// Returns the singleton instance of the port manager
    pub fn instance() -> &'static Self {
        static MANAGER: OnceLock<PortManager> = OnceLock::new();
        MANAGER.get_or_init(|| {
            let temp_dir = std::env::temp_dir();
            let port_file = temp_dir.join(PORT_FILE_NAME);
            let lock_file = temp_dir.join(LOCK_FILE_NAME);
            Self { port_file, lock_file }
        })
    }

    /// Returns the next available port
    pub async fn next_port(&self) -> u16 {
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 100;

        while attempts < MAX_ATTEMPTS {
            let port = match self.get_next_port_from_file().await {
                Ok(port) => port,
                Err(e) => panic!("Failed to get next port: {}", e),
            };

            if port > MAX_PORTS {
                panic!("Port range exhausted");
            }

            if is_port_available(port) {
                return port;
            }

            attempts += 1;
        }

        panic!("Failed to acquire port after {} attempts", MAX_ATTEMPTS);
    }

    async fn get_next_port_from_file(&self) -> io::Result<u16> {
        // Try to create lock file with timeout
        wait_until!(
            { OpenOptions::new().write(true).create_new(true).open(&self.lock_file).is_ok() },
            Duration::from_secs(60)
        );

        // We have the lock, read and update port
        let port = {
            let contents = fs::read_to_string(&self.port_file)
                .unwrap_or_else(|_| DEFAULT_START_PORT.to_string());
            let current_port = contents.parse::<u16>().unwrap_or(DEFAULT_START_PORT);
            let next_port = current_port + 1;

            // Write next port
            fs::write(&self.port_file, next_port.to_string())?;
            current_port
        };

        // Release lock by removing the lock file
        fs::remove_file(&self.lock_file)?;

        Ok(port)
    }
}
