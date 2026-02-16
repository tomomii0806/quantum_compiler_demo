use crate::model::{Gate, Circuit};

pub fn compile(circuit: &Circuit) -> Circuit {
    let mut optimized = Circuit::new(circuit.num_qubits);
    let mut prev_gates: Vec<Option<Gate>> = vec![None; circuit.num_qubits];  // Track last gate per qubit

    for gate in &circuit.gates {
        match gate {
            Gate::X(q) => {
                // Skip if XX = I
                if let Some(Gate::X(_)) = prev_gates[*q] {
                    optimized.gates.pop();
                    prev_gates[*q] = None;
                } else {
                    optimized.add_gate(gate.clone());
                    prev_gates[*q] = Some(gate.clone());
                }
            }
            Gate::Y(q) => {
                // Skip if YY = I
                if let Some(Gate::Y(_)) = prev_gates[*q] {
                    optimized.gates.pop();
                    prev_gates[*q] = None;
                } else {
                    optimized.add_gate(gate.clone());
                    prev_gates[*q] = Some(gate.clone());
                }
            }
            
            Gate::Z(q) => {
                // Skip if ZZ = I
                if let Some(Gate::Z(_)) = prev_gates[*q] {
                    optimized.gates.pop();
                    prev_gates[*q] = None;
                } else {
                    optimized.add_gate(gate.clone());
                    prev_gates[*q] = Some(gate.clone());
                }
            }
            Gate::H(q) => {
                // Skip if HH = I
                if let Some(Gate::H(_)) = prev_gates[*q] {
                    optimized.gates.pop();
                    prev_gates[*q] = None;
                } else {
                    optimized.add_gate(gate.clone());
                    prev_gates[*q] = Some(gate.clone());
                }
            }
            Gate::CNOT(c, t) => {
                // Skip if CNOT is repeated on the same control and target
                if let Some(Gate::CNOT(pc, pt)) = prev_gates[*c] {
                    if pc == *c && pt == *t {
                        optimized.gates.pop();
                        prev_gates[*c] = None;
                    } else {
                        optimized.add_gate(gate.clone());
                    }
                } else {
                    optimized.add_gate(gate.clone());
                    prev_gates[*c] = Some(gate.clone());
                }
            }
            Gate::S(q) => {
                // If SS = Z, add Z instead of S
                if let Some(Gate::S(_)) = prev_gates[*q] {
                    optimized.gates.pop();
                    optimized.add_gate(Gate::Z(*q));
                    prev_gates[*q] = Some(Gate::Z(*q));
                } else {
                    optimized.add_gate(gate.clone());
                    prev_gates[*q] = Some(gate.clone());
                }
            }
            Gate::T(q) => {
                // If TT = S, add S instead of T
                if let Some(Gate::T(_)) = prev_gates[*q] {
                    optimized.gates.pop();
                    optimized.add_gate(Gate::S(*q));
                    prev_gates[*q] = Some(Gate::S(*q));
                } else {
                    optimized.add_gate(gate.clone());
                    prev_gates[*q] = Some(gate.clone());
                }
            }
            Gate::SWAP(a, b) => {
                // SWAP is symmetric; therefore swapping A↔B is the same as swapping B↔A and SWAP-SWAP cancels out
                if let Some(Gate::SWAP(pa, pb)) = &prev_gates[*a] {
                    if (*pa == *a && *pb == *b) || (*pa == *b && *pb == *a) {
                        optimized.gates.pop();
                        prev_gates[*a] = None;
                        prev_gates[*b] = None;
                    } else {
                        optimized.add_gate(gate.clone());
                        prev_gates[*a] = Some(gate.clone());
                        prev_gates[*b] = Some(gate.clone());
                    }
                } else {
                    optimized.add_gate(gate.clone());
                    prev_gates[*a] = Some(gate.clone());
                    prev_gates[*b] = Some(gate.clone());
                }
            }
            Gate::Rx(q, angle) => {
                // Rx(θ) + Rx(φ) = Rx(θ + φ), cancel if sum is ~0 (mod 2π)
                if let Some(Gate::Rx(_, prev_angle)) = prev_gates[*q] 
                {
                    let new_angle = (prev_angle + angle) % (2.0 * std::f64::consts::PI);
                    // Consider angles close to 0 or 2π as canceling out
                    // 0.0000000001  (close to 0) or 6.2831853071 (close to 2π)
                    if new_angle.abs() < 1e-10 || (new_angle - 2.0 * std::f64::consts::PI).abs() < 1e-10 {
                        optimized.gates.pop();
                        prev_gates[*q] = None;
                    } else {
                        optimized.gates.pop();
                        optimized.add_gate(Gate::Rx(*q, new_angle));
                        prev_gates[*q] = Some(Gate::Rx(*q, new_angle));
                    }
                } else {
                    optimized.add_gate(gate.clone());
                    prev_gates[*q] = Some(gate.clone());
                }
            }
            Gate::Ry(q, angle) => {
                // Ry(θ) + Ry(φ) = Ry(θ + φ)
                if let Some(Gate::Ry(_, prev_angle)) = prev_gates[*q] {
                    let new_angle = (prev_angle + angle) % (2.0 * std::f64::consts::PI);
                    if new_angle.abs() < 1e-10 || (new_angle - 2.0 * std::f64::consts::PI).abs() < 1e-10 {
                        optimized.gates.pop();
                        prev_gates[*q] = None;
                    } else {
                        optimized.gates.pop();
                        optimized.add_gate(Gate::Ry(*q, new_angle));
                        prev_gates[*q] = Some(Gate::Ry(*q, new_angle));
                    }
                } else {
                    optimized.add_gate(gate.clone());
                    prev_gates[*q] = Some(gate.clone());
                }
            }
            Gate::Rz(q, angle) => {
                // Rz(θ) + Rz(φ) = Rz(θ + φ)
                if let Some(Gate::Rz(_, prev_angle)) = prev_gates[*q] {
                    let new_angle = (prev_angle + angle) % (2.0 * std::f64::consts::PI);
                    if new_angle.abs() < 1e-10 || (new_angle - 2.0 * std::f64::consts::PI).abs() < 1e-10 {
                        optimized.gates.pop();
                        prev_gates[*q] = None;
                    } else {
                        optimized.gates.pop();
                        optimized.add_gate(Gate::Rz(*q, new_angle));
                        prev_gates[*q] = Some(Gate::Rz(*q, new_angle));
                    }
                } else {
                    optimized.add_gate(gate.clone());
                    prev_gates[*q] = Some(gate.clone());
                }
            }
        }
    }
    optimized
}
