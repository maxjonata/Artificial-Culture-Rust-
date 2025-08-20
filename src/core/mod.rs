pub mod constants;
pub mod entities;
mod spawning;

use bevy::prelude::*;

/// Skeleton core infrastructure plugin (reset state)
///
/// Provides foundational systems and components for the Artificial Society simulation.
/// This plugin registers core entity types, initializes game constants, and sets up
/// essential startup systems.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Auto-register all debug types (components and resources)
        crate::utils::macros::register_all_debug_types(app);

        app
            .init_resource::<constants::GameConstants>()
            .add_systems(Startup, spawning::spawn_npcs_system);
    }
}
