pub trait HGate {
    fn apply_h(&mut self, qarg: usize);
}