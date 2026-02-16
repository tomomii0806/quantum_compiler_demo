pub mod model;
pub mod compiler;

// Re-export for convenience
pub use model::{Gate, Circuit};
pub use compiler::compile;

