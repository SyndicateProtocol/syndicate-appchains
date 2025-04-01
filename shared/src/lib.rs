//! The `shared` library contains functions and code that are reused by the `metabased-rollup`
//! stack.

pub mod json_rpc;
pub mod logger;
pub mod parse;
pub mod tx_validation;

pub mod test_utils;
pub use crate::test_utils::assert_eventually; // Needed for `test_utils::wait_until!` usage
