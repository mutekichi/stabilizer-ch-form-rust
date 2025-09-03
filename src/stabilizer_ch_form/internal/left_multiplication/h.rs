use crate::StabilizerCHForm;
use ndarray::Array1;

impl StabilizerCHForm {
    /// Applies the Hadamard gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n^2)
    ///
    /// See around Proposition 4. of arXiv:1808.00128 for details.
    pub fn _left_multiply_h(&mut self, qarg: usize) {
        if qarg >= self.n {
            panic!("Qubit index out of bounds.");
        }
        let (vec_t, vec_u, alpha, beta) = self._prepare_h_superposition_args(qarg);
        let delta = if alpha ^ beta {
            self.gamma[qarg].flipped()
        } else {
            self.gamma[qarg]
        };
        if alpha {
            self.phase_factor.flip_sign();
        }
        self._resolve_superposition(&vec_t, &vec_u, delta);
    }

    /// Prepares vec_t, vec_u, alpha, beta for applying H to qubit `qarg`.
    pub fn _prepare_h_superposition_args(
        &self,
        qarg: usize,
    ) -> (Array1<bool>, Array1<bool>, bool, bool) {
        let g_row = self.mat_g.row(qarg);
        let f_row = self.mat_f.row(qarg);
        let m_row = self.mat_m.row(qarg);

        let not_vec_v = !&self.vec_v;

        // eq. (48) of arXiv:1808.00128
        let vec_t = &self.vec_s ^ (&g_row & &self.vec_v);
        let vec_u = &self.vec_s ^ (&f_row & &not_vec_v) ^ (&m_row & &self.vec_v);

        // eq. (49) of arXiv:1808.00128
        // alpha
        let alpha = (g_row.iter().zip(&self.vec_v).zip(&self.vec_s))
            .filter(|&((&g, &v), &s)| g && !v && s)
            .fold(false, |acc, ((&g, &nv), &s)| acc ^ (g && nv && s));
        // beta
        let beta = {
            let term1_is_odd = m_row
                .iter()
                .zip(&not_vec_v)
                .zip(&self.vec_s)
                .fold(false, |acc, ((&m, &nv), &s)| acc ^ (m && nv && s));
            let term2_is_odd = f_row
                .iter()
                .zip(&self.vec_v)
                .zip(m_row)
                .fold(false, |acc, ((&f, &v), &m)| acc ^ (f && v && m));
            let term3_is_odd = f_row
                .iter()
                .zip(&self.vec_v)
                .zip(&self.vec_s)
                .fold(false, |acc, ((&f, &v), &s)| acc ^ (f && v && s));
            term1_is_odd ^ term2_is_odd ^ term3_is_odd
        };

        (vec_t, vec_u, alpha, beta)
    }
}
