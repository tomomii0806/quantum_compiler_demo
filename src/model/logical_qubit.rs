/// Represents a logical qubit encoded in multiple physical qubits
#[derive(Debug, Clone)]
pub struct LogicalQubit {
    /// Physical qubit indices that encode this logical qubit
    pub physical_qubits: Vec<usize>,
    /// Type of encoding used
    pub encoding: EncodingType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EncodingType {
    /// No encoding - bare physical qubit
    Physical,
    /// 3-qubit bit-flip code
    BitFlipRepetition,
    /// 3-qubit phase-flip code
    PhaseFlipRepetition,
    /// Surface code with specified distance
    SurfaceCode { distance: usize },
}

impl LogicalQubit {
    /// Create a new logical qubit from physical qubits
    pub fn new(physical_qubits: Vec<usize>, encoding: EncodingType) -> Self {
        Self {
            physical_qubits,
            encoding,
        }
    }
    
    /// Create a bare physical qubit (no encoding)
    pub fn physical(qubit_index: usize) -> Self {
        Self {
            physical_qubits: vec![qubit_index],
            encoding: EncodingType::Physical,
        }
    }
    
    /// Number of physical qubits used
    pub fn num_physical_qubits(&self) -> usize {
        self.physical_qubits.len()
    }
    
    /// Check if this is an encoded logical qubit
    pub fn is_encoded(&self) -> bool {
        self.encoding != EncodingType::Physical
    }
    
    /// Get data qubits (for codes with ancilla qubits, this returns only data qubits)
    pub fn data_qubits(&self) -> Vec<usize> {
        match self.encoding {
            EncodingType::Physical => self.physical_qubits.clone(),
            EncodingType::BitFlipRepetition | EncodingType::PhaseFlipRepetition => {
                // For simple repetition codes, all qubits are data qubits
                self.physical_qubits.clone()
            }
            EncodingType::SurfaceCode { distance: _ } => {
                // For surface code, we'll define data vs syndrome qubits later
                // For now, return all
                self.physical_qubits.clone()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physical_qubit() {
        let q = LogicalQubit::physical(0);
        assert_eq!(q.num_physical_qubits(), 1);
        assert!(!q.is_encoded());
        assert_eq!(q.encoding, EncodingType::Physical);
    }

    #[test]
    fn test_encoded_qubit() {
        let q = LogicalQubit::new(
            vec![0, 1, 2],
            EncodingType::BitFlipRepetition,
        );
        assert_eq!(q.num_physical_qubits(), 3);
        assert!(q.is_encoded());
        assert_eq!(q.data_qubits(), vec![0, 1, 2]);
    }
}
