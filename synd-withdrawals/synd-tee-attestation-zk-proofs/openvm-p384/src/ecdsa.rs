// re-export types that are visible in the p384 crate for API compatibility

// Use these types instead of unpatched p384::ecdsa::{Signature, VerifyingKey}
// because those are type aliases that use non-zkvm implementations

use super::NistP384;
pub use ecdsa_core::signature::{self, Error};
#[cfg(feature = "ecdsa")]
use openvm_ecc_guest::ecdsa::VerifyCustomHook;
#[cfg(feature = "ecdsa")]
use {super::P384Point, ecdsa_core::hazmat::VerifyPrimitive};

/// ECDSA/secp256k1 signature (fixed-size)
pub type Signature = ecdsa_core::Signature<NistP384>;

#[cfg(feature = "sha384")]
impl ecdsa_core::hazmat::DigestPrimitive for NistP384 {
    type Digest = sha2::Sha384;
}

/// ECDSA/secp256k1 signing key
#[cfg(feature = "ecdsa")]
pub type SigningKey = ecdsa_core::SigningKey<NistP384>;

/// ECDSA/secp256k1 verification key (i.e. public key)
#[cfg(feature = "ecdsa")]
pub type VerifyingKey = openvm_ecc_guest::ecdsa::VerifyingKey<NistP384>;

// No custom hook
#[cfg(feature = "ecdsa")]
impl VerifyCustomHook<NistP384> for P384Point {}

#[cfg(feature = "ecdsa")]
impl VerifyPrimitive<NistP384> for P384Point {
    fn verify_prehashed(
        &self,
        z: &crate::point::FieldBytes,
        sig: &Signature,
    ) -> Result<(), ecdsa_core::Error> {
        openvm_ecc_guest::ecdsa::verify_prehashed::<NistP384>(
            *self,
            z.as_slice(),
            sig.to_bytes().as_slice(),
        )
        .map_err(|_| ecdsa_core::Error::new())
    }
}
