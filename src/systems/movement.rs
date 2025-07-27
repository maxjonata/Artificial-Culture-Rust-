use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::components::npc::Npc;
use crate::components::needs::Desire;
use crate::components::resources::GameConstants;

// ML-HOOK: Events for quantifiable movement behavior tracking and reward calculation
#[derive(Event)]
pub struct BoundaryCollisionEvent {
    pub entity: Entity,
    pub position: Vec2,
    pub old_direction: Vec2,
    pub new_direction: Vec2,
    pub collision_normal: Vec2, // ML-HOOK: Quantifiable collision data
}

#[derive(Event)]
pub struct MovementBehaviorEvent {
    pub entity: Entity,
    pub current_desire: Desire,
    pub velocity: Vec2,
    pub movement_efficiency: f32, // ML-HOOK: Performance metric for learning
}

/// Utility function implementing boundary physics based on elastic collision theory
/// System based on Elastic Collision Theory - perfect reflection for containment
fn calculate_bounds(window: &Window, npc_radius: f32) -> Vec2 {
    Vec2::new(
        window.width() / 2.0 - npc_radius,
        window.height() / 2.0 - npc_radius
    )
}

/// Utility function to extract normalized direction from current velocity
/// System based on Classical Mechanics - velocity vector decomposition
fn get_direction(velocity: &Velocity) -> Vec2 {
    if velocity.linvel.length_squared() > 0.0 {
        velocity.linvel.normalize()
    } else {
        Vec2::new(1.0, 0.0) // Default direction if velocity is zero
    }
}

/// Utility function implementing elastic reflection for boundary collisions
/// System based on Law of Reflection - angle of incidence equals angle of reflection
fn reflect_direction(direction: Vec2, normal: Vec2) -> Vec2 {
    direction - 2.0 * direction.dot(normal) * normal
}

/// Utility function for boundary detection with predictive collision system
/// System based on Predictive Collision Detection for smooth boundary handling
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

/// Utility function for vector normalization with fallback handling
/// System based on Linear Algebra - unit vector normalization
fn normalize_direction(direction: Vec2, fallback: Vec2) -> Vec2 {
    if direction.length_squared() > 0.0 {
        direction.normalize()
    } else {
        fallback
    }
}

/// Utility function implementing desire-driven movement behavior
/// System based on Goal-Oriented Behavior and Motivational Psychology
fn calculate_desire_influenced_velocity(
    current_velocity: Vec2,
    desire: &Desire,
    base_speed: f32,
    time_delta: f32
) -> Vec2 {
    // ML-HOOK: Different desires could influence movement patterns for learning
    let speed_multiplier = match desire {
        Desire::FindFood => 1.2,     // Urgency increases speed
        Desire::FindWater => 1.3,    // Thirst is more urgent
        Desire::FindSafety => 1.5,   // Safety seeking is fastest
        Desire::Rest => 0.6,         // Fatigue reduces speed
        Desire::Socialize => 1.0,    // Normal social movement
        Desire::Wander => 0.8,       // Relaxed wandering pace
    };

    // Apply desire-based speed modification with smoothing
    let target_speed = base_speed * speed_multiplier;
    let current_speed = current_velocity.length();

    // Smooth speed transition to avoid jerky movement
    let speed_change_rate = 2.0; // Units per second
    let new_speed = if current_speed < target_speed {
        (current_speed + speed_change_rate * time_delta).min(target_speed)
    } else {
        (current_speed - speed_change_rate * time_delta).max(target_speed)
    };

    // Maintain direction but adjust speed
    if current_velocity.length_squared() > 0.0 {
        current_velocity.normalize() * new_speed
    } else {
        Vec2::new(new_speed, 0.0)
    }
}

/// Utility function for calculating movement efficiency metrics
/// ML-HOOK: Provides quantifiable performance metrics for behavior optimization
fn calculate_movement_efficiency(velocity: Vec2, desired_speed: f32) -> f32 {
    let current_speed = velocity.length();
    if desired_speed > 0.0 {
        (current_speed / desired_speed).clamp(0.0, 1.0)
    } else {
        0.0
    }
}

/// System implementing desire-driven movement with boundary collision handling
/// System based on Goal-Oriented Behavior and Classical Mechanics
pub fn movement_system(
    mut query: Query<(Entity, &mut Transform, &mut Velocity, &Desire), With<Npc>>,
    game_constants: Res<GameConstants>,
    windows: Query<&Window>,
    time: Res<Time>,
    mut boundary_events: EventWriter<BoundaryCollisionEvent>,
    mut movement_events: EventWriter<MovementBehaviorEvent>,
) {
    let Ok(window) = windows.get_single() else {
        return; // Exit early if no window found
    };
    let bounds = calculate_bounds(window, game_constants.npc_radius);
    let time_delta = time.delta_secs();

    for (entity, mut transform, mut velocity, desire) in query.iter_mut() {
        let position = transform.translation.truncate();
        let current_direction = get_direction(&velocity);

        // ML-HOOK: Apply desire-influenced movement behavior
        let desire_velocity = calculate_desire_influenced_velocity(
            velocity.linvel,
            desire,
            game_constants.npc_speed,
            time_delta
        );

        // Check for boundary collisions and apply reflection if needed
        if let Some(collision_normal) = check_boundaries(position, current_direction, bounds) {
            let reflected_direction = reflect_direction(current_direction, collision_normal);
            let new_velocity = reflected_direction * desire_velocity.length();

            // ML-HOOK: Fire event for quantifiable boundary collision tracking
            boundary_events.send(BoundaryCollisionEvent {
                entity,
                position,
                old_direction: current_direction,
                new_direction: reflected_direction,
                collision_normal,
            });

            velocity.linvel = new_velocity;
        } else {
            // Apply desire-influenced velocity when not colliding
            velocity.linvel = desire_velocity;
        }

        // ML-HOOK: Calculate and track movement efficiency metrics
        let efficiency = calculate_movement_efficiency(velocity.linvel, game_constants.npc_speed);

        movement_events.send(MovementBehaviorEvent {
            entity,
            current_desire: *desire,
            velocity: velocity.linvel,
            movement_efficiency: efficiency,
        });

        // Ensure direction is normalized for consistent behavior
        if velocity.linvel.length_squared() > 0.0 {
            let normalized_velocity = normalize_direction(velocity.linvel, Vec2::new(1.0, 0.0));
            velocity.linvel = normalized_velocity * velocity.linvel.length();
        }
    }
}

/// System for analyzing movement patterns and behavioral metrics
/// ML-HOOK: Provides quantifiable movement analytics for learning optimization
pub fn analyze_movement_patterns(
    query: Query<(&Transform, &Velocity, &Desire), With<Npc>>,
    mut last_analysis_time: Local<f32>,
    time: Res<Time>,
) {
    // Analyze every 3 seconds to track movement patterns
    *last_analysis_time += time.delta_secs();
    if *last_analysis_time >= 3.0 {
        *last_analysis_time = 0.0;

        let mut desire_speeds: std::collections::HashMap<Desire, Vec<f32>> = std::collections::HashMap::new();
        let mut total_movement_energy = 0.0;

        for (_transform, velocity, desire) in query.iter() {
            let speed = velocity.linvel.length();
            desire_speeds.entry(*desire).or_insert_with(Vec::new).push(speed);
            total_movement_energy += speed * speed; // Kinetic energy approximation
        }

        // ML-HOOK: Calculate quantifiable movement statistics by desire type
        for (desire, speeds) in desire_speeds.iter() {
            if !speeds.is_empty() {
                let avg_speed = speeds.iter().sum::<f32>() / speeds.len() as f32;
                let max_speed = speeds.iter().fold(0.0f32, |a, &b| a.max(b));

                debug!(
                    "Movement Analysis - Desire: {:?}, Avg Speed: {:.1}, Max Speed: {:.1}, Count: {}",
                    desire, avg_speed, max_speed, speeds.len()
                );
            }
        }

        // ML-HOOK: Overall system energy for behavioral optimization
        debug!(
            "Movement Energy Analysis - Total Kinetic Energy: {:.1}, NPCs: {}",
            total_movement_energy,
            query.iter().count()
        );
    }
}
