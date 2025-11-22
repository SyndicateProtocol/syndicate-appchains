use crate::{
    internal::{P384Coord, P384Point, P384Scalar},
    EncodedPoint, NistP384,
};
use core::{
    iter::Sum,
    ops::{Mul, MulAssign},
};
use elliptic_curve::{
    bigint::{ArrayEncoding, U384},
    ops::{LinearCombination, MulByGenerator},
    point::{AffineCoordinates, DecompactPoint, DecompressPoint},
    rand_core::RngCore,
    sec1::{FromEncodedPoint, ToEncodedPoint},
    subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption},
    zeroize::DefaultIsZeroes,
    FieldBytesEncoding,
};
use openvm_algebra_guest::IntMod;
use openvm_ecc_guest::{
    weierstrass::{IntrinsicCurve, WeierstrassPoint},
    CyclicGroup,
};

// --- Implement elliptic_curve traits on P384Point ---

/// P384 field element serialized as bytes.
///
/// Byte array containing a serialized field element value (base field or scalar).
pub type FieldBytes = elliptic_curve::FieldBytes<NistP384>;

impl FieldBytesEncoding<NistP384> for U384 {
    fn decode_field_bytes(field_bytes: &FieldBytes) -> Self {
        U384::from_be_byte_array(*field_bytes)
    }

    fn encode_field_bytes(&self) -> FieldBytes {
        self.to_be_byte_array()
    }
}

impl AffineCoordinates for P384Point {
    type FieldRepr = FieldBytes;

    fn x(&self) -> FieldBytes {
        *FieldBytes::from_slice(&<Self as WeierstrassPoint>::x(self).to_be_bytes())
    }

    fn y_is_odd(&self) -> Choice {
        (self.y().as_le_bytes()[0] & 1).into()
    }
}

impl Copy for P384Point {}

impl ConditionallySelectable for P384Point {
    fn conditional_select(a: &P384Point, b: &P384Point, choice: Choice) -> P384Point {
        P384Point::from_xy_unchecked(
            P384Coord::conditional_select(
                <Self as WeierstrassPoint>::x(a),
                <Self as WeierstrassPoint>::x(b),
                choice,
            ),
            P384Coord::conditional_select(a.y(), b.y(), choice),
        )
    }
}

impl ConstantTimeEq for P384Point {
    fn ct_eq(&self, other: &P384Point) -> Choice {
        <Self as WeierstrassPoint>::x(self).ct_eq(<Self as WeierstrassPoint>::x(other)) &
            self.y().ct_eq(other.y())
    }
}

impl Default for P384Point {
    fn default() -> Self {
        <Self as WeierstrassPoint>::IDENTITY
    }
}

impl DefaultIsZeroes for P384Point {}

impl Sum for P384Point {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(<Self as WeierstrassPoint>::IDENTITY, |a, b| a + b)
    }
}

impl<'a> Sum<&'a P384Point> for P384Point {
    fn sum<I: Iterator<Item = &'a P384Point>>(iter: I) -> Self {
        iter.cloned().sum()
    }
}

impl Mul<P384Scalar> for P384Point {
    type Output = P384Point;

    fn mul(self, other: P384Scalar) -> P384Point {
        NistP384::msm(&[other], &[self])
    }
}

impl Mul<&P384Scalar> for &P384Point {
    type Output = P384Point;

    fn mul(self, other: &P384Scalar) -> P384Point {
        NistP384::msm(&[*other], &[*self])
    }
}

impl Mul<&P384Scalar> for P384Point {
    type Output = P384Point;

    fn mul(self, other: &P384Scalar) -> P384Point {
        NistP384::msm(&[*other], &[self])
    }
}

impl MulAssign<P384Scalar> for P384Point {
    fn mul_assign(&mut self, rhs: P384Scalar) {
        *self = NistP384::msm(&[rhs], &[*self]);
    }
}

impl MulAssign<&P384Scalar> for P384Point {
    fn mul_assign(&mut self, rhs: &P384Scalar) {
        *self = NistP384::msm(&[*rhs], &[*self]);
    }
}

impl elliptic_curve::Group for P384Point {
    type Scalar = P384Scalar;

    fn random(mut _rng: impl RngCore) -> Self {
        // Self::GENERATOR * Self::Scalar::random(&mut rng)
        unimplemented!()
    }

    fn identity() -> Self {
        <Self as WeierstrassPoint>::IDENTITY
    }

    fn generator() -> Self {
        Self::GENERATOR
    }

    fn is_identity(&self) -> Choice {
        (<Self as openvm_ecc_guest::Group>::is_identity(self) as u8).into()
    }

    fn double(&self) -> Self {
        self + self
    }
}

impl elliptic_curve::group::Curve for P384Point {
    type AffineRepr = P384Point;

    fn to_affine(&self) -> P384Point {
        *self
    }
}

impl LinearCombination for P384Point {
    fn lincomb(x: &Self, k: &Self::Scalar, y: &Self, l: &Self::Scalar) -> Self {
        NistP384::msm(&[*k, *l], &[*x, *y])
    }
}

// default implementation
impl MulByGenerator for P384Point {}

impl DecompressPoint<NistP384> for P384Point {
    /// Note that this is not constant time
    fn decompress(x_bytes: &FieldBytes, y_is_odd: Choice) -> CtOption<Self> {
        use openvm_ecc_guest::weierstrass::FromCompressed;

        let x = P384Coord::from_be_bytes_unchecked(x_bytes.as_slice());
        let rec_id = y_is_odd.unwrap_u8();
        CtOption::new(x, (x.is_reduced() as u8).into()).and_then(|x| {
            let y = <P384Point as FromCompressed<P384Coord>>::decompress(x, &rec_id);
            match y {
                Some(point) => CtOption::new(point, 1.into()),
                None => CtOption::new(P384Point::default(), 0.into()),
            }
        })
    }
}

impl DecompactPoint<NistP384> for P384Point {
    fn decompact(x_bytes: &FieldBytes) -> CtOption<Self> {
        Self::decompress(x_bytes, Choice::from(0))
    }
}

impl FromEncodedPoint<NistP384> for P384Point {
    /// Attempts to parse the given [`EncodedPoint`] as an SEC1-encoded [`P384Point`].
    ///
    /// # Returns
    ///
    /// `None` value if `encoded_point` is not on the secp256k1 curve.
    fn from_encoded_point(encoded_point: &EncodedPoint) -> CtOption<Self> {
        match openvm_ecc_guest::ecdsa::VerifyingKey::<NistP384>::from_sec1_bytes(
            encoded_point.as_bytes(),
        ) {
            Ok(verifying_key) => CtOption::new(*verifying_key.as_affine(), 1.into()),
            Err(_) => CtOption::new(P384Point::default(), 0.into()),
        }
    }
}

impl ToEncodedPoint<NistP384> for P384Point {
    fn to_encoded_point(&self, compress: bool) -> EncodedPoint {
        EncodedPoint::conditional_select(
            &EncodedPoint::from_affine_coordinates(
                &<Self as WeierstrassPoint>::x(self).to_be_bytes().into(),
                &<Self as WeierstrassPoint>::y(self).to_be_bytes().into(),
                compress,
            ),
            &EncodedPoint::identity(),
            elliptic_curve::Group::is_identity(self),
        )
    }
}

impl TryFrom<EncodedPoint> for P384Point {
    type Error = elliptic_curve::Error;

    fn try_from(point: EncodedPoint) -> elliptic_curve::Result<P384Point> {
        P384Point::try_from(&point)
    }
}

impl TryFrom<&EncodedPoint> for P384Point {
    type Error = elliptic_curve::Error;

    fn try_from(point: &EncodedPoint) -> elliptic_curve::Result<P384Point> {
        Option::from(P384Point::from_encoded_point(point)).ok_or(elliptic_curve::Error)
    }
}

impl From<P384Point> for EncodedPoint {
    fn from(affine_point: P384Point) -> EncodedPoint {
        EncodedPoint::from(&affine_point)
    }
}

impl From<&P384Point> for EncodedPoint {
    fn from(affine_point: &P384Point) -> EncodedPoint {
        affine_point.to_encoded_point(true)
    }
}
