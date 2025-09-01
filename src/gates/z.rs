use crate::StabilizerCHForm;

pub trait ZGate {
    fn apply_z(&mut self, qarg: usize);
}

impl StabilizerCHForm{
    /// Applies the Pauli-Z gate to the qubit at index `qarg`.
    /// 
    /// Time complexity: O(n)
    ///
    /// NOTE: temporarily implemented as Z = SS, which might be inefficient.
    pub fn apply_z(&mut self, qarg: usize) {
        if qarg >= self.n {
            panic!("Qubit index out of bounds.");
        }

        self.apply_s(qarg);
        self.apply_s(qarg);
    }
}