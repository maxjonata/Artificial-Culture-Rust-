pub mod environment;
pub mod physics;        // Rapier2D physics configuration and utilities
use bevy::prelude::*;

/// Plugin for world management (environment, physics, etc.)
/// Separate from AI intelligence - handles the physical world
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add Environment management systems
            .add_plugins(environment::EnvironmentPlugin)
            // Add Physics configuration and systems
            .add_plugins(physics::PhysicsPlugin);
    }
}
