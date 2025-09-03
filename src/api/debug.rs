//! # Debugging Utilities
//!
//! Provides APIs to inspect the internal state of stabilizer representations.
//! These APIs are intended for debugging and testing purposes and may change
//! without notice in future versions.

use ndarray::{Array1, Array2};
use crate::stabilizer_ch_form::StabilizerCHForm;

/// A snapshot of the internal boolean matrices and vectors of a `StabilizerCHForm`.
///
/// This struct is primarily used for debugging and comparing internal states
/// with reference implementations.
#[derive(Debug)]
pub struct CHFormInternalState {
    pub n_qubits: usize,
    pub mat_g: Array2<bool>,
    pub mat_f: Array2<bool>,
    pub mat_m: Array2<bool>,
    pub vec_v: Array1<bool>,
    pub vec_s: Array1<bool>,
}

impl CHFormInternalState {
    /// Prints the internal state to the console in a human-readable format (0s and 1s).
    pub fn pretty_print(&self) {
        println!("--- CHFormInternalState (n={}) ---", self.n_qubits);

        let print_mat = |name: &str, mat: &Array2<bool>| {
            println!("{}: [", name);
            for row in mat.rows() {
                let s: String = row.iter().map(|&b| if b { '1' } else { '0' }).collect();
                println!("  {}", s);
            }
            println!("]");
        };

        let print_vec = |name: &str, vec: &Array1<bool>| {
            let s: String = vec.iter().map(|&b| if b { '1' } else { '0' }).collect();
            println!("{}: [{}]", name, s);
        };
        
        print_mat("mat_g", &self.mat_g);
        print_mat("mat_f", &self.mat_f);
        print_mat("mat_m", &self.mat_m);
        print_vec("vec_v", &self.vec_v);
        print_vec("vec_s", &self.vec_s);
        println!("------------------------------------");
    }
}

/// Provides a method to get a snapshot of the internal state.
pub trait Inspectable {
    fn get_internal_state(&self) -> CHFormInternalState;
}

impl Inspectable for StabilizerCHForm {
    /// Creates a snapshot of the `StabilizerCHForm`'s internal boolean state.
    fn get_internal_state(&self) -> CHFormInternalState {
        CHFormInternalState {
            n_qubits: self.n_qubits(),
            mat_g: self.mat_g.clone(),
            mat_f: self.mat_f.clone(),
            mat_m: self.mat_m.clone(),
            vec_v: self.vec_v.clone(),
            vec_s: self.vec_s.clone(),
        }
    }
}