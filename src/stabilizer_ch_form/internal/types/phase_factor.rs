/// Represents the phase factor of a stabilizer generator.
use std::ops::{Mul, MulAssign};

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

impl MulAssign for PhaseFactor {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl PhaseFactor {
    pub fn to_complex(&self) -> num_complex::Complex64 {
        match self {
            PhaseFactor::PlusOne => num_complex::Complex64::new(1.0, 0.0),
            PhaseFactor::PlusI => num_complex::Complex64::new(0.0, 1.0),
            PhaseFactor::MinusOne => num_complex::Complex64::new(-1.0, 0.0),
            PhaseFactor::MinusI => num_complex::Complex64::new(0.0, -1.0),
        }
    }

    pub fn flipped(&self) -> Self {
        match self {
            PhaseFactor::PlusOne => PhaseFactor::MinusOne,
            PhaseFactor::PlusI => PhaseFactor::MinusI,
            PhaseFactor::MinusOne => PhaseFactor::PlusOne,
            PhaseFactor::MinusI => PhaseFactor::PlusI,
        }
    }

    pub fn flip_sign(&mut self) {
        *self = self.flipped();
    }
}
