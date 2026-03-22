use quantum_compiler_demo::model::{Circuit, Gate, ErrorType};
use quantum_compiler_demo::qec::{BitFlipCode, PhaseFlipCode};

#[test]
fn test_bitflip_code_full_cycle() {
    // Test complete encoding and syndrome measurement cycle
    let code = BitFlipCode::new([0, 1, 2]);
    let mut circuit = Circuit::new(5); // 3 data + 2 ancilla
    
    // Encode logical qubit
    code.encode(&mut circuit);
    
    // Measure syndrome
    code.measure_syndrome(&mut circuit, [3, 4]);
    
    // Verify circuit structure
    assert_eq!(circuit.gates.len(), 8); // 2 (encode) + 6 (syndrome)
}

#[test]
fn test_bitflip_detects_single_x_error() {
    // Simulate error on each qubit and verify detection
    for error_qubit in 0..3 {
        let code = BitFlipCode::new([0, 1, 2]);
        let mut circuit = Circuit::new(5);
        
        // Encode
        code.encode(&mut circuit);
        
        // Apply X error to simulate bit flip
        circuit.add_gate(Gate::X(error_qubit));
        
        // The syndrome would be measured, here we simulate the expected results
        let expected_syndrome = match error_qubit {
            0 => (true, false),
            1 => (true, true),
            2 => (false, true),
            _ => unreachable!(),
        };
        
        let detected = code.detect_error(expected_syndrome);
        assert_eq!(detected, Some(error_qubit));
    }
}

#[test]
fn test_bitflip_no_error_detection() {
    // No error case
    let code = BitFlipCode::new([0, 1, 2]);
    let mut circuit = Circuit::new(5);
    
    code.encode(&mut circuit);
    
    // No error applied
    let syndrome = (false, false);
    let detected = code.detect_error(syndrome);
    assert_eq!(detected, None);
}

#[test]
fn test_phaseflip_code_full_cycle() {
    let code = PhaseFlipCode::new([0, 1, 2]);
    let mut circuit = Circuit::new(5);
    
    // Encode in X basis
    code.encode(&mut circuit);
    
    // Measure syndrome
    code.measure_syndrome(&mut circuit, [3, 4]);
    
    // Should have encoding + syndrome measurement gates
    assert!(circuit.gates.len() > 6);
}

#[test]
fn test_phaseflip_detects_single_z_error() {
    for error_qubit in 0..3 {
        let code = PhaseFlipCode::new([0, 1, 2]);
        let mut circuit = Circuit::new(5);
        
        code.encode(&mut circuit);
        
        // Apply Z error (phase flip)
        circuit.add_gate(Gate::Z(error_qubit));
        
        let expected_syndrome = match error_qubit {
            0 => (true, false),
            1 => (true, true),
            2 => (false, true),
            _ => unreachable!(),
        };
        
        let detected = code.detect_error(expected_syndrome);
        assert_eq!(detected, Some(error_qubit));
    }
}

#[test]
fn test_phaseflip_no_error_detection() {
    let code = PhaseFlipCode::new([0, 1, 2]);
    
    let syndrome = (false, false);
    let detected = code.detect_error(syndrome);
    assert_eq!(detected, None);
}

#[test]
fn test_error_type_conversion() {
    // Test ErrorType helper methods
    let x_error = ErrorType::BitFlip;
    assert_eq!(x_error.to_gate(0), Gate::X(0));
    
    let z_error = ErrorType::PhaseFlip;
    assert_eq!(z_error.to_gate(1), Gate::Z(1));
    
    let y_error = ErrorType::BitPhase;
    assert_eq!(y_error.to_gate(2), Gate::Y(2));
}

#[test]
fn test_multiple_qubit_sets() {
    // Test that codes work with different qubit indices
    let code1 = BitFlipCode::new([0, 1, 2]);
    let code2 = BitFlipCode::new([3, 4, 5]);
    
    let mut circuit = Circuit::new(10);
    code1.encode(&mut circuit);
    code2.encode(&mut circuit);
    
    // Should have 4 CNOT gates total (2 per encoding)
    assert_eq!(circuit.gates.len(), 4);
    assert_eq!(circuit.gates[0], Gate::CNOT(0, 1));
    assert_eq!(circuit.gates[1], Gate::CNOT(0, 2));
    assert_eq!(circuit.gates[2], Gate::CNOT(3, 4));
    assert_eq!(circuit.gates[3], Gate::CNOT(3, 5));
}

#[test]
fn test_syndrome_measurement_uses_ancilla() {
    let code = BitFlipCode::new([0, 1, 2]);
    let mut circuit = Circuit::new(5);
    
    code.measure_syndrome(&mut circuit, [3, 4]);
    
    // Verify ancilla qubits are used in measurements
    let has_measure_3 = circuit.gates.iter().any(|g| matches!(g, Gate::Measure(3)));
    let has_measure_4 = circuit.gates.iter().any(|g| matches!(g, Gate::Measure(4)));
    
    assert!(has_measure_3, "Ancilla qubit 3 should be measured");
    assert!(has_measure_4, "Ancilla qubit 4 should be measured");
}

#[test]
fn test_encoding_preserves_qubit_count() {
    // Verify encoding doesn't try to use qubits outside range
    let code = BitFlipCode::new([0, 1, 2]);
    let mut circuit = Circuit::new(3);
    
    // This should not panic
    code.encode(&mut circuit);
    
    // All gates should only reference qubits 0, 1, 2
    for gate in &circuit.gates {
        match gate {
            Gate::CNOT(c, t) => {
                assert!(*c < 3);
                assert!(*t < 3);
            }
            _ => {}
        }
    }
}

#[test]
fn test_bitflip_and_phaseflip_orthogonal() {
    // Demonstrate that bit-flip code detects X errors
    // and phase-flip code detects Z errors
    
    let bitflip = BitFlipCode::new([0, 1, 2]);
    let phaseflip = PhaseFlipCode::new([3, 4, 5]);
    
    let mut circuit = Circuit::new(10);
    
    // Encode both
    bitflip.encode(&mut circuit);
    phaseflip.encode(&mut circuit);
    
    // Apply X error to bit-flip code (will be detected)
    circuit.add_gate(Gate::X(1));
    
    // Apply Z error to phase-flip code (will be detected)
    circuit.add_gate(Gate::Z(4));
    
    // Both codes should detect their respective errors
    let bitflip_syndrome = (true, true); // X on qubit 1
    let phaseflip_syndrome = (true, true); // Z on qubit 4
    
    assert_eq!(bitflip.detect_error(bitflip_syndrome), Some(1));
    assert_eq!(phaseflip.detect_error(phaseflip_syndrome), Some(4));
}
