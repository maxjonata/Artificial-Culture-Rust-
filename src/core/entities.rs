use bevy::prelude::*;

/// Component identifying an NPC agent in the simulation
/// Contains essential identification and tracking information
#[derive(Component, Reflect, PartialEq, Debug)]
#[reflect(Component)]
pub struct Npc {
    /// Unique identifier for this NPC
    pub id: u32,
}
