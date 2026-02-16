use quantum_compiler_demo::{Circuit, Gate, compile};

#[test]
fn test_xx_cancels() {
    let mut c = Circuit::new(1);
    c.add_gate(Gate::X(0));
    c.add_gate(Gate::X(0));
    let compiled = compile(&c);
    assert_eq!(compiled.gates.len(), 0);
}

#[test]
fn test_yy_cancels() {
    let mut c = Circuit::new(1);
    c.add_gate(Gate::Y(0));
    c.add_gate(Gate::Y(0));
    let compiled = compile(&c);
    assert_eq!(compiled.gates.len(), 0);
}

#[test]
fn test_zz_cancels() {
    let mut c = Circuit::new(1);
    c.add_gate(Gate::Z(0));
    c.add_gate(Gate::Z(0));
    let compiled = compile(&c);
    assert_eq!(compiled.gates.len(), 0);
}

#[test]
fn test_hh_cancels() {
    let mut c = Circuit::new(1);
    c.add_gate(Gate::H(0));
    c.add_gate(Gate::H(0));
    let compiled = compile(&c);
    assert_eq!(compiled.gates.len(), 0);
}

#[test]
fn test_ss_becomes_z() {
    let mut c = Circuit::new(1);
    c.add_gate(Gate::S(0));
    c.add_gate(Gate::S(0));
    let compiled = compile(&c);
    assert_eq!(compiled.gates, vec![Gate::Z(0)]);
}

#[test]
fn test_tt_becomes_s() {
    let mut c = Circuit::new(1);
    c.add_gate(Gate::T(0));
    c.add_gate(Gate::T(0));
    let compiled = compile(&c);
    assert_eq!(compiled.gates, vec![Gate::S(0)]);
}

#[test]
fn test_cnot_cancels() {
    let mut c = Circuit::new(2);
    c.add_gate(Gate::CNOT(0, 1));
    c.add_gate(Gate::CNOT(0, 1));
    let compiled = compile(&c);
    assert_eq!(compiled.gates.len(), 0);
}

#[test]
fn test_mixed_gates() {
    let mut c = Circuit::new(2);
    c.add_gate(Gate::H(0));
    c.add_gate(Gate::T(0));
    c.add_gate(Gate::T(0));  // Should become S
    c.add_gate(Gate::CNOT(0, 1));
    let compiled = compile(&c);
    assert_eq!(compiled.gates.len(), 3);  // H, S, CNOT
    assert_eq!(compiled.gates[1], Gate::S(0));
}

#[test]
fn test_swap_cancels() {
    let mut c = Circuit::new(2);
    c.add_gate(Gate::SWAP(0, 1));
    c.add_gate(Gate::SWAP(0, 1));
    let compiled = compile(&c);
    assert_eq!(compiled.gates.len(), 0);
}

#[test]
fn test_swap_reverse_cancels() {
    let mut c = Circuit::new(2);
    c.add_gate(Gate::SWAP(0, 1));
    c.add_gate(Gate::SWAP(1, 0));  // Same as SWAP(0,1)
    let compiled = compile(&c);
    assert_eq!(compiled.gates.len(), 0);
}

#[test]
fn test_rx_combines() {
    use std::f64::consts::PI;
    let mut c = Circuit::new(1);
    c.add_gate(Gate::Rx(0, PI / 4.0));
    c.add_gate(Gate::Rx(0, PI / 4.0));
    let compiled = compile(&c);
    assert_eq!(compiled.gates.len(), 1);
    // Should combine to Rx(PI/2)
    if let Gate::Rx(_, angle) = compiled.gates[0] {
        assert!((angle - PI / 2.0).abs() < 1e-10);
    } else {
        panic!("Expected Rx gate");
    }
}

#[test]
fn test_rx_cancels_at_2pi() {
    use std::f64::consts::PI;
    let mut c = Circuit::new(1);
    c.add_gate(Gate::Rx(0, PI));
    c.add_gate(Gate::Rx(0, PI));
    let compiled = compile(&c);
    assert_eq!(compiled.gates.len(), 0);  // 2*PI = identity
}

#[test]
fn test_ry_combines() {
    use std::f64::consts::PI;
    let mut c = Circuit::new(1);
    c.add_gate(Gate::Ry(0, PI / 3.0));
    c.add_gate(Gate::Ry(0, PI / 6.0));
    let compiled = compile(&c);
    assert_eq!(compiled.gates.len(), 1);
    if let Gate::Ry(_, angle) = compiled.gates[0] {
        assert!((angle - PI / 2.0).abs() < 1e-10);
    } else {
        panic!("Expected Ry gate");
    }
}

#[test]
fn test_rz_combines() {
    use std::f64::consts::PI;
    let mut c = Circuit::new(1);
    c.add_gate(Gate::Rz(0, PI / 2.0));
    c.add_gate(Gate::Rz(0, PI / 2.0));
    let compiled = compile(&c);
    assert_eq!(compiled.gates.len(), 1);
    if let Gate::Rz(_, angle) = compiled.gates[0] {
        assert!((angle - PI).abs() < 1e-10);
    } else {
        panic!("Expected Rz gate");
    }
}
