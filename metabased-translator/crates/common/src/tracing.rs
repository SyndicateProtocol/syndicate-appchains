//! Common tracing module for Metabased Translator
//!
//! This module initializes the tracing configuration for the Metabased Translator, and by extension
//! other crates used in the Metabased Translator as well.

use core::fmt;
use std::{error::Error, fmt::Display};
use tracing_subscriber::{fmt as subscriber_fmt, EnvFilter};

// TODO(SEQ-515): Reconsider location, put me in `bin` ?

#[allow(missing_docs)]
#[derive(Debug)]
pub enum TracingError {
    SubscriberInit(String),
}

impl Display for TracingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SubscriberInit(msg) => {
                write!(f, "Failed to initialize subscriber: {}", msg)
            }
        }
    }
}

impl Error for TracingError {}

/// Initializes the common tracing subscriber for the Metabased Translator
pub fn init_tracing() -> Result<(), TracingError> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    subscriber_fmt()
        .json()
        .with_target(true)
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(env_filter)
        .try_init()
        .map_err(|e| TracingError::SubscriberInit(format!("{:?}", e)))?;

    Ok(())
}

/// Initializes a tracing subscriber for testing purposes
pub fn init_test_tracing(level: &str) -> Result<(), TracingError> {
    subscriber_fmt()
        .with_env_filter(EnvFilter::new(level))
        .try_init()
        .map_err(|e| TracingError::SubscriberInit(format!("{:?}", e)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_tracing() {
        let result = init_tracing();
        assert!(result.is_ok());
    }
}
