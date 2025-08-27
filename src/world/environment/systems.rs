use crate::world::environment::factories;
use bevy::log::info;
use bevy::math::Vec2;
use bevy::prelude::Commands;

/// System to update colliders when PNG images are loaded (simplified).
/// Currently disabled due to complexity - use default colliders for now.
pub fn update_png_colliders_system() {
    // This system can be expanded later to provide pixel-perfect collision
    // from PNG images when needed. For now, we use simple bounding box colliders.
}

/// Create empty room for testing.
pub fn create_empty_room_system(mut commands: Commands) {
    factories::spawn_bounds(&mut commands, Vec2::new(400.0, 300.0), 5.0);
    info!("Created empty room (400x300)");
}

/// Configure physics for top-down movement.
pub fn configure_physics_system() {
    // Rapier physics is configured with pixels_per_meter in the plugin
    // Gravity can be disabled by setting RigidBody::Dynamic with damping
    // or by using RigidBody::KinematicVelocityBased for controlled movement

}