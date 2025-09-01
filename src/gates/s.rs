use crate::{PhaseFactor, StabilizerCHForm};

pub trait SGate {
    fn apply_s(&mut self, qarg: usize);
    fn apply_sdg(&mut self, qarg: usize);
}

impl StabilizerCHForm {
    /// Applies the Phase (S) gate to the qubit at index `qarg`.
    ///     
    /// Time complexity: O(n)
    /// 
    /// See around the end of Proposition 4 of arXiv:1808.00128 for details.
    pub fn apply_s(&mut self, qarg: usize) {
        if qarg >= self.n {
            panic!("Qubit index out of bounds.");
        }

        let g_row = self.mat_g.row(qarg).to_owned();
        let mut m_row = self.mat_m.row_mut(qarg);
        m_row ^= &g_row;

        self.gamma[qarg] *= PhaseFactor::MinusI;
    }

    
    /// Applies the adjoint Phase (S†) gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n)
    pub fn apply_sdg(&mut self, qarg: usize) {
        if qarg >= self.n {
            panic!("Qubit index out of bounds.");
        }

        let g_row = self.mat_g.row(qarg).to_owned();
        let mut m_row = self.mat_m.row_mut(qarg);
        m_row ^= &g_row;

        self.gamma[qarg] *= PhaseFactor::PlusI;
    }
}