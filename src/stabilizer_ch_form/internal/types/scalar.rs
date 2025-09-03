use super::PhaseFactor;
use num_complex::Complex64;

/// Represents a scalar value in the form `phase * 2^(-r/2)` or zero.
/// This is used for representing amplitudes and inner products exactly, avoiding floating-point errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scalar {
    Zero,
    NonZero { phase: PhaseFactor, r: usize },
}

impl Scalar {
    /// Converts the scalar to its `Complex64` representation.
    pub fn to_complex(&self) -> Complex64 {
        match self {
            Scalar::Zero => Complex64::new(0.0, 0.0),
            Scalar::NonZero { phase, r } => {
                let norm = 2.0_f64.powf(-(*r as f64) / 2.0);
                phase.to_complex() * norm
            }
        }
    }
}