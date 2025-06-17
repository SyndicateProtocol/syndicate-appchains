//! The `synd-proposer` is responsible for extracting the appchain root state and submitting
//! assertions to the settlement chain `AssertionPoster` contract

pub mod config;
pub mod metrics;
pub mod proposer;
#[allow(missing_docs)]
pub mod types;
