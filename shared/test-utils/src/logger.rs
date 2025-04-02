//! The `logger` module relates to test tracing
use crate::logger::Error::TestSubscriberInit;
use thiserror::Error;
use tracing::Level;
use tracing_subscriber::EnvFilter;

/// Initializes a tracing subscriber for testing purposes. Output from crates not specified below is
/// filtered out
pub fn init_test_tracing(level: Level) -> Result<(), Error> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // TODO SEQ-765: add other MB packages below and make sure they work 
        EnvFilter::new(format!(
            "off,metabased_translator={level},ingestor={level},slotter={level},block_builder={level}"
        ))
    });

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .try_init()
        .map_err(|e| TestSubscriberInit(format!("{:?}", e)))?;
    Ok(())
}

#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to initialize test tracing subscriber: {0}")]
    TestSubscriberInit(String),
}
