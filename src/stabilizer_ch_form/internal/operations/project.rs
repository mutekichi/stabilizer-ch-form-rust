use crate::StabilizerCHForm;
use crate::stabilizer_ch_form::internal::types::PhaseFactor;
use ndarray::Array1;

impl StabilizerCHForm {
    /// Projects the state onto a Z-basis eigenstate for a specific qubit.
    ///
    /// This is an internal helper for the measurement operation. It updates the
    /// state to a normalized state corresponding to the measurement outcome.
    /// This function assumes the qubit is in a superposition, and thus the
    /// projection is always possible.
    pub(crate) fn _collapse_to_z_basis_state(&mut self, q: usize, outcome: bool) {
        let g_row = self.mat_g.row(q);

        // As per the reference paper, projecting onto a Z-eigenstate is equivalent
        // to creating a superposition of the current state |s> with a shifted
        // version |u>, and then resolving it.

        // t = |s>
        let t: Array1<bool> = self.vec_s.clone();
        // u = |s> + G_q * v
        let u: Array1<bool> = &(&g_row & &self.vec_v) ^ &self.vec_s;

        // The phase `delta` determines which part of the superposition is kept.
        let not_v = !&self.vec_v;
        let z = if outcome { 1 } else { 0 };

        let delta_exponent =
            g_row
                .iter()
                .zip(&not_v)
                .zip(&self.vec_s)
                .fold(
                    0,
                    |acc, ((&g, &nv), &s)| {
                        if g && nv && s { acc + 1 } else { acc }
                    },
                );

        // The phase delta is i^(2*delta_exponent + 2*z)
        let delta = PhaseFactor::new((2 * delta_exponent + 2 * z) % 4);

        // When t != u (i.e., collapsing a superposition), the state is normalized
        // by a factor of 1/sqrt(2). This is handled by `_resolve_superposition`.
        // However, the overall global phase `omega` must also be scaled.
        self.omega /= 2.0_f64.sqrt();

        self._resolve_superposition(&t, &u, delta);
    }
}
