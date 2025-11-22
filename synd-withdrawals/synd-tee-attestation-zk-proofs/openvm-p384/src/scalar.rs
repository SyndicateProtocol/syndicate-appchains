use crate::{internal::P384Scalar, point::FieldBytes, NistP384, ORDER_HEX};
use alloc::vec::Vec;
use core::{cmp::Ordering, ops::ShrAssign};
use elliptic_curve::{
    bigint::{ArrayEncoding, Encoding, U384},
    ops::{Invert, Reduce},
    rand_core::RngCore,
    scalar::{FromUintUnchecked, IsHigh},
    subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption},
    zeroize::DefaultIsZeroes,
    Field, PrimeField, ScalarPrimitive,
};
use hex_literal::hex;
use openvm_algebra_guest::IntMod;

impl P384Scalar {
    /// Returns the SEC1 encoding of this scalar.
    pub fn to_bytes(&self) -> FieldBytes {
        self.to_be_bytes().into()
    }
}
// --- Implement elliptic_curve traits on P384Scalar ---

impl Copy for P384Scalar {}

impl From<u64> for P384Scalar {
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl Default for P384Scalar {
    fn default() -> Self {
        <Self as IntMod>::ZERO
    }
}

// Requires canonical form
impl ConstantTimeEq for P384Scalar {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.as_le_bytes().ct_eq(other.as_le_bytes())
    }
}

impl ConditionallySelectable for P384Scalar {
    fn conditional_select(a: &P384Scalar, b: &P384Scalar, choice: Choice) -> P384Scalar {
        P384Scalar::from_le_bytes_unchecked(
            &a.as_le_bytes()
                .iter()
                .zip(b.as_le_bytes().iter())
                .map(|(a, b)| u8::conditional_select(a, b, choice))
                .collect::<Vec<_>>(),
        )
    }
}

impl Field for P384Scalar {
    const ZERO: Self = <Self as IntMod>::ZERO;
    const ONE: Self = <Self as IntMod>::ONE;

    fn random(mut _rng: impl RngCore) -> Self {
        unimplemented!()
    }

    fn square(&self) -> Self {
        self * self
    }

    fn double(&self) -> Self {
        self + self
    }

    fn invert(&self) -> CtOption<Self> {
        // needs to be in canonical form for ct_eq
        self.assert_reduced();
        let is_zero = self.ct_eq(&<Self as IntMod>::ZERO);
        CtOption::new(<P384Scalar as openvm_algebra_guest::Field>::invert(self), !is_zero)
    }

    #[allow(clippy::many_single_char_names)]
    fn sqrt(&self) -> CtOption<Self> {
        match <Self as openvm_algebra_guest::Sqrt>::sqrt(self) {
            Some(sqrt) => CtOption::new(sqrt, 1.into()),
            None => CtOption::new(<Self as Field>::ZERO, 0.into()),
        }
    }

    fn sqrt_ratio(num: &Self, div: &Self) -> (Choice, Self) {
        ff::helpers::sqrt_ratio_generic(num, div)
    }
}

// from https://docs.rs/p384/latest/src/p384/arithmetic/scalar.rs.html
impl PrimeField for P384Scalar {
    type Repr = FieldBytes;

    const MODULUS: &'static str = ORDER_HEX;
    const CAPACITY: u32 = 383;
    const NUM_BITS: u32 = 384;
    const TWO_INV: Self = Self::from_const_bytes(hex!("ba946266b50c7676bd535824d9060dacef961bfac0a6b1e3ffffffffffffffffffffffffffffffffffffffffffffff7f"));
    const MULTIPLICATIVE_GENERATOR: Self = Self::from_const_bytes(hex!("020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"));
    const S: u32 = 1;
    const ROOT_OF_UNITY: Self = Self::from_const_bytes(hex!("7229c5cc6a19ecec7aa7b048b20d1a58df2d37f4814d63c7ffffffffffffffffffffffffffffffffffffffffffffffff"));
    const ROOT_OF_UNITY_INV: Self = Self::from_const_bytes(hex!("7229c5cc6a19ecec7aa7b048b20d1a58df2d37f4814d63c7ffffffffffffffffffffffffffffffffffffffffffffffff"));
    const DELTA: Self = Self::from_const_bytes(hex!("040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"));

    /// Attempts to parse the given byte array as an SEC1-encoded scalar.
    ///
    /// Returns None if the byte array does not contain a big-endian integer in the range
    /// [0, p).
    fn from_repr(bytes: FieldBytes) -> CtOption<Self> {
        let ret = Self::from_be_bytes_unchecked(bytes.as_slice());
        CtOption::new(ret, (ret.is_reduced() as u8).into())
    }

    // Endianness should match from_repr
    fn to_repr(&self) -> FieldBytes {
        *FieldBytes::from_slice(&self.to_be_bytes())
    }

    fn is_odd(&self) -> Choice {
        (self.as_le_bytes()[0] & 1).into()
    }
}

#[cfg(test)]
#[test]
fn test_p384_scalar() {
    assert_eq!(<P384Scalar as PrimeField>::MODULUS, p384::Scalar::MODULUS);
    assert_eq!(P384Scalar::CAPACITY, p384::Scalar::CAPACITY);
    assert_eq!(P384Scalar::NUM_BITS, p384::Scalar::NUM_BITS);
    assert_eq!(P384Scalar::TWO_INV.to_bytes(), p384::Scalar::TWO_INV.to_bytes());
    assert_eq!(
        P384Scalar::MULTIPLICATIVE_GENERATOR.to_bytes(),
        p384::Scalar::MULTIPLICATIVE_GENERATOR.to_bytes()
    );
    assert_eq!(P384Scalar::S, p384::Scalar::S);
    assert_eq!(P384Scalar::ROOT_OF_UNITY.to_bytes(), p384::Scalar::ROOT_OF_UNITY.to_bytes());
    assert_eq!(
        P384Scalar::ROOT_OF_UNITY_INV.to_bytes(),
        p384::Scalar::ROOT_OF_UNITY_INV.to_bytes()
    );
    assert_eq!(P384Scalar::DELTA.to_bytes(), p384::Scalar::DELTA.to_bytes());
}

impl ShrAssign<usize> for P384Scalar {
    fn shr_assign(&mut self, _rhs: usize) {
        // I don't think this is used anywhere
        unimplemented!()
    }
}

impl Reduce<U384> for P384Scalar {
    type Bytes = FieldBytes;

    fn reduce(w: U384) -> Self {
        <Self as openvm_algebra_guest::Reduce>::reduce_le_bytes(&w.to_le_bytes())
    }

    #[inline]
    fn reduce_bytes(bytes: &FieldBytes) -> Self {
        Self::reduce(U384::from_be_byte_array(*bytes))
    }
}

impl PartialOrd for P384Scalar {
    // requires self and other to be in canonical form
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.assert_reduced();
        other.assert_reduced();
        Some(
            self.to_be_bytes()
                .iter()
                .zip(other.to_be_bytes().iter())
                .map(|(a, b)| a.cmp(b))
                .find(|ord| *ord != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
        )
    }
}

impl IsHigh for P384Scalar {
    fn is_high(&self) -> Choice {
        // self > n/2
        // iff self + self overflows
        // iff self + self < self
        ((self + self < *self) as u8).into()
    }
}

impl Invert for P384Scalar {
    type Output = CtOption<Self>;

    fn invert(&self) -> CtOption<Self> {
        <Self as Field>::invert(self)
    }
}

impl FromUintUnchecked for P384Scalar {
    type Uint = U384;

    fn from_uint_unchecked(uint: Self::Uint) -> Self {
        Self::from_le_bytes_unchecked(&uint.to_le_bytes())
    }
}

impl From<ScalarPrimitive<NistP384>> for P384Scalar {
    fn from(scalar: ScalarPrimitive<NistP384>) -> Self {
        Self::from_le_bytes_unchecked(&scalar.as_uint().to_le_bytes())
    }
}

impl From<P384Scalar> for ScalarPrimitive<NistP384> {
    fn from(scalar: P384Scalar) -> ScalarPrimitive<NistP384> {
        ScalarPrimitive::from_slice(&scalar.to_be_bytes()).unwrap()
    }
}

impl DefaultIsZeroes for P384Scalar {}

impl AsRef<P384Scalar> for P384Scalar {
    fn as_ref(&self) -> &P384Scalar {
        self
    }
}

impl From<P384Scalar> for U384 {
    fn from(scalar: P384Scalar) -> Self {
        U384::from_be_slice(&scalar.to_be_bytes())
    }
}

impl From<P384Scalar> for FieldBytes {
    fn from(scalar: P384Scalar) -> Self {
        *FieldBytes::from_slice(&scalar.to_be_bytes())
    }
}
