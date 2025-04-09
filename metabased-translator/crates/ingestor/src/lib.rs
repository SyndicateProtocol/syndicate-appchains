//! the `ingestor` crate exports the `ingestor` module

mod ingestor;
pub use ingestor::run;

mod ingestor_subscription;
pub use ingestor_subscription::run_subscription;

mod ingestor_http;
pub use ingestor_http::run_http;

pub mod config;
pub mod metrics;
