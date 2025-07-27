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
}

impl Default for GameConstants {
    fn default() -> Self {
        Self {
            num_npcs: 20,
            npc_radius: 15.0,
            npc_speed: 200.0,
            social_distance: 100.0,
            hunger_decay: 0.1,
            thirst_decay: 0.1,
            fatigue_regen: 0.1,
            safety_decay: 0.1,
            social_decay: 0.1,
        }
    }
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

impl Default for ColorConstants {
    fn default() -> Self {
        Self {
            red: Color::srgb(1.0, 0.2, 0.2),
            green: Color::srgb(0.2, 1.0, 0.2),
        }
    }
}