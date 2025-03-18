//! The `metabased-poster` is responsible for extracting the appchain root state and submitting
//! assertions to the settlement chain.

pub mod config;
pub mod poller;
pub mod shutdown_channels;
pub mod spawn;
pub mod submitter;
pub mod types;
