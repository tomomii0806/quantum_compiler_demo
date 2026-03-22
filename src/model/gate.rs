#[derive(Debug, Clone, PartialEq)]  // Derives for printing, copying, and equality checks
pub enum Gate {
    // Single-qubit gates
    H(usize),    // Hadamard on qubit index
    X(usize),    // Pauli-X (NOT)
    Y(usize),    // Pauli-Y
    Z(usize),    // Pauli-Z
    S(usize),    // Phase gate (sqrt(Z))
    T(usize),    // T gate (fourth root of Z)

    // Rotation gates (angle in radians)
    Rx(usize, f64),  // Rotation around X-axis
    Ry(usize, f64),  // Rotation around Y-axis
    Rz(usize, f64),  // Rotation around Z-axis

    // Two-qubit gates
    CNOT(usize, usize),  // Control on first, target on second
    SWAP(usize, usize),  // Swap two qubits
    CZ(usize, usize),    // Controlled-Z gate
    
    // Measurement
    Measure(usize),      // Measure qubit in Z-basis
    MeasureX(usize),     // Measure qubit in X-basis
    MeasureY(usize),     // Measure qubit in Y-basis
}

/// Represents different types of quantum errors
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorType {
    BitFlip,    // X error
    PhaseFlip,  // Z error
    BitPhase,   // Y error (both X and Z)
}

impl ErrorType {
    pub fn to_gate(&self, qubit: usize) -> Gate {
        match self {
            ErrorType::BitFlip => Gate::X(qubit),
            ErrorType::PhaseFlip => Gate::Z(qubit),
            ErrorType::BitPhase => Gate::Y(qubit),
        }
    }
}
