//! the `ingestor` crate exports the `ingestor` module

pub mod ingestor;
pub use ingestor::IngestorArgs;

mod ingestor_http;
pub use ingestor_http::run_http;

mod ingestor_subscription;
pub use ingestor_subscription::run_subscription;

pub mod config;
pub mod metrics;
