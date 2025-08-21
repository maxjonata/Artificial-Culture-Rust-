pub mod constants;
pub mod entities;
pub mod types;
mod spawning;

use crate::utils::helpers::AppRegisterTypesExt;
use bevy::prelude::*;

/// Skeleton core infrastructure plugin (reset state)
///
/// Provides foundational systems and components for the Artificial Society simulation.
/// This plugin registers core entity types, initializes game constants, and sets up
/// essential startup systems.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_types::<(
                constants::GameConstants,
                entities::Npc,
                types::Normalized
            )>()
            .init_resource::<constants::GameConstants>()
            .add_systems(Startup, spawning::spawn_npcs_system);
    }
}
