use bevy::prelude::*;

/// Environment plugin that manages all environmental systems.
///
/// This plugin coordinates the spawning and management of environmental
/// objects, resources, and spatial structures that agents interact with
/// during the simulation.
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {}
}