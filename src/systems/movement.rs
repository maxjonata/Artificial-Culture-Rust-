use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::components::npc::Npc;
use crate::components::resources::GameConstants;

/// Calculate the effective bounds for NPC movement
fn calculate_bounds(window: &Window, npc_radius: f32) -> Vec2 {
    Vec2::new(
        window.width() / 2.0 - npc_radius,
        window.height() / 2.0 - npc_radius
    )
}

/// Get the current direction from velocity or use default
fn get_direction(velocity: &Velocity) -> Vec2 {
    if velocity.linvel.length_squared() > 0.0 {
        velocity.linvel.normalize()
    } else {
        Vec2::new(1.0, 0.0) // Default direction if velocity is zero
    }
}

/// Reflect a direction vector across a normal vector
fn reflect_direction(direction: Vec2, normal: Vec2) -> Vec2 {
    direction - 2.0 * direction.dot(normal) * normal
}

/// Check boundaries and return the appropriate reflection normal if collision detected
fn check_boundaries(position: Vec2, direction: Vec2, bounds: Vec2) -> Option<Vec2> {
    // Check horizontal boundaries (left/right)
    if position.x < -bounds.x && direction.x < 0.0 {
        Some(Vec2::new(1.0, 0.0)) // Left wall normal
    } else if position.x > bounds.x && direction.x > 0.0 {
        Some(Vec2::new(-1.0, 0.0)) // Right wall normal
    }
    // Check vertical boundaries (top/bottom)
    else if position.y < -bounds.y && direction.y < 0.0 {
        Some(Vec2::new(0.0, 1.0)) // Bottom wall normal
    } else if position.y > bounds.y && direction.y > 0.0 {
        Some(Vec2::new(0.0, -1.0)) // Top wall normal
    } else {
        None
    }
}

/// Ensure a direction vector is normalized
fn normalize_direction(direction: Vec2, fallback: Vec2) -> Vec2 {
    if direction.length_squared() > 0.0 {
        direction.normalize()
    } else {
        fallback
    }
}

/// Update velocity based on direction and speed
fn update_velocity(velocity: &mut Velocity, direction: Vec2, speed: f32) {
    velocity.linvel = direction * speed;
}

/// System for handling NPC movement with boundary checking
pub fn movement_system(
    mut query: Query<(&Transform, &mut Velocity), With<Npc>>,
    windows: Query<&Window>,
    game_constants: Res<GameConstants>,
) {
    if let Some(window) = windows.iter().next() {
        let bounds = calculate_bounds(window, game_constants.npc_radius);
        
        for (transform, mut velocity) in query.iter_mut() {
            let position = transform.translation.truncate();
            let direction = get_direction(&velocity);
            let mut new_direction = direction;
            
            // Check all boundaries and apply reflection if needed
            if let Some(normal) = check_boundaries(position, direction, bounds) {
                new_direction = reflect_direction(direction, normal);
            }
            
            // Ensure the direction is normalized
            new_direction = normalize_direction(new_direction, direction);
            
            // Update velocity
            update_velocity(&mut velocity, new_direction, game_constants.npc_speed);
        }
    }
}