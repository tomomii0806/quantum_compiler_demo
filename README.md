# Quantum Compiler Demo

A simple quantum circuit compiler written in Rust for personal learning purposes. This project demonstrates basic quantum gate optimization techniques.

## Features

- Define quantum circuits with various gates
- Optimize circuits by canceling redundant gates
- Combine consecutive rotation gates

## Supported Gates

### Single-Qubit Gates
| Gate | Description |
|------|-------------|
| H | Hadamard |
| X, Y, Z | Pauli gates |
| S | Phase gate (√Z) |
| T | T gate (⁴√Z) |
| Rx(θ), Ry(θ), Rz(θ) | Rotation gates |

### Two-Qubit Gates
| Gate | Description |
|------|-------------|
| CNOT | Controlled-X |
| SWAP | Swap two qubits |

## Optimizations

The compiler applies the following optimizations:

- **Gate cancellation**: XX, YY, ZZ, HH → Identity
- **CNOT cancellation**: Two consecutive CNOTs cancel out
- **SWAP cancellation**: Two consecutive SWAPs cancel out
- **Phase gate fusion**: SS → Z, TT → S
- **Rotation merging**: Rx(θ) + Rx(φ) → Rx(θ+φ)

## Usage

```rust
use quantum_compiler_demo::{Circuit, Gate, compile};

fn main() {
    let mut circuit = Circuit::new(2);
    circuit.add_gate(Gate::H(0));
    circuit.add_gate(Gate::X(0));
    circuit.add_gate(Gate::X(0));  // Will be optimized out
    circuit.add_gate(Gate::CNOT(0, 1));

    let compiled = compile(&circuit);
    println!("{:?}", compiled);
}
```

## Running

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Project Structure

```
src/
├── lib.rs          # Library exports
├── compiler.rs     # Optimization logic
└── model/
    ├── gate.rs     # Gate definitions
    └── circuit.rs  # Circuit struct
tests/
└── compiler_tests.rs
```

## Roadmap

More gates and features will be added in the future:

- [ ] S†, T†
- [ ] CZ
- [ ] Toffoli (CCX) gate
- [ ] Gate decomposition
- [ ] QASM export
- [ ] Circuit visualization

