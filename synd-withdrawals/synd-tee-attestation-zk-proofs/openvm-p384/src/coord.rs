use crate::internal::P384Coord;
use alloc::vec::Vec;
use elliptic_curve::subtle::{Choice, ConditionallySelectable, ConstantTimeEq};
use openvm_algebra_guest::IntMod;

// --- Implement elliptic_curve traits on P384Coord ---

impl Copy for P384Coord {}

impl Default for P384Coord {
    fn default() -> Self {
        <Self as IntMod>::ZERO
    }
}

impl ConditionallySelectable for P384Coord {
    fn conditional_select(a: &P384Coord, b: &P384Coord, choice: Choice) -> P384Coord {
        P384Coord::from_le_bytes_unchecked(
            &a.as_le_bytes()
                .iter()
                .zip(b.as_le_bytes().iter())
                .map(|(a, b)| u8::conditional_select(a, b, choice))
                .collect::<Vec<_>>(),
        )
    }
}

impl ConstantTimeEq for P384Coord {
    fn ct_eq(&self, other: &P384Coord) -> Choice {
        #[cfg(not(target_os = "zkvm"))]
        {
            // Requires canonical form
            self.as_le_bytes().ct_eq(other.as_le_bytes())
        }
        #[cfg(target_os = "zkvm")]
        {
            // The zkVM implementation calls iseqmod opcode so it is constant time, _except_ a check
            // of whether the setup opcode has been called already
            Choice::from((self == other) as u8)
        }
    }
}
