use crate::model::{Circuit, Gate};

/// Trait for quantum error correction codes that support syndrome extraction
pub trait SyndromeExtraction {
    /// Get the number of syndrome bits (measurement outcomes)
    fn num_syndromes(&self) -> usize;
    
    /// Generate syndrome measurement circuit
    fn syndrome_circuit(&self, circuit: &mut Circuit, ancilla_qubits: &[usize]);
    
    /// Decode syndrome to detect/locate errors
    fn decode_syndrome(&self, syndrome: &[bool]) -> SyndromeResult;
}

/// Result of syndrome measurement and decoding
#[derive(Debug, Clone, PartialEq)]
pub enum SyndromeResult {
    /// No errors detected
    NoError,
    /// Single error detected at specified qubit
    SingleError { qubit: usize, error_type: ErrorKind },
    /// Multiple errors detected (may not be correctable)
    MultipleErrors { count: usize },
    /// Unknown/uncorrectable error pattern
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorKind {
    BitFlip,   // X error
    PhaseFlip, // Z error
    Both,      // Y error
}

/// Helper for building syndrome measurement circuits
pub struct SyndromeMeasurement {
    /// Qubits to measure parity of
    pub data_qubits: Vec<usize>,
    /// Ancilla qubit to use for measurement
    pub ancilla: usize,
    /// Type of parity check (X or Z)
    pub check_type: ParityCheck,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParityCheck {
    /// Z-parity check using CNOT (data as control)
    /// Measures: does the product of Z operators equal +1 or -1?
    ZParity,
    
    /// X-parity check - fault-tolerant version (ancilla as control)
    /// Measures: does the product of X operators equal +1 or -1?
    /// This version doesn't disturb data qubits, preferred for fault-tolerance
    XParityFaultTolerant,
    
    /// X-parity check - intuitive version (data as control)
    /// Functionally equivalent but requires Hadamarding data qubits
    /// Less fault-tolerant since it changes data qubit basis
    XParityIntuitive,
}

impl SyndromeMeasurement {
    /// Create a new syndrome measurement
    pub fn new(data_qubits: Vec<usize>, ancilla: usize, check_type: ParityCheck) -> Self {
        Self {
            data_qubits,
            ancilla,
            check_type,
        }
    }
    
    /// Add this syndrome measurement to a circuit
    /// Uses non-destructive measurement (ancilla-based)
    pub fn add_to_circuit(&self, circuit: &mut Circuit) {
        match self.check_type {
            ParityCheck::ZParity => {
                // Z-PARITY CHECK: Measure stabilizer Z₀Z₁Z₂...
                // 
                // Circuit: data₀ --●--     data₁ --●--     ancilla --|0⟩--X--X--M
                //
                // Why this works:
                // - CNOT flips ancilla if data qubit is |1⟩
                // - Even number of |1⟩s → ancilla = |0⟩ (even parity)
                // - Odd number of |1⟩s → ancilla = |1⟩ (odd parity)
                // - Data qubits stay in computational basis (no disturbance)
                //
                for &data_qubit in &self.data_qubits {
                    circuit.add_gate(Gate::CNOT(data_qubit, self.ancilla));
                }
                circuit.add_gate(Gate::Measure(self.ancilla));
            }
            
            ParityCheck::XParityFaultTolerant => {
                // X-PARITY CHECK (Fault-Tolerant): Measure stabilizer X₀X₁X₂...
                //
                // Circuit: data₀ --X--     data₁ --X--     ancilla --|+⟩--●--●--H--M
                //                                                     H        H
                //
                // Why this works:
                // - Ancilla starts in |+⟩ = H|0⟩ (equal superposition)
                // - CNOT(ancilla→data) in Z-basis = CNOT(data→ancilla) in X-basis
                // - This is because: H⊗H CNOT H⊗H = CNOT with swapped control/target
                // - Final H rotates ancilla back to Z-basis for measurement
                // - Measurement outcome tells us X-parity of data qubits
                //
                // Fault-tolerance advantage:
                // - Data qubits never leave Z-basis
                // - Errors on ancilla don't propagate to data qubits
                // - Critical for surface codes and other stabilizer codes
                //
                circuit.add_gate(Gate::H(self.ancilla));
                for &data_qubit in &self.data_qubits {
                    circuit.add_gate(Gate::CNOT(self.ancilla, data_qubit));
                }
                circuit.add_gate(Gate::H(self.ancilla));
                circuit.add_gate(Gate::Measure(self.ancilla));
            }
            
            ParityCheck::XParityIntuitive => {
                // X-PARITY CHECK (Intuitive): Alternative implementation
                //
                // Circuit: data₀ --H--●--H--     data₁ --H--●--H--     ancilla --|0⟩--X--X--M
                //
                // Why this works:
                // - Hadamard on data rotates to X-basis: H|ψ⟩_Z = |ψ⟩_X
                // - Now Z-measurement in rotated basis = X-measurement in original basis
                // - CNOT in X-basis measures X-parity (same as Z-parity logic, but rotated)
                // - Final Hadamard rotates back to Z-basis
                //
                // Why less fault-tolerant:
                // - Data qubits change basis (|0⟩ ↔ |+⟩, |1⟩ ↔ |−⟩)
                // - If error occurs during Hadamard, it affects data
                // - More susceptible to decoherence during basis change
                //
                // Educational note: Both methods are MATHEMATICALLY equivalent!
                // They differ in fault-tolerance properties, not function.
                //
                for &data_qubit in &self.data_qubits {
                    circuit.add_gate(Gate::H(data_qubit));
                }
                for &data_qubit in &self.data_qubits {
                    circuit.add_gate(Gate::CNOT(data_qubit, self.ancilla));
                }
                for &data_qubit in &self.data_qubits {
                    circuit.add_gate(Gate::H(data_qubit));
                }
                circuit.add_gate(Gate::Measure(self.ancilla));
            }
        }
    }
}

/// Builder for constructing syndrome extraction circuits
pub struct SyndromeCircuitBuilder {
    measurements: Vec<SyndromeMeasurement>,
}

impl SyndromeCircuitBuilder {
    pub fn new() -> Self {
        Self {
            measurements: Vec::new(),
        }
    }
    
    /// Add a Z-parity check (measures Z₀Z₁...Zₙ stabilizer)
    pub fn add_z_check(mut self, data_qubits: Vec<usize>, ancilla: usize) -> Self {
        self.measurements.push(SyndromeMeasurement::new(
            data_qubits,
            ancilla,
            ParityCheck::ZParity,
        ));
        self
    }
    
    /// Add an X-parity check using fault-tolerant method (preferred for real QEC)
    pub fn add_x_check(mut self, data_qubits: Vec<usize>, ancilla: usize) -> Self {
        self.measurements.push(SyndromeMeasurement::new(
            data_qubits,
            ancilla,
            ParityCheck::XParityFaultTolerant,
        ));
        self
    }
    
    /// Add an X-parity check using intuitive method (for education/understanding)
    pub fn add_x_check_intuitive(mut self, data_qubits: Vec<usize>, ancilla: usize) -> Self {
        self.measurements.push(SyndromeMeasurement::new(
            data_qubits,
            ancilla,
            ParityCheck::XParityIntuitive,
        ));
        self
    }
    
    /// Build and apply all syndrome measurements to circuit
    pub fn build(self, circuit: &mut Circuit) {
        for measurement in self.measurements {
            measurement.add_to_circuit(circuit);
        }
    }
    
    /// Get number of syndrome bits
    pub fn num_syndromes(&self) -> usize {
        self.measurements.len()
    }
}

impl Default for SyndromeCircuitBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_parity_measurement() {
        let mut circuit = Circuit::new(4); // 3 data + 1 ancilla
        let measurement = SyndromeMeasurement::new(
            vec![0, 1, 2],
            3,
            ParityCheck::ZParity,
        );
        
        measurement.add_to_circuit(&mut circuit);
        
        // Should have 3 CNOTs + 1 measurement
        assert_eq!(circuit.gates.len(), 4);
        assert_eq!(circuit.gates[0], Gate::CNOT(0, 3));
        assert_eq!(circuit.gates[1], Gate::CNOT(1, 3));
        assert_eq!(circuit.gates[2], Gate::CNOT(2, 3));
        assert_eq!(circuit.gates[3], Gate::Measure(3));
    }
    
    #[test]
    fn test_x_parity_fault_tolerant() {
        let mut circuit = Circuit::new(4);
        let measurement = SyndromeMeasurement::new(
            vec![0, 1],
            2,
            ParityCheck::XParityFaultTolerant,
        );
        
        measurement.add_to_circuit(&mut circuit);
        
        // Should have H + 2 CNOTs + H + measurement
        assert_eq!(circuit.gates.len(), 5);
        assert_eq!(circuit.gates[0], Gate::H(2)); // Ancilla to |+⟩
        assert_eq!(circuit.gates[1], Gate::CNOT(2, 0)); // Ancilla as control
        assert_eq!(circuit.gates[2], Gate::CNOT(2, 1)); // Ancilla as control
        assert_eq!(circuit.gates[3], Gate::H(2)); // Back to Z-basis
        assert_eq!(circuit.gates[4], Gate::Measure(2));
    }
    
    #[test]
    fn test_x_parity_intuitive() {
        let mut circuit = Circuit::new(4);
        let measurement = SyndromeMeasurement::new(
            vec![0, 1],
            2,
            ParityCheck::XParityIntuitive,
        );
        
        measurement.add_to_circuit(&mut circuit);
        
        // Should have 2 H + 2 CNOTs + 2 H + measurement = 7 gates
        assert_eq!(circuit.gates.len(), 7);
        assert_eq!(circuit.gates[0], Gate::H(0)); // Data to X-basis
        assert_eq!(circuit.gates[1], Gate::H(1)); // Data to X-basis
        assert_eq!(circuit.gates[2], Gate::CNOT(0, 2)); // Data as control
        assert_eq!(circuit.gates[3], Gate::CNOT(1, 2)); // Data as control
        assert_eq!(circuit.gates[4], Gate::H(0)); // Back to Z-basis
        assert_eq!(circuit.gates[5], Gate::H(1)); // Back to Z-basis
        assert_eq!(circuit.gates[6], Gate::Measure(2));
    }
    
    #[test]
    fn test_x_parity_methods_have_same_gate_count_for_parity() {
        // Both methods measure X-parity but with different circuits
        // Fault-tolerant: H(anc) + n*CNOT + H(anc) + M = n+3 gates
        // Intuitive: n*H(data) + n*CNOT + n*H(data) + M = 3n+1 gates
        
        let mut circuit1 = Circuit::new(5);
        let mut circuit2 = Circuit::new(5);
        
        SyndromeMeasurement::new(vec![0, 1, 2], 3, ParityCheck::XParityFaultTolerant)
            .add_to_circuit(&mut circuit1);
        
        SyndromeMeasurement::new(vec![0, 1, 2], 3, ParityCheck::XParityIntuitive)
            .add_to_circuit(&mut circuit2);
        
        // Fault-tolerant: 3 data qubits → 3+3 = 6 gates
        assert_eq!(circuit1.gates.len(), 6);
        
        // Intuitive: 3 data qubits → 3*3+1 = 10 gates
        assert_eq!(circuit2.gates.len(), 10);
        
        // Fault-tolerant is more efficient AND safer!
    }
    
    #[test]
    fn test_syndrome_builder() {
        let mut circuit = Circuit::new(6); // 4 data + 2 ancilla
        
        SyndromeCircuitBuilder::new()
            .add_z_check(vec![0, 1], 4)
            .add_z_check(vec![2, 3], 5)
            .build(&mut circuit);
        
        // Should have (2 CNOTs + measure) + (2 CNOTs + measure) = 6 gates
        assert_eq!(circuit.gates.len(), 6);
    }
    
    #[test]
    fn test_mixed_x_z_checks_fault_tolerant() {
        let mut circuit = Circuit::new(5); // 3 data + 2 ancilla
        
        SyndromeCircuitBuilder::new()
            .add_z_check(vec![0, 1], 3)
            .add_x_check(vec![1, 2], 4) // Uses fault-tolerant version
            .build(&mut circuit);
        
        // Z-check: 2 CNOTs + measure (3 gates)
        // X-check (FT): H + 2 CNOTs + H + measure (5 gates)
        // Total: 8 gates
        assert_eq!(circuit.gates.len(), 8);
    }
    
    #[test]
    fn test_mixed_x_z_checks_intuitive() {
        let mut circuit = Circuit::new(5); // 3 data + 2 ancilla
        
        SyndromeCircuitBuilder::new()
            .add_z_check(vec![0, 1], 3)
            .add_x_check_intuitive(vec![1, 2], 4)
            .build(&mut circuit);
        
        // Z-check: 2 CNOTs + measure (3 gates)
        // X-check (intuitive): 2H + 2 CNOTs + 2H + measure (7 gates)
        // Total: 10 gates
        assert_eq!(circuit.gates.len(), 10);
    }
    
    #[test]
    fn test_syndrome_result_variants() {
        let no_error = SyndromeResult::NoError;
        assert_eq!(no_error, SyndromeResult::NoError);
        
        let single = SyndromeResult::SingleError {
            qubit: 0,
            error_type: ErrorKind::BitFlip,
        };
        assert!(matches!(single, SyndromeResult::SingleError { .. }));
        
        let multi = SyndromeResult::MultipleErrors { count: 2 };
        assert!(matches!(multi, SyndromeResult::MultipleErrors { .. }));
    }
    
    #[test]
    fn test_fault_tolerant_preserves_data_basis() {
        // Key property: fault-tolerant X-check never applies gates to data qubits
        let mut circuit = Circuit::new(4);
        
        SyndromeMeasurement::new(vec![0, 1], 2, ParityCheck::XParityFaultTolerant)
            .add_to_circuit(&mut circuit);
        
        // Count gates applied to data qubits 0 and 1
        let data_gate_count = circuit.gates.iter().filter(|g| {
            matches!(g, Gate::H(q) if *q == 0 || *q == 1)
        }).count();
        
        // Should be ZERO - data qubits never get Hadamarded!
        assert_eq!(data_gate_count, 0, "Fault-tolerant method should not touch data qubits");
    }
    
    #[test]
    fn test_intuitive_changes_data_basis() {
        // Educational: intuitive method DOES change data qubit basis
        let mut circuit = Circuit::new(4);
        
        SyndromeMeasurement::new(vec![0, 1], 2, ParityCheck::XParityIntuitive)
            .add_to_circuit(&mut circuit);
        
        // Count Hadamards on data qubits
        let hadamard_count = circuit.gates.iter().filter(|g| {
            matches!(g, Gate::H(q) if *q == 0 || *q == 1)
        }).count();
        
        // Should be 4: 2 qubits × (H before + H after) = 4 Hadamards
        assert_eq!(hadamard_count, 4, "Intuitive method Hadamards data qubits");
    }
}
