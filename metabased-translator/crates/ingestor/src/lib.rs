//! the `ingestor` crate exports the `ingestor` module

mod ingestor;
pub use ingestor::run;

pub mod config;
pub mod metrics;
