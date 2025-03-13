//! Block builder crate handles block construction, rollup translation, and connection to the meta
//! chain

mod block_builder;
pub use block_builder::{run, BlockBuilderError};

pub mod config;
pub mod connectors;
pub mod metrics;
pub mod rollups;
