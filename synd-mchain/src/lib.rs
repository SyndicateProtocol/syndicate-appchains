//! The `synd-mchain` package provides a JSON-RPC interface for submitting
//! appchain batches and delayed messages to the `synd-mchain`.
//!
//! Note that the `synd-mchain` is not a real blockchain and is backed by
//! a rocksdb database instead.
#![allow(missing_docs)]
pub mod client;
pub mod db;
pub mod metrics;
pub mod server;
