//! Optimism rollup block-builder implementation
//!
//! This module provides functionality for encoding batches of transactions into frames
//! that can be submitted by the batcher.
pub mod batch;
pub mod frame;

/// Optimism block builder
pub mod optimism_builder;
