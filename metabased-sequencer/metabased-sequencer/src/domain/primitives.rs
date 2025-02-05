//! Primitive types commonly used in Ethereum such as 256-bit unsigned integers.
//!
//! It provides:
//! * The source of truth which defines the primitives in use.
//! * A mechanism to easily swap out the primitives without affecting all the places that use them.

pub use alloy::primitives::{Address, Bytes, TxHash, B256};
