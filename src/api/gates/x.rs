use crate::StabilizerCHForm;
use ndarray::Array1;

pub trait XGate {
    fn apply_x(&mut self, qarg: usize);
}

impl StabilizerCHForm {
    /// Applies the Pauli-X gate to the qubit at index `qarg`.
    /// 
    /// Time complexity: O(n)
    /// 
    /// See around eq.(48) of arXiv:1808.00128 for details.
    pub fn apply_x(&mut self, qarg: usize) {
        self._apply_x(qarg);
    }
}