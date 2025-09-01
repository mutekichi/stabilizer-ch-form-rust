pub trait CZGate {
    fn apply_cz(&mut self, qarg1: usize, qarg2: usize);
}