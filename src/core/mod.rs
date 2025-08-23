pub mod constants;
pub mod entities;
pub mod types;
pub mod spawning;
pub mod builders;

use crate::core::builders::ComponentSystemExt;
use bevy::prelude::*;
use builders::ComponentBuilderPlugin;

/// Core-specific type registration functionality.
///
/// This provides bulk type registration for core components, embedded directly
/// in the core module following the component-private functionality principle.
trait CoreTypeRegistration {
    fn register_core_types(&mut self) -> &mut Self;
}

impl CoreTypeRegistration for App {
    fn register_core_types(&mut self) -> &mut Self {
        self.register_type::<constants::GameConstants>()
    }
}

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
            .register_core_types()
            .init_resource::<constants::GameConstants>()
            .add_systems(Startup, spawning::spawn_npcs_system);
    }
}
