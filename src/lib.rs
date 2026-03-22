pub mod model;
pub mod compiler;
pub mod qec;

// Re-export for convenience
pub use model::{Gate, Circuit};
pub use compiler::compile;

