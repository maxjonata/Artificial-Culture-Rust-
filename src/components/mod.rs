#![allow(unused_imports)]

pub mod components_default;
pub mod components_environment;
pub mod components_knowledge;
pub mod components_needs;
pub mod components_npc;
pub mod components_pathfinding;
pub mod components_resources;

// Re-export commonly used types for easier access
pub use components_default::*;
pub use components_knowledge::*;
pub use components_npc::*;
pub use components_resources::*;
