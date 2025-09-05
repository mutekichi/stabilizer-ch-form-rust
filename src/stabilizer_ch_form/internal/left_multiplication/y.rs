use crate::{stabilizer_ch_form::internal::types::PhaseFactor, StabilizerCHForm};

impl StabilizerCHForm {
    pub(crate) fn _left_multiply_y(&mut self, qarg: usize) {
        self._left_multiply_z(qarg);
        self._left_multiply_x(qarg);
        self.phase_factor *= PhaseFactor::PLUS_I;
    }
}