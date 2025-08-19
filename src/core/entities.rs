//! Core entity definitions for the Artificial Society simulation.
//!
//! This module contains the fundamental entity types that populate the simulation world.
//! All entities follow the Entity-Component-System (ECS) architecture principles,
//! where entities are lightweight identifiers and components store the actual data.
//!
//! # Key Entity Types
//!
//! - `Npc`: Non-player characters with full cognitive capabilities
//! - `Player`: Human-controlled entities with limited direct control
//! - `Resource`: Environmental items that can be collected or consumed
//!
//! Following the "Equality of Potential" principle, all NPCs operate on the same
//! fundamental cognitive architecture, with differences arising from experience
//! rather than design limitations.

use bevy::prelude::*;

/// Represents a Non-Player Character in the simulation.
///
/// NPCs are the primary agents in the Artificial Society simulation. Each NPC
/// possesses a complete cognitive architecture including needs, memories, beliefs,
/// and decision-making capabilities. They serve as the foundation for emergent
/// social dynamics.
///
/// # Cognitive Architecture
///
/// NPCs implement the "Brain as a Society of Systems" model, where behavior
/// emerges from the interaction of multiple internal sub-systems rather than
/// a single decision-maker.
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Npc {
    /// Unique identifier for this NPC within the simulation
    pub id: u32,
    /// Display name for debugging and visualization
    pub name: String,
}
