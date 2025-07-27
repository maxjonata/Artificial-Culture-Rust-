use bevy::prelude::*;

/// Marker component for NPCs
#[derive(Component, Reflect, PartialEq, Debug)]
#[reflect(Component)]
pub struct Npc;

/// Component representing NPC personality traits
#[derive(Component, Reflect, PartialEq, Debug)]
#[reflect(Component)]
pub struct Personality {
    /// Openness to new ideas and experiences (affects rumor spread)
    pub openness: f32,
}
