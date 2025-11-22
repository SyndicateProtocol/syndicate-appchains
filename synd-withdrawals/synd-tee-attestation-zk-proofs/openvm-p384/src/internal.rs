use crate::NistP384;
use core::ops::{Add, Neg};
use hex_literal::hex;
use openvm_algebra_guest::IntMod;
use openvm_algebra_moduli_macros::moduli_declare;
use openvm_ecc_guest::{
    weierstrass::{CachedMulTable, IntrinsicCurve, WeierstrassPoint},
    CyclicGroup, Group,
};
use openvm_ecc_sw_macros::sw_declare;

// --- Define the OpenVM modular arithmetic and ecc types ---

moduli_declare! {
    P384Coord { modulus = "0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000ffffffff" },
    P384Scalar { modulus = "0xffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973" },
}

#[cfg(test)]
#[test]
fn test_p384_moduli() {
    use elliptic_curve::bigint::Encoding as _;
    assert_eq!(
        P384Coord::MODULUS,
        p384::U384::from_be_hex(
            <<p384::NistP384 as primeorder::PrimeCurveParams>::FieldElement as ff::PrimeField>::MODULUS
        )
        .to_le_bytes()
    );
    assert_eq!(
        P384Scalar::MODULUS,
        p384::U384::from_be_hex(<p384::Scalar as ff::PrimeField>::MODULUS).to_le_bytes()
    );
}

// from_const_bytes is little endian
pub const CURVE_A: P384Coord = P384Coord::from_const_bytes(hex!(
    "fcffffff0000000000000000fffffffffeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
));
pub const CURVE_B: P384Coord = P384Coord::from_const_bytes(hex!(
    "ef2aecd3edc8852a9dd12e8a8d3956c65a8713508f081403124181fe6e9c1d18192df8e36b058e98e4e73ee2a72f31b3"
));

sw_declare! {
    P384Point { mod_type = P384Coord, a = CURVE_A, b = CURVE_B },
}

// --- Implement internal traits ---

impl CyclicGroup for P384Point {
    // The constants are taken from: https://neuromancer.sk/std/secg/secp256r1
    const GENERATOR: Self = P384Point {
        // from_const_bytes takes a little endian byte string
        x: P384Coord::from_const_bytes(hex!(
            "b70a7672385e543a6c2955bf5df20255382a5482e041f759989ba78b623b1d6e74ad20f31ec7b18e37058bbe22ca87aa"
        )),
        y: P384Coord::from_const_bytes(hex!(
            "5f0eea907c1d437a9d817e1dceb1600ac0b8f0b51331dae97c149a28bd1df4f829dc9292bf989e5d6f2c26964ade1736"
        )),
    };
    const NEG_GENERATOR: Self = P384Point {
        x: P384Coord::from_const_bytes(hex!(
            "b70a7672385e543a6c2955bf5df20255382a5482e041f759989ba78b623b1d6e74ad20f31ec7b18e37058bbe22ca87aa"
        )),
        y: P384Coord::from_const_bytes(hex!(
            "a0f1156f84e2bc85627e81e2304e9ff53e470f4aecce251683eb65d742e20b07d6236d6d406761a290d3d969b521e8c9"
        )),
    };
}

#[cfg(test)]
#[test]
fn test_p384_point() {
    use primeorder::PrimeCurveParams;
    assert_eq!(P384Point::CURVE_A.to_be_bytes(), p384::NistP384::EQUATION_A.to_bytes().as_slice());
    assert_eq!(P384Point::CURVE_B.to_be_bytes(), p384::NistP384::EQUATION_B.to_bytes().as_slice());
    assert_eq!(
        P384Point::GENERATOR.x.to_be_bytes(),
        p384::NistP384::GENERATOR.0.to_bytes().as_slice()
    );
    assert_eq!(
        P384Point::GENERATOR.y.to_be_bytes(),
        p384::NistP384::GENERATOR.1.to_bytes().as_slice()
    );
    assert_eq!(P384Point::NEG_GENERATOR.x, (-P384Point::GENERATOR).x);
    assert_eq!(P384Point::NEG_GENERATOR.y, (-P384Point::GENERATOR).y);
}

impl IntrinsicCurve for NistP384 {
    type Scalar = P384Scalar;
    type Point = P384Point;

    fn msm(coeffs: &[Self::Scalar], bases: &[Self::Point]) -> Self::Point
    where
        for<'a> &'a Self::Point: Add<&'a Self::Point, Output = Self::Point>,
    {
        if coeffs.len() < 25 {
            let table = CachedMulTable::<Self>::new_with_prime_order(bases, 4);
            table.windowed_mul(coeffs)
        } else {
            openvm_ecc_guest::msm(coeffs, bases)
        }
    }
}

// --- Implement helpful methods mimicking the structs in p384 ---

impl P384Point {
    pub fn x_be_bytes(&self) -> [u8; 48] {
        <Self as WeierstrassPoint>::x(self).to_be_bytes()
    }

    pub fn y_be_bytes(&self) -> [u8; 48] {
        <Self as WeierstrassPoint>::y(self).to_be_bytes()
    }
}
