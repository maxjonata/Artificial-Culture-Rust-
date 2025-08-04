use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Physics configuration and constants for the simulation
/// This module handles Rapier2D setup and physics-related utilities
///
/// Physics constants used throughout the simulation
#[derive(Resource)]
pub struct PhysicsConstants {
    /// Default gravity for the 2D world
    pub gravity: Vec2,
    /// Default physics timestep
    pub timestep: f32,
    /// Default collision groups
    pub collision_groups: CollisionGroups,
}

impl Default for PhysicsConstants {
    fn default() -> Self {
        Self {
            // No gravity by default for top-down simulation
            gravity: Vec2::ZERO,
            // Standard 60fps physics timestep
            timestep: 1.0 / 60.0,
            // Default collision groups - can be customized per entity
            collision_groups: CollisionGroups::new(Group::GROUP_1, Group::ALL),
        }
    }
}

/// Plugin for physics setup and configuration
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Insert physics constants as a resource
            .insert_resource(PhysicsConstants::default())
            // Add Rapier physics plugins for top-down simulation
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            // Enable debug rendering of physics shapes for development
            .add_plugins(RapierDebugRenderPlugin::default())
            // Add world boundary system for top-down containment
            .add_systems(Startup, spawn_world_boundaries_system)
            .add_systems(Update, enforce_boundary_constraints_system);
    }
}

/// System to spawn invisible walls around the world boundaries
/// Ensures NPCs cannot escape the simulation area (top-down pattern)
pub fn spawn_world_boundaries_system(
    mut commands: Commands,
    windows: Query<&Window>,
) {
    let (world_width, world_height) = if let Ok(window) = windows.single() {
        (window.width(), window.height())
    } else {
        (800.0, 600.0) // Default world size
    };

    let wall_thickness = 50.0;
    let half_width = world_width / 2.0;
    let half_height = world_height / 2.0;

    // Create invisible boundary walls
    let boundaries = [
        // Top wall
        (Vec2::new(0.0, half_height + wall_thickness / 2.0), Vec2::new(world_width, wall_thickness)),
        // Bottom wall
        (Vec2::new(0.0, -half_height - wall_thickness / 2.0), Vec2::new(world_width, wall_thickness)),
        // Left wall
        (Vec2::new(-half_width - wall_thickness / 2.0, 0.0), Vec2::new(wall_thickness, world_height)),
        // Right wall
        (Vec2::new(half_width + wall_thickness / 2.0, 0.0), Vec2::new(wall_thickness, world_height)),
    ];

    for (position, size) in boundaries.iter() {
        commands.spawn((
            Transform::from_translation(position.extend(0.0)),
            RigidBody::Fixed, // Static walls that don't move
            Collider::cuboid(size.x / 2.0, size.y / 2.0),
            Name::new("WorldBoundary"),
        ));
    }

    println!("Spawned world boundaries for {}x{} world", world_width, world_height);
}

/// System to enforce boundary constraints for NPCs (backup safety system)
/// Based on traditional top-down game patterns where entities reflect off borders
pub fn enforce_boundary_constraints_system(
    mut query: Query<(&mut Transform, &mut Velocity), With<crate::core::entities::Npc>>,
    windows: Query<&Window>,
) {
    let (world_width, world_height) = if let Ok(window) = windows.single() {
        (window.width(), window.height())
    } else {
        (800.0, 600.0)
    };

    let margin = 20.0; // Keep NPCs slightly away from the edge
    let half_width = world_width / 2.0 - margin;
    let half_height = world_height / 2.0 - margin;

    for (mut transform, mut velocity) in query.iter_mut() {
        let pos = &mut transform.translation;

        // Reflect velocity and constrain position if hitting boundaries
        if pos.x > half_width {
            pos.x = half_width;
            velocity.linvel.x = -velocity.linvel.x.abs(); // Bounce left
        } else if pos.x < -half_width {
            pos.x = -half_width;
            velocity.linvel.x = velocity.linvel.x.abs(); // Bounce right
        }

        if pos.y > half_height {
            pos.y = half_height;
            velocity.linvel.y = -velocity.linvel.y.abs(); // Bounce down
        } else if pos.y < -half_height {
            pos.y = -half_height;
            velocity.linvel.y = velocity.linvel.y.abs(); // Bounce up
        }
    }
}

/// Utility functions for physics operations
pub mod utils {
    use super::*;

    /// Create a basic rigid body bundle for an NPC
    pub fn create_npc_physics_bundle(position: Vec3) -> (RigidBody, Collider, Transform) {
        (
            RigidBody::Dynamic,
            Collider::ball(16.0), // 16 pixel radius for NPC collision
            Transform::from_translation(position),
        )
    }

    /// Create a static collider for environment objects
    pub fn create_static_collider(size: Vec2) -> (RigidBody, Collider) {
        (
            RigidBody::Fixed,
            Collider::cuboid(size.x / 2.0, size.y / 2.0),
        )
    }

    /// Check if two entities are within interaction distance
    pub fn is_within_interaction_range(pos1: Vec3, pos2: Vec3, max_distance: f32) -> bool {
        pos1.distance(pos2) <= max_distance
    }
}
