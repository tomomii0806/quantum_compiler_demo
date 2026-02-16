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
}
