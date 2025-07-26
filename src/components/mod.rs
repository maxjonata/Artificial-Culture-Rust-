pub mod default;
pub mod npc;
pub mod knowledge;
pub mod needs;
pub mod resources;

// Re-export components from specific modules
pub use npc::*;
pub use knowledge::*;
pub use resources::*;