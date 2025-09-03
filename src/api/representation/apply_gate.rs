use crate::api::{QuantumCircuit, QuantumGate};

impl QuantumCircuit {
    pub fn add_gate(&mut self, gate: QuantumGate) {
        match &gate {
            QuantumGate::H(q) | QuantumGate::S(q) | QuantumGate::X(q) | QuantumGate::Z(q) => {
                if *q >= self.n_qubits {
                    panic!("Qubit index out of bounds.");
                }
            }
            QuantumGate::CX(control, target) | QuantumGate::CZ(control, target) => {
                if *control >= self.n_qubits || *target >= self.n_qubits {
                    panic!("Qubit index out of bounds.");
                }
                if control == target {
                    panic!("Control and target qubits must be different.");
                }
            }
        }
        self.gates.push(gate);
    }

    pub fn add_multiple_gates(&mut self, gates: Vec<QuantumGate>) {
        for gate in gates {
            self.add_gate(gate);
        }
    }

    pub fn apply_h(&mut self, qarg: usize) {
        self.add_gate(QuantumGate::H(qarg));
    }

    pub fn apply_x(&mut self, qarg: usize) {
        self.add_gate(QuantumGate::X(qarg));
    }

    pub fn apply_s(&mut self, qarg: usize) {
        self.add_gate(QuantumGate::S(qarg));
    }

    pub fn apply_z(&mut self, qarg: usize) {
        self.add_gate(QuantumGate::Z(qarg));
    }

    pub fn apply_cx(&mut self, control: usize, target: usize) {
        self.add_gate(QuantumGate::CX(control, target));
    }

    pub fn apply_cz(&mut self, control: usize, target: usize) {
        self.add_gate(QuantumGate::CZ(control, target));
    }
}
