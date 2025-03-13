//! Slotter crate for metabased-translator

mod slotter;
pub use slotter::{run, SlotterError};

pub mod config;
pub mod metrics;
