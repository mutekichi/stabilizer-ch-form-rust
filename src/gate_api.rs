use super::StabilizerCHForm;
use ndarray::Array1;

use crate::phase_factor::PhaseFactor;

impl StabilizerCHForm {
    pub fn apply_x(&mut self, qarg: usize) {
        // Applies the Pauli-X gate to the qubit at index `qarg`.
        // 
        // See around eq.(48) of arXiv:1808.00128 for details.
        
        if qarg >= self.n {
            panic!("Qubit index out of bounds.");
        }
        // calculate u appearing in eq.(48) of arXiv:1808.00128 :
        // $$
        // u_j = s_j \oplus (F_{p,j} \land \lnot v_j) \oplus (M_{p,j} \land v_j)
        // $$
        let f_row = self.mat_f.row(qarg);
        let m_row = self.mat_m.row(qarg);

        let u: Array1<bool> = self.vec_s.iter()
            .zip(self.vec_v.iter())
            .zip(f_row.iter())
            .zip(m_row.iter())
            .map(|(((&s, &v), &f), &m)| {
                s ^ (f & !v) ^ (m & v)
            })
            .collect();

        let term1 = m_row.iter().zip(&self.vec_v).zip(&self.vec_s).filter(|&((&m, &v), &s)| m && !v && s).count();
        let term2 = f_row.iter().zip(&self.vec_v).zip(m_row).filter(|&((&f, &v), &m)| f && v && m).count();
        let term3 = f_row.iter().zip(&self.vec_v).zip(&self.vec_s).filter(|&((&f, &v), &s)| f && v && s).count();
        let beta = (term1 + term2 + term3) % 2;

        if beta == 1 {
            self.phase_factor = self.phase_factor.flip_sign();
        }

        self.vec_s = u;
    }

    pub fn apply_s(&mut self, qarg: usize) {
        // Applies the Phase (S) gate to the qubit at index `qarg`.
        // 
        // See around the end of Proposition 4 of arXiv:1808.00128 for details.

        if qarg >= self.n {
            panic!("Qubit index out of bounds.");
        }

        let g_row = self.mat_g.row(qarg).to_owned();
        let mut m_row = self.mat_m.row_mut(qarg);
        m_row ^= &g_row;

        self.gamma[qarg] *= PhaseFactor::MinusI;
    }

    pub fn apply_z(&mut self, qarg: usize) {
        // Applies the Pauli-Z gate to the qubit at index `qarg`.
        // 
        // NOTE: temporarily implemented as Z = SS, which might be inefficient.

        if qarg >= self.n {
            panic!("Qubit index out of bounds.");
        }

        self.apply_s(qarg);
        self.apply_s(qarg);
    }

}