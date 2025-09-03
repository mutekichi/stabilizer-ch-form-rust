use crate::StabilizerCHForm;

impl StabilizerCHForm {
    pub fn inner_product(&self, other: &StabilizerCHForm) -> num_complex::Complex64 {
        self._inner_product(other)
    }
}
