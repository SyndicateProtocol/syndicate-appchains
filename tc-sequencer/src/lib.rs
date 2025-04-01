//! The `tc-sequencer` crate provides a JSON-RPC interface for submitting transactions to the TC
//! Sequencer.

pub mod bytecode;
pub mod config;
pub mod errors;
pub mod server;
pub mod tc_client;
pub mod validation;
