use ndarray::{Array1, Array2};
use num_complex::Complex64;


mod internal;
use crate::api::QuantumCircuit;
use crate::api::QuantumGate;
use crate::api::gates::*;
use internal::types::phase_factor::PhaseFactor;

#[derive(Debug, Clone)]
pub struct StabilizerCHForm {
    n: usize,
    mat_g: Array2<bool>,
    mat_f: Array2<bool>,
    mat_m: Array2<bool>,
    gamma: Array1<PhaseFactor>,
    vec_v: Array1<bool>,
    vec_s: Array1<bool>,
    omega: Complex64,
    phase_factor: PhaseFactor,
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

    pub fn n_qubits(&self) -> usize {
        self.n
    }

    pub fn set_global_phase(&mut self, phase: Complex64) {
        if (phase.norm_sqr() - 1.0).abs() > 1e-8 {
            panic!("Global phase must be a unit complex number.");
        }
        self.omega = phase;
    }

    pub fn global_phase(&self) -> Complex64 {
        self.omega
    }

    pub fn try_from(circuit: &QuantumCircuit) -> Result<Self, String> {
        if circuit.n_qubits == 0 {
            return Err("Number of qubits must be greater than zero.".to_string());
        }
        let mut ch_form = StabilizerCHForm::new(circuit.n_qubits);

        for gate in &circuit.gates {
            match gate {
                QuantumGate::H(q) => ch_form.apply_h(*q),
                QuantumGate::S(q) => ch_form.apply_s(*q),
                QuantumGate::X(q) => ch_form.apply_x(*q),
                QuantumGate::Z(q) => ch_form.apply_z(*q),
                QuantumGate::CX(c, t) => ch_form.apply_cx(*c, *t),
                QuantumGate::CZ(q1, q2) => ch_form.apply_cz(*q1, *q2),
            }
        }
        Ok(ch_form)
    }
}
