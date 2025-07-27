use bevy::prelude::*;
use rand::prelude::*;

use crate::components::npc::Npc;
use crate::components::needs::{BasicNeeds, Desire};
use crate::components::environment::{Well, Restaurant, Hotel, SafeZone, ResourceType};
use crate::components::pathfinding::{PathTarget, SteeringBehavior, ResourceMemory};
use crate::components::resources::GameConstants;

// ML-HOOK: Events for quantifiable pathfinding behavior tracking and reward calculation
#[derive(Event)]
pub struct PathTargetSetEvent {
    pub npc_entity: Entity,
    pub target_position: Vec2,
    pub target_entity: Option<Entity>,
    pub target_type: ResourceType,
    pub distance_to_target: f32, // ML-HOOK: Quantifiable pathfinding efficiency
}

#[derive(Event)]
pub struct PathTargetReachedEvent {
    pub npc_entity: Entity,
    pub target_position: Vec2,
    pub target_entity: Option<Entity>,
    pub time_to_reach: f32, // ML-HOOK: Performance metric for navigation efficiency
}

#[derive(Event)]
pub struct ResourceDiscoveredEvent {
    pub npc_entity: Entity,
    pub resource_position: Vec2,
    pub resource_entity: Entity,
    pub resource_type: ResourceType,
    pub discovery_distance: f32, // ML-HOOK: Spatial cognition metrics
}

/// Utility function implementing Craig Reynolds' Seek steering behavior
/// System based on Steering Behaviors for Autonomous Characters
fn calculate_seek_force(
    current_position: Vec2,
    current_velocity: Vec2,
    target_position: Vec2,
    max_speed: f32,
    max_force: f32,
) -> Vec2 {
    // Calculate desired velocity towards target
    let desired_velocity = (target_position - current_position).normalize_or_zero() * max_speed;

    // Calculate steering force (desired - current)
    let steering_force = desired_velocity - current_velocity;

    // Limit steering force to maximum
    if steering_force.length() > max_force {
        steering_force.normalize() * max_force
    } else {
        steering_force
    }
}

/// Utility function implementing Wander steering behavior for autonomous movement
/// System based on Craig Reynolds' Wander steering behavior
fn calculate_wander_force(
    current_velocity: Vec2,
    wander_angle: &mut f32,
    wander_angle_change: f32,
    max_speed: f32,
    max_force: f32,
) -> Vec2 {
    // Update wander angle with random change
    let mut rng = rand::rng();
    *wander_angle += rng.random_range(-wander_angle_change..=wander_angle_change);

    // Calculate wander direction
    let wander_direction = Vec2::new(wander_angle.cos(), wander_angle.sin());
    let desired_velocity = wander_direction * max_speed;

    // Calculate steering force
    let steering_force = desired_velocity - current_velocity;

    // Limit steering force
    if steering_force.length() > max_force {
        steering_force.normalize() * max_force
    } else {
        steering_force
    }
}

/// Utility function to find nearest resource of a specific type
/// System based on Spatial Cognition and Mental Maps theory
fn find_nearest_resource_of_type(
    memory: &ResourceMemory,
    current_position: Vec2,
    resource_type: ResourceType,
) -> Option<Vec2> {
    let resource_list = match resource_type {
        ResourceType::Water => &memory.known_wells,
        ResourceType::Food => &memory.known_restaurants,
        ResourceType::Rest => &memory.known_hotels,
        ResourceType::Safety => &memory.known_safe_zones,
    };

    resource_list
        .iter()
        .min_by(|a, b| {
            let dist_a = current_position.distance(**a);
            let dist_b = current_position.distance(**b);
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        })
        .copied()
}

/// System for discovering and memorizing environmental resources
/// System based on Spatial Cognition and Memory Formation theory
pub fn resource_discovery_system(
    mut npc_query: Query<(Entity, &Transform, &mut ResourceMemory), With<Npc>>,
    well_query: Query<(Entity, &Transform), (With<Well>, Without<Npc>)>,
    restaurant_query: Query<(Entity, &Transform), (With<Restaurant>, Without<Npc>)>,
    hotel_query: Query<(Entity, &Transform), (With<Hotel>, Without<Npc>)>,
    safe_zone_query: Query<(Entity, &Transform), (With<SafeZone>, Without<Npc>)>,
    mut discovery_events: EventWriter<ResourceDiscoveredEvent>,
) {
    for (npc_entity, npc_transform, mut memory) in npc_query.iter_mut() {
        let npc_pos = npc_transform.translation.truncate();

        // Discover wells
        for (resource_entity, resource_transform) in well_query.iter() {
            let resource_pos = resource_transform.translation.truncate();
            let distance = npc_pos.distance(resource_pos);

            if distance <= memory.discovery_radius && !memory.known_wells.contains(&resource_pos) {
                memory.known_wells.push(resource_pos);

                discovery_events.send(ResourceDiscoveredEvent {
                    npc_entity,
                    resource_position: resource_pos,
                    resource_entity,
                    resource_type: ResourceType::Water,
                    discovery_distance: distance,
                });

                info!("NPC discovered a well at distance {:.1}", distance);
            }
        }

        // Discover restaurants
        for (resource_entity, resource_transform) in restaurant_query.iter() {
            let resource_pos = resource_transform.translation.truncate();
            let distance = npc_pos.distance(resource_pos);

            if distance <= memory.discovery_radius && !memory.known_restaurants.contains(&resource_pos) {
                memory.known_restaurants.push(resource_pos);

                discovery_events.send(ResourceDiscoveredEvent {
                    npc_entity,
                    resource_position: resource_pos,
                    resource_entity,
                    resource_type: ResourceType::Food,
                    discovery_distance: distance,
                });

                info!("NPC discovered a restaurant at distance {:.1}", distance);
            }
        }

        // Discover hotels
        for (resource_entity, resource_transform) in hotel_query.iter() {
            let resource_pos = resource_transform.translation.truncate();
            let distance = npc_pos.distance(resource_pos);

            if distance <= memory.discovery_radius && !memory.known_hotels.contains(&resource_pos) {
                memory.known_hotels.push(resource_pos);

                discovery_events.send(ResourceDiscoveredEvent {
                    npc_entity,
                    resource_position: resource_pos,
                    resource_entity,
                    resource_type: ResourceType::Rest,
                    discovery_distance: distance,
                });

                info!("NPC discovered a hotel at distance {:.1}", distance);
            }
        }

        // Discover safe zones
        for (resource_entity, resource_transform) in safe_zone_query.iter() {
            let resource_pos = resource_transform.translation.truncate();
            let distance = npc_pos.distance(resource_pos);

            if distance <= memory.discovery_radius && !memory.known_safe_zones.contains(&resource_pos) {
                memory.known_safe_zones.push(resource_pos);

                discovery_events.send(ResourceDiscoveredEvent {
                    npc_entity,
                    resource_position: resource_pos,
                    resource_entity,
                    resource_type: ResourceType::Safety,
                    discovery_distance: distance,
                });

                info!("NPC discovered a safe zone at distance {:.1}", distance);
            }
        }
    }
}

/// System for setting pathfinding targets based on current desires
/// System based on Goal-Oriented Action Planning (GOAP) theory
pub fn target_selection_system(
    mut npc_query: Query<(Entity, &Transform, &Desire, &ResourceMemory, &mut PathTarget), With<Npc>>,
    time: Res<Time>,
    mut target_events: EventWriter<PathTargetSetEvent>,
) {
    for (npc_entity, transform, desire, memory, mut path_target) in npc_query.iter_mut() {
        let current_pos = transform.translation.truncate();

        // Only set new target if we don't have one or if desire changed
        if path_target.has_target {
            // Check if target should timeout
            if time.elapsed_secs() - path_target.target_set_time > path_target.max_pursuit_time {
                path_target.has_target = false;
                info!("NPC pathfinding target timed out");
            } else {
                continue; // Keep current target
            }
        }

        // ML-HOOK: This target selection could be replaced by an RL agent's action selection
        let target_resource_type = match desire {
            Desire::FindWater => ResourceType::Water,
            Desire::FindFood => ResourceType::Food,
            Desire::Rest => ResourceType::Rest,
            Desire::FindSafety => ResourceType::Safety,
            Desire::Wander | Desire::Socialize => continue, // No specific target needed
        };

        // Find nearest known resource of the desired type
        if let Some(target_pos) = find_nearest_resource_of_type(memory, current_pos, target_resource_type) {
            let distance = current_pos.distance(target_pos);

            path_target.target_position = target_pos;
            path_target.target_entity = None; // Could be enhanced to track specific entities
            path_target.has_target = true;
            path_target.target_set_time = time.elapsed_secs();

            // ML-HOOK: Fire event for quantifiable target selection tracking
            target_events.send(PathTargetSetEvent {
                npc_entity,
                target_position: target_pos,
                target_entity: None,
                target_type: target_resource_type,
                distance_to_target: distance,
            });

            info!("NPC set pathfinding target: {:?} at distance {:.1}", target_resource_type, distance);
        }
    }
}

/// System implementing steering behaviors for pathfinding navigation
/// System based on Craig Reynolds' Steering Behaviors for Autonomous Characters
pub fn steering_navigation_system(
    mut npc_query: Query<(Entity, &Transform, &mut bevy_rapier2d::prelude::Velocity, &PathTarget, &mut SteeringBehavior), With<Npc>>,
    game_constants: Res<GameConstants>,
    time: Res<Time>,
    mut reached_events: EventWriter<PathTargetReachedEvent>,
) {
    for (npc_entity, transform, mut velocity, path_target, mut steering) in npc_query.iter_mut() {
        let current_pos = transform.translation.truncate();
        let current_vel = velocity.linvel;

        // Armazenar valores antes de usar a referência mutável
        let max_steering_force = steering.max_steering_force;
        let seek_weight = steering.seek_weight;
        let wander_weight = steering.wander_weight;
        let wander_angle_change = steering.wander_angle_change;

        // Reset steering forces
        steering.desired_velocity = Vec2::ZERO;
        steering.steering_force = Vec2::ZERO;

        if path_target.has_target {
            // Check if target is reached
            let distance_to_target = current_pos.distance(path_target.target_position);

            if distance_to_target <= path_target.arrival_threshold {
                // Target reached!
                reached_events.send(PathTargetReachedEvent {
                    npc_entity,
                    target_position: path_target.target_position,
                    target_entity: path_target.target_entity,
                    time_to_reach: time.elapsed_secs() - path_target.target_set_time,
                });

                info!("NPC reached pathfinding target at distance {:.1}", distance_to_target);
                continue;
            }

            // Calculate seek force towards target
            let seek_force = calculate_seek_force(
                current_pos,
                current_vel,
                path_target.target_position,
                game_constants.npc_speed,
                max_steering_force,
            );

            steering.steering_force += seek_force * seek_weight;
        } else {
            // No target - use wander behavior for autonomous movement
            let wander_force = calculate_wander_force(
                current_vel,
                &mut steering.wander_angle,
                wander_angle_change,
                game_constants.npc_speed,
                max_steering_force,
            );

            steering.steering_force += wander_force * wander_weight;
        }

        // Apply steering force to velocity
        velocity.linvel += steering.steering_force * time.delta_secs();

        // Limit velocity to max speed
        if velocity.linvel.length() > game_constants.npc_speed {
            velocity.linvel = velocity.linvel.normalize() * game_constants.npc_speed;
        }

        // Store desired velocity for analysis
        steering.desired_velocity = velocity.linvel;
    }
}

/// System for clearing reached targets and resetting pathfinding state
/// System based on Goal Achievement and State Management theory
pub fn target_cleanup_system(
    mut npc_query: Query<&mut PathTarget, With<Npc>>,
    mut reached_events: EventReader<PathTargetReachedEvent>,
) {
    for event in reached_events.read() {
        // Find the NPC that reached its target and clear it
        for mut path_target in npc_query.iter_mut() {
            if path_target.has_target &&
               path_target.target_position.distance(event.target_position) < 1.0 {
                path_target.has_target = false;
                path_target.target_entity = None;
                break;
            }
        }
    }
}

/// System for analyzing pathfinding performance and behavior
/// ML-HOOK: Provides quantifiable pathfinding metrics for learning optimization
pub fn analyze_pathfinding_behavior(
    query: Query<(&Transform, &PathTarget, &SteeringBehavior), With<Npc>>,
    mut last_analysis_time: Local<f32>,
    time: Res<Time>,
) {
    *last_analysis_time += time.delta_secs();
    if *last_analysis_time >= 4.0 {
        *last_analysis_time = 0.0;

        let mut npcs_with_targets = 0;
        let mut total_distance_to_targets = 0.0;
        let mut total_steering_efficiency = 0.0;

        for (transform, path_target, steering) in query.iter() {
            if path_target.has_target {
                npcs_with_targets += 1;

                let distance = transform.translation.truncate().distance(path_target.target_position);
                total_distance_to_targets += distance;

                // Calculate steering efficiency (how aligned current velocity is with desired)
                let efficiency = if steering.desired_velocity.length() > 0.0 {
                    steering.desired_velocity.normalize().dot(
                        if steering.steering_force.length() > 0.0 {
                            steering.steering_force.normalize()
                        } else {
                            Vec2::ZERO
                        }
                    ).max(0.0)
                } else {
                    0.0
                };
                total_steering_efficiency += efficiency;
            }
        }

        if npcs_with_targets > 0 {
            let avg_distance = total_distance_to_targets / npcs_with_targets as f32;
            let avg_efficiency = total_steering_efficiency / npcs_with_targets as f32;

            debug!(
                "Pathfinding Analysis - NPCs with targets: {}, Avg distance to target: {:.1}, Avg steering efficiency: {:.2}",
                npcs_with_targets, avg_distance, avg_efficiency
            );
        }
    }
}
