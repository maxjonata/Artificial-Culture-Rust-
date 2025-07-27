use bevy::prelude::*;

/// Resource for timing rumor injection
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct RumorTimer(pub Timer);

/// Resource for game simulation constants
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct GameConstants {
    /// Number of NPCs in the simulation
    pub num_npcs: usize,
    /// Radius of each NPC
    pub npc_radius: f32,
    /// Movement speed of NPCs
    pub npc_speed: f32,
    /// Social interaction distance
    pub social_distance: f32,
    /// Decay rate for hunger
    pub hunger_decay: f32,
    /// Decay rate for thirst
    pub thirst_decay: f32,
    /// Regeneration rate for fatigue
    pub fatigue_regen: f32,
    /// Decay rate for safety
    pub safety_decay: f32,
    /// Decay rate to need for social interaction
    pub social_decay: f32,
    /// Number of wells (water sources) to spawn
    pub num_wells: usize,
    /// Number of restaurants (food sources) to spawn
    pub num_restaurants: usize,
    /// Number of hotels (rest sources) to spawn
    pub num_hotels: usize,
    /// Number of safe zones to spawn
    pub num_safe_zones: usize,
}

/// Resource for color constants
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ColorConstants {
    /// Red color for NPCs that know the rumor
    pub red: Color,
    /// Green color for NPCs that don't know the rumor
    pub green: Color,
}
