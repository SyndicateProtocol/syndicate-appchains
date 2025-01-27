//! Service status tracking for metabased-translator components
//!
//! This module provides a thread-safe way to track the lifecycle state of services
//! using atomic operations. It's used by various components to coordinate their
//! startup, running, and shutdown states.

use std::sync::atomic::{AtomicU8, Ordering};

/// Represents the possible states of a service
#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Status {
    /// Service has been instantiated but not yet started
    NotStarted = 0,
    /// Service is currently running
    Started = 1,
    /// Service has been stopped
    Stopped = 2,
}

/// A thread-safe wrapper for tracking service status
///
/// Uses atomic operations to safely share status across threads without requiring locks.
#[derive(Debug)]
pub struct ServiceStatus {
    status: AtomicU8,
}

// TODO delete me
impl ServiceStatus {
    /// Creates a new [`ServiceStatus`] with the default status of [`Status::NotStarted`]
    pub const fn new() -> Self {
        Self { status: AtomicU8::new(Status::NotStarted as u8) }
    }

    /// Atomically stores a new status value
    ///
    /// Uses Release ordering to ensure all previous operations are visible
    /// to other threads that load this value.
    pub fn store(&self, status: Status) {
        self.status.store(status as u8, Ordering::Release);
    }

    /// Atomically loads the current status
    ///
    /// Uses Acquire ordering to ensure all store operations before this load are seen
    ///
    /// Returns `Status::NotStarted` if the internal value is invalid.
    pub fn load(&self) -> Status {
        match self.status.load(Ordering::Acquire) {
            0 => Status::NotStarted,
            1 => Status::Started,
            2 => Status::Stopped,
            _ => panic!("Invalid service status value"),
        }
    }
}

impl Default for ServiceStatus {
    fn default() -> Self {
        Self::new()
    }
}
