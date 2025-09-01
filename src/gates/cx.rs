pub trait CXGate {
    fn apply_cx(&mut self, control: usize, target: usize);
}