use crate::stabilizer_ch_form::StabilizerCHForm;

impl StabilizerCHForm {
    pub(crate) fn _left_multiply_z(&mut self, qarg: usize) {
        if qarg >= self.n {
            // TODO: better error handling
            panic!("Qubit index out of bounds.");
        }
        self.gamma[qarg].flipped();
    }
}
