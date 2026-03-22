use crate::model::{Circuit, Gate, LogicalQubit, EncodingType};

/// 3-qubit bit-flip repetition code
/// Encodes 1 logical qubit into 3 physical qubits
/// Can detect (but not correct) 1 bit-flip (X) error
/// Encoding: |0⟩ → |000⟩, |1⟩ → |111⟩
pub struct BitFlipCode {
    /// Indices of the 3 physical qubits
    qubits: [usize; 3],
}

impl BitFlipCode {
    /// Create a new bit-flip code using specified qubit indices
    pub fn new(qubits: [usize; 3]) -> Self {
        Self { qubits }
    }
    
    /// Generate encoding circuit
    /// Takes logical state on qubit[0] and encodes into all 3 qubits
    /// |ψ⟩ → |ψψψ⟩ using CNOT gates
    pub fn encode(&self, circuit: &mut Circuit) {
        let [q0, q1, q2] = self.qubits;
        // CNOT from q0 to q1 and q2 to create |ψψψ⟩
        circuit.add_gate(Gate::CNOT(q0, q1));
        circuit.add_gate(Gate::CNOT(q0, q2));
    }
    
    /// Measure syndrome (error pattern) using 2 ancilla qubits
    /// Returns the syndrome measurement circuit
    /// Syndrome bits indicate which qubit (if any) has an error
    pub fn measure_syndrome(&self, circuit: &mut Circuit, ancilla: [usize; 2]) {
        let [q0, q1, q2] = self.qubits;
        let [a0, a1] = ancilla;
        
        // First ancilla checks parity of q0 and q1
        circuit.add_gate(Gate::CNOT(q0, a0));
        circuit.add_gate(Gate::CNOT(q1, a0));
        circuit.add_gate(Gate::Measure(a0));
        
        // Second ancilla checks parity of q1 and q2
        circuit.add_gate(Gate::CNOT(q1, a1));
        circuit.add_gate(Gate::CNOT(q2, a1));
        circuit.add_gate(Gate::Measure(a1));
    }
    
    /// Decode syndrome to determine error location
    /// Returns which qubit has an error (if any)
    /// - (false, false) → no error
    /// - (true, false) → error on qubit 0
    /// - (true, true) → error on qubit 1
    /// - (false, true) → error on qubit 2
    pub fn detect_error(&self, syndrome: (bool, bool)) -> Option<usize> {
        match syndrome {
            (false, false) => None,           // No error
            (true, false) => Some(self.qubits[0]),  // Error on q0
            (true, true) => Some(self.qubits[1]),   // Error on q1
            (false, true) => Some(self.qubits[2]),  // Error on q2
        }
    }
    
    /// Create a LogicalQubit representation
    pub fn as_logical_qubit(&self) -> LogicalQubit {
        LogicalQubit::new(
            self.qubits.to_vec(),
            EncodingType::BitFlipRepetition,
        )
    }
}

/// 3-qubit phase-flip repetition code
/// Encodes 1 logical qubit into 3 physical qubits
/// Can detect (but not correct) 1 phase-flip (Z) error
/// Encoding: |+⟩ → |+++⟩, |−⟩ → |−−−⟩
/// (This is bit-flip code in the X basis)
pub struct PhaseFlipCode {
    /// Indices of the 3 physical qubits
    qubits: [usize; 3],
}

impl PhaseFlipCode {
    pub fn new(qubits: [usize; 3]) -> Self {
        Self { qubits }
    }
    
    /// Generate encoding circuit
    /// Encodes in X basis: apply Hadamard before and after bit-flip encoding
    pub fn encode(&self, circuit: &mut Circuit) {
        let [q0, q1, q2] = self.qubits;
        
        // Transform to X basis
        circuit.add_gate(Gate::H(q0));
        
        // Encode like bit-flip code
        circuit.add_gate(Gate::CNOT(q0, q1));
        circuit.add_gate(Gate::CNOT(q0, q2));
        
        // Transform back to Z basis
        circuit.add_gate(Gate::H(q0));
        circuit.add_gate(Gate::H(q1));
        circuit.add_gate(Gate::H(q2));
    }
    
    /// Measure syndrome for phase errors
    pub fn measure_syndrome(&self, circuit: &mut Circuit, ancilla: [usize; 2]) {
        let [q0, q1, q2] = self.qubits;
        let [a0, a1] = ancilla;
        
        // Transform to X basis for measurement
        circuit.add_gate(Gate::H(q0));
        circuit.add_gate(Gate::H(q1));
        circuit.add_gate(Gate::H(q2));
        
        // Measure parities (same as bit-flip in X basis)
        circuit.add_gate(Gate::CNOT(q0, a0));
        circuit.add_gate(Gate::CNOT(q1, a0));
        circuit.add_gate(Gate::Measure(a0));
        
        circuit.add_gate(Gate::CNOT(q1, a1));
        circuit.add_gate(Gate::CNOT(q2, a1));
        circuit.add_gate(Gate::Measure(a1));
        
        // Transform back to Z basis
        circuit.add_gate(Gate::H(q0));
        circuit.add_gate(Gate::H(q1));
        circuit.add_gate(Gate::H(q2));
    }
    
    /// Decode syndrome to determine phase error location
    pub fn detect_error(&self, syndrome: (bool, bool)) -> Option<usize> {
        match syndrome {
            (false, false) => None,
            (true, false) => Some(self.qubits[0]),
            (true, true) => Some(self.qubits[1]),
            (false, true) => Some(self.qubits[2]),
        }
    }
    
    pub fn as_logical_qubit(&self) -> LogicalQubit {
        LogicalQubit::new(
            self.qubits.to_vec(),
            EncodingType::PhaseFlipRepetition,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitflip_encoding() {
        let code = BitFlipCode::new([0, 1, 2]);
        let mut circuit = Circuit::new(3);
        code.encode(&mut circuit);
        
        // Should have 2 CNOT gates
        assert_eq!(circuit.gates.len(), 2);
        assert_eq!(circuit.gates[0], Gate::CNOT(0, 1));
        assert_eq!(circuit.gates[1], Gate::CNOT(0, 2));
    }
    
    #[test]
    fn test_bitflip_syndrome_measurement() {
        let code = BitFlipCode::new([0, 1, 2]);
        let mut circuit = Circuit::new(5); // 3 data + 2 ancilla
        code.measure_syndrome(&mut circuit, [3, 4]);
        
        // Should have 2 CNOTs + 1 measurement, then 2 CNOTs + 1 measurement
        assert_eq!(circuit.gates.len(), 6);
    }
    
    #[test]
    fn test_bitflip_error_detection() {
        let code = BitFlipCode::new([0, 1, 2]);
        
        assert_eq!(code.detect_error((false, false)), None);
        assert_eq!(code.detect_error((true, false)), Some(0));
        assert_eq!(code.detect_error((true, true)), Some(1));
        assert_eq!(code.detect_error((false, true)), Some(2));
    }
    
    #[test]
    fn test_phaseflip_encoding() {
        let code = PhaseFlipCode::new([0, 1, 2]);
        let mut circuit = Circuit::new(3);
        code.encode(&mut circuit);
        
        // Should have: H, CNOT, CNOT, H, H, H
        assert_eq!(circuit.gates.len(), 6);
        assert_eq!(circuit.gates[0], Gate::H(0));
        assert_eq!(circuit.gates[1], Gate::CNOT(0, 1));
        assert_eq!(circuit.gates[2], Gate::CNOT(0, 2));
    }
    
    #[test]
    fn test_phaseflip_error_detection() {
        let code = PhaseFlipCode::new([5, 6, 7]);
        
        assert_eq!(code.detect_error((false, false)), None);
        assert_eq!(code.detect_error((true, false)), Some(5));
        assert_eq!(code.detect_error((true, true)), Some(6));
        assert_eq!(code.detect_error((false, true)), Some(7));
    }
    
    #[test]
    fn test_logical_qubit_creation() {
        let code = BitFlipCode::new([0, 1, 2]);
        let logical = code.as_logical_qubit();
        
        assert_eq!(logical.num_physical_qubits(), 3);
        assert!(logical.is_encoded());
        assert_eq!(logical.encoding, EncodingType::BitFlipRepetition);
    }
}
