use num_complex::Complex64;

use crate::phase_factor::PhaseFactor;
// Represents i^p 2^{-r/2} where p in {0,1,2,3} and r >= 0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Amplitude {
    Zero,
    NonZero { phase: PhaseFactor, r: usize },
}

impl Amplitude {
    pub fn to_complex(&self) -> Complex64 {
        match self {
            Amplitude::Zero => Complex64::new(0.0, 0.0),
            Amplitude::NonZero { phase, r } => {
                let norm = 2f64.powf(-(*r as f64) / 2.0);
                phase.to_complex() * norm
            }
        }
    }
}
// test
#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-15;

    fn complex_approx_eq(a: Complex64, b: Complex64) -> bool {
        (a.re - b.re).abs() < EPSILON && (a.im - b.im).abs() < EPSILON
    }

    #[test]
    fn test_amplitude_to_complex() {
        let amp1 = Amplitude::Zero;
        assert!(complex_approx_eq(amp1.to_complex(), Complex64::new(0.0, 0.0)));

        let amp2 = Amplitude::NonZero { phase: PhaseFactor::PlusOne, r: 0 };
        assert!(complex_approx_eq(amp2.to_complex(), Complex64::new(1.0, 0.0)));

        let amp3 = Amplitude::NonZero { phase: PhaseFactor::PlusI, r: 1 };
        assert!(complex_approx_eq(amp3.to_complex(), Complex64::new(0.0, 1.0 / (2f64).sqrt())));

        let amp4 = Amplitude::NonZero { phase: PhaseFactor::MinusOne, r: 2 };
        assert!(complex_approx_eq(amp4.to_complex(), Complex64::new(-1.0 / 2.0, 0.0)));

        let amp5 = Amplitude::NonZero { phase: PhaseFactor::MinusI, r: 3 };
        assert!(complex_approx_eq(amp5.to_complex(), Complex64::new(0.0, -1.0 / (2f64).sqrt() / 2.0)));
    }
}