use quantum_compiler_demo::{Circuit, Gate, compile};

fn main() {
    let mut circuit = Circuit::new(2);
    circuit.add_gate(Gate::H(0));
    circuit.add_gate(Gate::T(0));
    circuit.add_gate(Gate::T(0));  // Should be optimized out
    circuit.add_gate(Gate::CNOT(0, 1));
    circuit.add_gate(Gate::CNOT(0, 1));

    println!("Original: {:?}", circuit);

    let compiled = compile(&circuit);
    println!("Compiled: {:?}", compiled);
}
