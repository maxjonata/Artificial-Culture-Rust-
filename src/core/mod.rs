pub mod constants;
mod entities;
mod spawning;

use bevy::prelude::*;

/// Skeleton core infrastructure plugin (reset state)
///
/// Provides foundational systems and components for the Artificial Society simulation.
/// This plugin registers core entity types, initializes game constants, and sets up
/// essential startup systems.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, _app: &mut App) {
        _app
            .register_type::<entities::Npc>()
            .init_resource::<constants::GameConstants>()
            .add_systems(Startup, spawning::spawn_npcs_system);
    }
}
