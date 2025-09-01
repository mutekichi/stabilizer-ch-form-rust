use ndarray::{Array1, Array2};
use num_complex::Complex64;

pub mod phase_factor;
pub mod gates;
pub mod statevector;
pub mod amplitude;
pub mod utils;

use phase_factor::PhaseFactor;

#[derive(Debug, Clone)]
pub struct StabilizerCHForm {
    pub n: usize,
    pub mat_g: Array2<bool>,
    pub mat_f: Array2<bool>,
    pub mat_m: Array2<bool>,
    pub gamma: Array1<PhaseFactor>,
    pub vec_v: Array1<bool>,
    pub vec_s: Array1<bool>,
    pub omega: Complex64,
    pub phase_factor: PhaseFactor,
}

impl StabilizerCHForm {
    pub fn new(n: usize) -> Self {
        if n == 0 {
            panic!("Number of qubits must be greater than zero.");
        }

        Self {
            n: n,
            // Initialize G, F as identity matrices, M as zero matrix
            mat_g: Array2::from_shape_fn((n, n), |(i, j)| i == j),
            mat_f: Array2::from_shape_fn((n, n), |(i, j)| i == j),
            mat_m: Array2::from_elem((n, n), false),
            // Initialize gamma as [+1, +1, ..., +1]
            gamma: Array1::from_elem(n, PhaseFactor::PlusOne),
            // Initialize v, s as zero vectors
            vec_v: Array1::from_elem(n, false),
            vec_s: Array1::from_elem(n, false),
            // Initialize omega as 1 + 0i
            omega: Complex64::new(1.0, 0.0),
            // Initialize overall phase factor as +1
            phase_factor: PhaseFactor::PlusOne,
        }
    }
}