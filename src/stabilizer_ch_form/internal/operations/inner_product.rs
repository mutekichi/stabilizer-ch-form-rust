use crate::stabilizer_ch_form::internal::types::PhaseFactor;
use crate::StabilizerCHForm;
use crate::stabilizer_ch_form::internal::types::internal_gate::InternalGate;

impl StabilizerCHForm {
    // TODO: Implement batch inner product calculation since the result of 
    // `_self._get_normalize_to_zero_ops()` can be reused.
    pub(crate) fn _inner_product(&self, other: &StabilizerCHForm) -> num_complex::Complex64 {
        if self.n != other.n {
            panic!("Inner product is only defined for states on the same number of qubits.");
        }

        let (ops, phase) = self._get_normalize_to_zero_ops();

        let transformed_other = other._get_ops_applied_state(&ops);

        
    }

    fn _get_normalize_to_zero_ops(&self) -> (Vec<InternalGate>, PhaseFactor) {
        let mut ops = Vec::new();
        (ops, PhaseFactor::PLUS_ONE) // Placeholder
    }
}
