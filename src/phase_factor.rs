/// Represents the phase factor of a stabilizer generator.
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

#[repr(u8)]
pub enum PhaseFactor {
    PlusOne,
    PlusI,
    MinusOne,
    MinusI,
}

impl Mul for PhaseFactor {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        use PhaseFactor::*;
        match (self, rhs) {
            (PlusOne, p) | (p, PlusOne) => p,
            (PlusI, PlusI) => MinusOne,
            (PlusI, MinusOne) => MinusI,
            (PlusI, MinusI) => PlusOne,
            (MinusOne, PlusI) => MinusI,
            (MinusOne, MinusOne) => PlusOne,
            (MinusOne, MinusI) => PlusI,
            (MinusI, PlusI) => PlusOne,
            (MinusI, MinusOne) => PlusI,
            (MinusI, MinusI) => MinusOne,
        }
    }
}