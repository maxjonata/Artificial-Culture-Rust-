//! Environment management for the Artificial Society.
//! Provides 2D top-down environment with physics, boundaries, and features.

mod components;
mod bundles;
mod factories;
mod helpers;
mod systems;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use components::RegisterEnvironmentComponentsPlugin;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add Rapier physics - gravity will be disabled in configure_physics_system
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RegisterEnvironmentComponentsPlugin)
            // Systems
            .add_systems(Startup, (
                systems::configure_physics_system,
                systems::create_empty_room_system,
            ))
            .add_systems(Update, systems::update_png_colliders_system);

        // Alternative environments (comment/uncomment as needed):
        // .add_systems(Startup, create_obstacle_course_system);
        // .add_systems(Startup, create_labyrinth_system);
    }
}