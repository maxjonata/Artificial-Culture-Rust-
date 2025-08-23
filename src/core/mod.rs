pub mod constants;
pub mod entities;
pub mod types;
pub mod spawning;
pub mod builders;

use crate::utils::helpers::overrides::AppRegisterTypesExt;
use bevy::prelude::*;
use builders::ComponentBuilderPlugin;

/// Skeleton core infrastructure plugin (reset state)
///
/// Provides foundational systems and components for the Artificial Society simulation.
/// This plugin registers core entity types, initializes game constants, and sets up
/// essential startup systems.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ComponentBuilderPlugin)
            .register_types::<(
                constants::GameConstants,
                entities::Npc,
                types::Normalized
            )>()
            .init_resource::<constants::GameConstants>()
            .add_systems(Startup, spawning::spawn_npcs_system);
    }
}
