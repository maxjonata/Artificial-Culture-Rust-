pub mod default;
pub mod npc;
pub mod knowledge;
pub mod needs;
pub mod resources;
pub mod environment;

pub mod pathfinding;

// Re-export components from specific modules
pub use npc::*;
pub use knowledge::*;
pub use resources::*;
pub use pathfinding::*;