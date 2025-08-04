pub mod entities;
pub mod constants;
pub mod spawning;
pub mod utils;       // Cross-domain utility functions

use crate::core::spawning::spawn_npcs_system;
use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Register components from this domain
            .register_type::<entities::Npc>()
            // Register resources (constants)
            .register_type::<constants::GameConstants>()
            .register_type::<constants::ColorConstants>()
            .register_type::<constants::RumorTimer>()
            // Add spawning systems
            .add_systems(Startup, spawn_npcs_system);
    }
}
