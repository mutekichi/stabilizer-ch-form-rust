#[derive(Debug, Clone, PartialEq)]
pub enum CliffordGate {
    H(usize),
    S(usize),
    Sdg(usize),
    X(usize),
    Y(usize),
    Z(usize),
    SqrtX(usize),
    SqrtXdg(usize),
    CX(usize, usize),
    CZ(usize, usize),
    Swap(usize, usize),
}