//! World simulation infrastructure for the Artificial Society.
//! Physical world simulation, environment management, and spatial relationships.

pub mod environment;
use bevy::prelude::*;

/// Main world plugin that orchestrates all physical world systems.
pub struct WorldPlugin;

/// Plugin for world management (environment, physics, etc.)
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add Environment management systems
            .add_plugins(environment::EnvironmentPlugin);
    }
}
