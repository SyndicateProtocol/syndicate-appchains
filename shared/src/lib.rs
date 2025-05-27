//! The `shared` library contains functions and code that are reused by the `syndicate-appchain`
//! stack.

pub mod append_only_db;
pub mod fixed_size_append_only_db;
pub mod json_rpc;
pub mod logger;
pub mod parse;
pub mod service_start_utils;
pub mod single_value_db;
pub mod tx_validation;
pub mod types;
pub mod zlib_compression;
