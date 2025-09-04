use crate::StabilizerCHForm;

impl StabilizerCHForm {
    pub fn project(&mut self, qarg: usize, outcome: bool) -> Result<(), &'static str> {
        self._project(qarg, outcome)
    }
}
