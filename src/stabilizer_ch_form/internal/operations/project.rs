use crate::{
    StabilizerCHForm,
    stabilizer_ch_form::internal::types::{PhaseFactor, measurement::QubitState},
};

impl StabilizerCHForm {
    pub(crate) fn _project(&mut self, qarg: usize, outcome: bool) -> Result<(), &'static str> {
        if qarg >= self.n {
            panic!("Qubit index out of bounds.");
        }
        if qarg >= self.n_qubits() {
            panic!("Qubit index out of bounds.");
        }

        let qubit_state = self._get_qubit_state(qarg);
        match qubit_state {
            QubitState::Determined(value) => {
                if value != outcome {
                    Err("Measurement outcome inconsistent with determined state.")
                } else {
                    // No change needed if the state is already determined and matches the outcome.
                    Ok(())
                }
            }
            QubitState::Superposition => {
                // Collapse the state to the desired outcome.
                // Applys the operator: (I + (-1)^(1-outcome) * Z_qarg) / 2
                // Z_arg application can be represented as:
                //   Z_qarg U_C U_H |s> = (-1)^α |t>
                // according to eq.(48) and (49) in arXiv:1808.00128
                let g_row = self.mat_g.row(qarg);
                let vec_t = &(&g_row & &self.vec_v) ^ &self.vec_s;
                let alpha = g_row
                    .iter()
                    .zip(&self.vec_v)
                    .zip(&self.vec_s)
                    .filter(|&((&g, &v), &s)| g && !v && s)
                    .count()
                    % 2
                    != 0;
                let delta = if alpha ^ outcome {
                    PhaseFactor::MINUS_ONE
                } else {
                    PhaseFactor::PLUS_ONE
                };
                self._resolve_superposition(&self.vec_s.to_owned(), &vec_t, delta);
                Ok(())
            }
        }
    }
}
