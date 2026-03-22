# Quantum Compiler Demo

A quantum circuit compiler written in Rust for learning fault-tolerant quantum computing. This project demonstrates quantum gate optimization and quantum error correction (QEC) techniques.

## Features

### Circuit Optimization
- Define quantum circuits with various gates
- Optimize circuits by canceling redundant gates
- Combine consecutive rotation gates
- Commutation-based optimization passes

### Quantum Error Correction (Phase 1)
- **3-qubit repetition codes**: Bit-flip and phase-flip detection
- **Syndrome extraction**: Non-destructive error measurement using ancilla qubits
- **Fault-tolerant measurements**: Two methods for X-parity (intuitive vs fault-tolerant)
- **Logical qubit abstraction**: Track encoded qubits and encoding types

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
| CZ | Controlled-Z |
| SWAP | Swap two qubits |

### Measurement Gates
| Gate | Description |
|------|-------------|
| Measure | Z-basis measurement |
| MeasureX | X-basis measurement |
| MeasureY | Y-basis measurement |

## Optimizations

The compiler applies the following optimizations:

- **Gate cancellation**: XX, YY, ZZ, HH → Identity
- **CNOT cancellation**: Two consecutive CNOTs cancel out
- **CZ cancellation**: Two consecutive CZ gates cancel out
- **SWAP cancellation**: Two consecutive SWAPs cancel out
- **Phase gate fusion**: SS → Z, TT → S
- **Rotation merging**: Rx(θ) + Rx(φ) → Rx(θ+φ)

## Quantum Error Correction

### Repetition Codes

Implemented 3-qubit repetition codes for error detection:

**Bit-Flip Code**: Detects single X errors
```rust
use quantum_compiler_demo::qec::BitFlipCode;

let code = BitFlipCode::new([0, 1, 2]);
let mut circuit = Circuit::new(5); // 3 data + 2 ancilla

// Encode: |ψ⟩ → |ψψψ⟩
code.encode(&mut circuit);

// Measure syndrome to detect errors
code.measure_syndrome(&mut circuit, [3, 4]);

// Decode syndrome to locate error
let syndrome = (true, false); // From measurement
let error_location = code.detect_error(syndrome); // Returns Some(0)
```

**Phase-Flip Code**: Detects single Z errors (works in X-basis)

### Syndrome Extraction Framework

Generic framework for building stabilizer measurement circuits:

```rust
use quantum_compiler_demo::qec::SyndromeCircuitBuilder;

let mut circuit = Circuit::new(6);

// Build syndrome measurement with Z and X stabilizers
SyndromeCircuitBuilder::new()
    .add_z_check(vec![0, 1, 2], 4)  // Z-parity measurement
    .add_x_check(vec![1, 2, 3], 5)  // X-parity (fault-tolerant)
    .build(&mut circuit);
```

**Two X-parity methods**:
- **Fault-tolerant**: Ancilla as control (data stays in Z-basis, fewer errors)
- **Intuitive**: Data as control (easier to understand, less fault-tolerant)

## Usage

### Basic Circuit Optimization

```rust
use quantum_compiler_demo::{Circuit, Gate, compile};

fn main() {
    let mut circuit = Circuit::new(2);
    circuit.add_gate(Gate::H(0));
    circuit.add_gate(Gate::X(0));
    circuit.add_gate(Gate::X(0));  // Will be optimized out
    circuit.add_gate(Gate::CNOT(0, 1));

    let compiled = compile(&circuit);
    println!("Optimized: {:?}", compiled);
}
```

### Quantum Error Correction Example

```rust
use quantum_compiler_demo::{Circuit, Gate};
use quantum_compiler_demo::qec::BitFlipCode;

fn main() {
    // Create 3-qubit bit-flip code
    let code = BitFlipCode::new([0, 1, 2]);
    let mut circuit = Circuit::new(5); // 3 data + 2 ancilla
    
    // Encode logical qubit
    code.encode(&mut circuit);
    
    // Simulate an X error
    circuit.add_gate(Gate::X(1));
    
    // Measure syndrome
    code.measure_syndrome(&mut circuit, [3, 4]);
    
    // In real hardware, measure ancillas and decode
    let syndrome = (true, true); // Both ancillas measure 1
    let error = code.detect_error(syndrome);
    println!("Error detected on qubit: {:?}", error); // Some(1)
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
├── lib.rs              # Library exports
├── compiler.rs         # Optimization logic
├── model/
│   ├── gate.rs        # Gate definitions + error types
│   ├── circuit.rs     # Circuit struct
│   └── logical_qubit.rs  # Logical qubit abstraction
└── qec/
    ├── repetition.rs   # Bit-flip and phase-flip codes
    └── syndrome.rs     # Syndrome extraction framework
tests/
├── compiler_tests.rs   # Optimization tests
└── qec_tests.rs       # Error correction tests
```

## Roadmap

### Phase 1: Repetition Codes (DONE)
- [x] Measurement gates (Measure, MeasureX, MeasureY)
- [x] Error types (BitFlip, PhaseFlip)
- [x] 3-qubit bit-flip repetition code
- [x] 3-qubit phase-flip repetition code
- [x] Syndrome extraction framework
- [x] Fault-tolerant vs intuitive measurement methods

### Phase 2: Surface Codes (WIP)
- [ ] Distance-3 surface code (17 qubits)
- [ ] 2D lattice structure
- [ ] X and Z stabilizer measurements
- [ ] Syndrome decoding
- [ ] Logical Pauli operations

### Future Phases
- [ ] **Phase 3**: Clifford+T decomposition, T-count optimization
- [ ] **Phase 4**: Lattice surgery, magic state distillation
- [ ] **Phase 5**: Full fault-tolerant compilation pipeline

### Additional Features
- [ ] S†, T† gates
- [ ] Toffoli (CCX) gate
- [ ] QASM export
- [ ] Circuit visualization

## Learning Goals

This project is designed for learning fault-tolerant quantum computing concepts relevant to quantum research developer positions:
- Stabilizer codes and syndrome measurement
- Fault-tolerant circuit design principles
- Resource estimation for FTQC
- Industry-standard techniques (surface codes, magic states)

