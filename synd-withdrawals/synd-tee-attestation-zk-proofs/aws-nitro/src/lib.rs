#![no_std]

mod attestation_document;
mod cose;

#[cfg(not(feature = "openvm"))]
pub fn keccak256(data: &[u8]) -> [u8; 32] {
    alloy_primitives::keccak256(data).0
}
pub use attestation_document::*;
#[cfg(feature = "openvm")]
pub use openvm_keccak256::keccak256;
#[cfg(feature = "openvm")]
pub use openvm_p384 as p384;
#[cfg(not(feature = "openvm"))]
pub use p384;
