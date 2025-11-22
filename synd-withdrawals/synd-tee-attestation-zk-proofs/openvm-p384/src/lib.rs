// Fork of RustCrypto's p384 crate https://docs.rs/p384/latest/p384/
// that uses zkvm instructions

#![no_std]
#![allow(deprecated)] // generic-array

extern crate alloc;

use elliptic_curve::{
    bigint::U384, consts::U48, point::PointCompression, Curve, CurveArithmetic, PrimeCurve,
};

mod coord;
mod internal;
mod point;
mod scalar;

#[cfg(feature = "ecdsa-core")]
pub mod ecdsa;

pub use elliptic_curve;
#[cfg(feature = "pkcs8")]
pub use elliptic_curve::pkcs8;
// Needs to be public so that the `sw_init` macro can access it
pub use internal::{P384Coord, P384Point, P384Scalar};

// -- Define the ZST for implementing the elliptic curve traits --
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct NistP384;

#[cfg(feature = "pkcs8")]
impl pkcs8::AssociatedOid for NistP384 {
    const OID: pkcs8::ObjectIdentifier = pkcs8::ObjectIdentifier::new_unwrap("1.3.132.0.34");
}

// --- Implement the Curve trait on P384 ---

/// Order of the P256 elliptic curve in hexadecimal.
const ORDER_HEX: &str = "ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973";

impl Curve for NistP384 {
    /// 32-byte serialized field elements.
    type FieldBytesSize = U48;

    // Perf: Use the U384 type from openvm_ruint here
    type Uint = U384;

    /// Curve order.
    const ORDER: U384 = U384::from_be_hex(ORDER_HEX);
}

#[cfg(test)]
#[test]
fn test_p384_order() {
    assert_eq!(NistP384::ORDER, p384::NistP384::ORDER);
}

impl PrimeCurve for NistP384 {}

impl CurveArithmetic for NistP384 {
    type AffinePoint = P384Point;
    /// The `ProjectivePoint` type is still internally represented as an affine point.
    type ProjectivePoint = P384Point;
    type Scalar = P384Scalar;
}

impl PointCompression for NistP384 {
    /// P384 points are typically uncompressed.
    const COMPRESS_POINTS: bool = false;
}

/// SEC1-encoded P384 curve point.
pub type EncodedPoint = elliptic_curve::sec1::EncodedPoint<NistP384>;
