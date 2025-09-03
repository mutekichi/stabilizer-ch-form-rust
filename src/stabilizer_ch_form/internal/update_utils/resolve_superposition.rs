use crate::StabilizerCHForm;
use crate::stabilizer_ch_form::internal::types::PhaseFactor;

impl StabilizerCHForm {
    pub fn _resolve_superposition(
        &mut self,
        vec_t: &ndarray::Array1<bool>,
        vec_u: &ndarray::Array1<bool>,
        mut delta: PhaseFactor,
    ) {
        if vec_t == vec_u {
            self._handle_same_vecs_case(delta, vec_t);
            return;
        }
        let (diff_indices_0, diff_indices_1) = self._get_differing_indices(vec_t, vec_u);
        let pivot = self._apply_basis_transform_circuit(&diff_indices_0, &diff_indices_1);
        // if t_q == 1, (y_q, z_q) = (1,0)
        // make sure y_q == 0 by:
        // |1> + i^delta |0> = i^delta (|0> + i^{-delta} |1>)
        if vec_t[pivot] {
            self.phase_factor *= delta;
            if delta == PhaseFactor::PlusI || delta == PhaseFactor::MinusI {
                delta.flip_sign();
            }
        }
        match delta {
            PhaseFactor::PlusOne => {
                self.vec_s[pivot] = false;
                self.vec_v[pivot] = !self.vec_v[pivot];
            }
            PhaseFactor::MinusOne => {
                self.vec_s[pivot] = true;
                self.vec_v[pivot] = !self.vec_v[pivot];
            }
            PhaseFactor::PlusI => {
                if self.vec_v[pivot] {
                    // rotate 45 deg
                    self.set_global_phase(
                        self.global_phase()
                            * num_complex::Complex64::new(1.0 / 2f64.sqrt(), 1.0 / 2f64.sqrt()),
                    );
                    self.vec_s[pivot] = true;
                    self._right_multiply_s(pivot);
                } else {
                    self.vec_s[pivot] = false;
                    self.vec_v[pivot] = true;
                }
            }
            PhaseFactor::MinusI => {
                if self.vec_v[pivot] {
                    // rotate -45 deg
                    self.set_global_phase(
                        self.global_phase()
                            * num_complex::Complex64::new(1.0 / 2f64.sqrt(), -1.0 / 2f64.sqrt()),
                    );
                    self.vec_s[pivot] = false;
                    self._right_multiply_s(pivot);
                } else {
                    self.vec_s[pivot] = true;
                    self.vec_v[pivot] = true;
                }
            }
        }
    }

    fn _handle_same_vecs_case(&mut self, delta: PhaseFactor, vec_t: &ndarray::Array1<bool>) {
        match delta {
            PhaseFactor::PlusOne => {
                self.vec_s = vec_t.clone();
            }
            PhaseFactor::MinusOne => {
                panic!("Inconsistent state: superposition with -1 coefficient.");
            }
            PhaseFactor::PlusI => {
                self.vec_s = vec_t.clone();
                // rotate 45 deg
                self.set_global_phase(
                    self.global_phase()
                        * num_complex::Complex64::new(1.0 / 2f64.sqrt(), 1.0 / 2f64.sqrt()),
                );
            }
            PhaseFactor::MinusI => {
                self.vec_s = vec_t.clone();
                // rotate -45 deg
                self.set_global_phase(
                    self.global_phase()
                        * num_complex::Complex64::new(1.0 / 2f64.sqrt(), -1.0 / 2f64.sqrt()),
                );
            }
        }
    }

    fn _get_differing_indices(
        &self,
        vec_t: &ndarray::Array1<bool>,
        vec_u: &ndarray::Array1<bool>,
    ) -> (Vec<usize>, Vec<usize>) {
        let mut diff_indices_0 = Vec::new();
        let mut diff_indices_1 = Vec::new();
        for i in 0..self.n {
            if vec_t[i] != vec_u[i] {
                if self.vec_v[i] {
                    diff_indices_1.push(i);
                } else {
                    diff_indices_0.push(i);
                }
            }
        }

        (diff_indices_0, diff_indices_1)
    }

    fn _apply_basis_transform_circuit(
        &mut self,
        diff_indices_0: &[usize],
        diff_indices_1: &[usize],
    ) -> usize {
        if diff_indices_0.is_empty() {
            let pivot = diff_indices_1[0];
            for &i in &diff_indices_1[1..] {
                self._right_multiply_cx(i, pivot);
            }
            pivot
        } else {
            let pivot = diff_indices_0[0];
            for &i in &diff_indices_0[1..] {
                self._right_multiply_cx(pivot, i);
            }
            for &i in diff_indices_1 {
                self._right_multiply_cz(pivot, i);
            }
            pivot
        }
    }
}
