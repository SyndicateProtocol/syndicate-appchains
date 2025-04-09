//! The `logger` module relates to Logger initialization and setup

use crate::logger::Error::DefaultLoggerInit;
use thiserror::Error;
use tracing_subscriber::EnvFilter;

/// Constructs the default subscriber for the `metabased` stack and
/// "Enables" the default subscriber as the global default for a Rust program
///
/// **IMPORTANT**: This function is intended to be called by binaries or other
/// crates *using* this library, and **must not be called within the `shared` crate itself**.
///
/// [`tracing::subscriber::set_global_default`] can only be called once globally.
/// Any invocation inside the `shared` library risks causing runtime
/// conflicts with the calling application or other libraries.
pub fn set_global_default_subscriber() -> Result<(), Error> {
    // Build an EnvFilter from the `RUST_LOG` environment variable, defaulting to `info` if the env
    // variable is not set.
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        // output in JSON format
        .json() // TODO SEQ-797 - add an option to disable this - so we can get colorized output in tests
        // include codepath origin of log
        .with_target(true)
        // log level is controlled by RUST_LOG setting
        .with_env_filter(env_filter)
        .try_init()
        .map_err(|e| DefaultLoggerInit(e.to_string()))
}

/// Errors relating to the logger
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum Error {
    /// error initializing the default logger
    #[error("unable to initialize default logger - did you call this more than once?: {0} ")]
    DefaultLoggerInit(String),
}
