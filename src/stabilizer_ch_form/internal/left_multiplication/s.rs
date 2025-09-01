use crate::stabilizer_ch_form::{internal::types::PhaseFactor, StabilizerCHForm};

impl StabilizerCHForm {
    pub(crate) fn _left_multiply_s(&mut self, qarg: usize) {
        if qarg >= self.n {
            // TODO: better error handling
            panic!("Qubit index out of bounds.");
        }
        let g_row = self.mat_g.row(qarg).to_owned();
        let mut m_row = self.mat_m.row_mut(qarg);
        m_row ^= &g_row;
        self.gamma[qarg] *= PhaseFactor::MinusI;
    }

    pub(crate) fn _left_multiply_sdg(&mut self, qarg: usize) {
        if qarg >= self.n {
            panic!("Qubit index out of bounds.");
        }
        let g_row = self.mat_g.row(qarg).to_owned();
        let mut m_row = self.mat_m.row_mut(qarg);
        m_row ^= &g_row;
        self.gamma[qarg] *= PhaseFactor::PlusI;
    }
}