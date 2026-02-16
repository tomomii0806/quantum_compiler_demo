use super::gate::Gate;

#[derive(Debug)]
pub struct Circuit {
    pub num_qubits: usize,
    pub gates: Vec<Gate>,
}

impl Circuit {
    pub fn new(num_qubits: usize) -> Self {
        Self { num_qubits, gates: Vec::new() }
    }

    pub fn add_gate(&mut self, gate: Gate) {
        // Basic validation: check qubit indices are in range
        match gate {
            Gate::H(q) | Gate::X(q) | Gate::Y(q) | Gate::Z(q) | Gate::S(q) | Gate::T(q) => {
                if q >= self.num_qubits {
                    panic!("Qubit index out of range!");
                }
            }
            Gate::Rx(q, _) | Gate::Ry(q, _) | Gate::Rz(q, _) => {
                if q >= self.num_qubits {
                    panic!("Qubit index out of range!");
                }
            }
            Gate::CNOT(c, t) | Gate::SWAP(c, t) => {
                if c >= self.num_qubits || t >= self.num_qubits || c == t {
                    panic!("Invalid two-qubit gate!");
                }
            }
        }
        self.gates.push(gate);
    }
}
