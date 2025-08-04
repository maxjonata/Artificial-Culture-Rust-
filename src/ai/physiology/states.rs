use bevy::prelude::*;

/// Component tracking NPC's physiological refilling/interaction state with resources
/// Based on Behavioral State Theory - agents have distinct behavioral modes
#[derive(Component, Reflect, PartialEq, Debug, Default)]
#[reflect(Component)]
pub struct PhysiologicalRefillState {
    /// Whether the NPC is currently refilling/interacting with a resource
    pub is_refilling: bool,
    /// Time when refilling started (for duration tracking)
    pub refill_start_time: f32,
    /// Duration of the refill action in seconds
    pub refill_duration: f32,
    /// Entity of the resource being used (if any)
    pub resource_entity: Option<Entity>,
}
