pub mod apply_gate;
pub mod qasm_parser;

#[derive(Debug, Clone, PartialEq)]
pub enum QuantumGate {
    H(usize),
    S(usize),
    X(usize),
    Z(usize),
    CX(usize, usize),
    CZ(usize, usize),
}

pub struct QuantumCircuit {
    pub n_qubits: usize,
    pub gates: Vec<QuantumGate>,
}

impl QuantumCircuit {
    pub fn new(n_qubits: usize) -> Self {
        if n_qubits == 0 {
            panic!("Number of qubits must be greater than zero.");
        }
        Self {
            n_qubits,
            gates: Vec::new(),
        }
    }
}
