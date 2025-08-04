use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

/// Component representing an NPC's pathfinding target and navigation state
/// System based on Goal-Oriented Action Planning (GOAP) theory
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PathTarget {
    /// Target position to navigate towards
    pub target_position: Vec2,
    /// Target entity (if navigating to a specific entity like a well/restaurant)
    pub target_entity: Option<Entity>,
    /// Distance threshold to consider target "reached"
    pub arrival_threshold: f32,
    /// Whether the NPC has a valid path target
    pub has_target: bool,
    /// Time when the target was set (for timeout purposes)
    pub target_set_time: f32,
    /// Maximum time to pursue a target before giving up
    pub max_pursuit_time: f32,
}

/// Component for steering behavior towards targets
/// System based on Craig Reynolds' Steering Behaviors for autonomous agents
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SteeringBehavior {
    /// Desired velocity towards target
    pub desired_velocity: Vec2,
    /// Steering force applied to reach target
    pub steering_force: Vec2,
    /// Maximum steering force that can be applied
    pub max_steering_force: f32,
    /// Weight for seek behavior (moving towards target)
    pub seek_weight: f32,
    /// Weight for wander behavior (random exploration)
    pub wander_weight: f32,
    /// Current wander angle for autonomous movement
    pub wander_angle: f32,
    /// How much the wander angle changes per frame
    pub wander_angle_change: f32,
}

/// Component tracking NPC's spatial knowledge of Thing locations
/// System based on Spatial Cognition and Mental Maps theory
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SpatialThingMemory {
    /// Known well locations
    pub known_wells: Vec<Vec2>,
    /// Known restaurant locations
    pub known_restaurants: Vec<Vec2>,
    /// Known hotel locations
    pub known_hotels: Vec<Vec2>,
    /// Known safe zone locations
    pub known_safe_zones: Vec<Vec2>,
    /// Discovery radius - how close NPC needs to be to "discover" an Thing
    pub discovery_radius: f32,
    /// Memory decay factor - how quickly forgotten locations become less reliable
    pub memory_decay_rate: f32,
}

impl Default for PathTarget {
    fn default() -> Self {
        Self {
            target_position: Vec2::ZERO,
            target_entity: None,
            arrival_threshold: 10.0,
            has_target: false,
            target_set_time: 0.0,
            max_pursuit_time: 30.0,
        }
    }
}

impl Default for SteeringBehavior {
    fn default() -> Self {
        Self {
            desired_velocity: Vec2::ZERO,
            steering_force: Vec2::ZERO,
            max_steering_force: 200.0,
            seek_weight: 1.0,
            wander_weight: 0.5,
            wander_angle: 0.0,
            wander_angle_change: 0.3,
        }
    }
}

impl Default for SpatialThingMemory {
    fn default() -> Self {
        Self {
            known_wells: Vec::new(),
            known_restaurants: Vec::new(),
            known_hotels: Vec::new(),
            known_safe_zones: Vec::new(),
            // NPCs can discover Things within 100 pixels
            discovery_radius: 100.0,
            // Memory slowly decays over time (realistic cognitive modeling)
            memory_decay_rate: 0.01,
        }
    }
}

/// Basic movement system that applies forces to make NPCs move around
/// This creates visible movement while the AI systems are developing
pub fn basic_movement_system(
    mut query: Query<(&mut Velocity, &Transform, &mut SteeringBehavior), With<crate::core::entities::Npc>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();
    let mut rng = rand::rng();

    for (mut velocity, _transform, mut steering) in query.iter_mut() {
        // Update wander angle for autonomous movement
        steering.wander_angle += rng.random_range(-steering.wander_angle_change..=steering.wander_angle_change);

        // Calculate wander direction
        let wander_direction = Vec2::from_angle(steering.wander_angle);

        // Increased base speed for more visible movement (was 50.0, now 120.0)
        let base_speed = 120.0;
        let desired_velocity = wander_direction * base_speed * steering.wander_weight;

        // Apply velocity directly for immediate response in top-down view
        velocity.linvel = desired_velocity;

        // Add slight variation to make movement more organic
        let speed_variation = rng.random_range(0.8..=1.2);
        velocity.linvel *= speed_variation;
    }
}

/// System to handle basic pathfinding towards targets
pub fn pathfinding_system(
    mut query: Query<(&mut Velocity, &Transform, &mut PathTarget, &mut SteeringBehavior), With<crate::core::entities::Npc>>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();

    for (mut velocity, transform, mut path_target, _steering) in query.iter_mut() {
        if path_target.has_target {
            // Check if target is reached
            let distance_to_target = transform.translation.truncate().distance(path_target.target_position);

            if distance_to_target <= path_target.arrival_threshold {
                // Target reached, clear it
                path_target.has_target = false;
                velocity.linvel = Vec2::ZERO; // Stop moving
                println!("NPC reached target at {:?}", path_target.target_position);
                continue;
            }

            // Check for timeout
            if current_time - path_target.target_set_time > path_target.max_pursuit_time {
                path_target.has_target = false;
                println!("NPC timed out pursuing target");
                continue;
            }

            // Calculate direct velocity towards target
            let direction_to_target = (path_target.target_position - transform.translation.truncate()).normalize();
            let seeking_speed = 180.0; // Increased from 200.0 - fast but not too fast when seeking targets

            // Apply velocity directly for immediate pathfinding response
            velocity.linvel = direction_to_target * seeking_speed;
        }
    }
}

/// System that translates AI desires into concrete pathfinding targets
/// This bridges the gap between cognition (what NPCs want) and navigation (where they go)
pub fn desire_to_pathfinding_system(
    mut npc_query: Query<(Entity, &crate::ai::cognition::decision::DesireDecision, &mut PathTarget, &Transform, &SpatialThingMemory)>,
    time: Res<Time>,
    windows: Query<&Window>,
) {
    let current_time = time.elapsed_secs();

    // Get world bounds for random target generation
    let (world_width, world_height) = if let Ok(window) = windows.single() {
        (window.width(), window.height())
    } else {
        (800.0, 600.0)
    };

    for (entity, desire, mut path_target, transform, spatial_memory) in npc_query.iter_mut() {
        // Skip if NPC already has an active target
        if path_target.has_target {
            continue;
        }

        // Convert desire into a concrete pathfinding target
        let target_position = match desire.current_goal {
            crate::ai::cognition::decision::GoalType::FindWater => {
                // Try to find a known well, otherwise search randomly
                if let Some(&well_pos) = spatial_memory.known_wells.first() {
                    well_pos
                } else {
                    generate_search_target(transform.translation.truncate(), world_width, world_height, "water")
                }
            }
            crate::ai::cognition::decision::GoalType::FindFood => {
                // Try to find a known restaurant, otherwise search randomly
                if let Some(&restaurant_pos) = spatial_memory.known_restaurants.first() {
                    restaurant_pos
                } else {
                    generate_search_target(transform.translation.truncate(), world_width, world_height, "food")
                }
            }
            crate::ai::cognition::decision::GoalType::FindRest => {
                // Try to find a known hotel, otherwise search randomly
                if let Some(&hotel_pos) = spatial_memory.known_hotels.first() {
                    hotel_pos
                } else {
                    generate_search_target(transform.translation.truncate(), world_width, world_height, "rest")
                }
            }
            crate::ai::cognition::decision::GoalType::FindSafety => {
                // Try to find a known safe zone, otherwise search randomly
                if let Some(&safe_pos) = spatial_memory.known_safe_zones.first() {
                    safe_pos
                } else {
                    generate_search_target(transform.translation.truncate(), world_width, world_height, "safety")
                }
            }
            crate::ai::cognition::decision::GoalType::Socialize => {
                // For now, just move to a social gathering area (center of world)
                Vec2::ZERO
            }
            crate::ai::cognition::decision::GoalType::Explore => {
                // Generate a random exploration target
                generate_search_target(transform.translation.truncate(), world_width, world_height, "explore")
            }
            crate::ai::cognition::decision::GoalType::Idle => {
                // No target needed for idle state
                continue;
            }
        };

        // Set the pathfinding target
        path_target.target_position = target_position;
        path_target.has_target = true;
        path_target.target_set_time = current_time;
        path_target.max_pursuit_time = 30.0; // 30 seconds to find target

        println!("NPC {:?} pursuing {:?} at position {:?}", entity, desire.current_goal, target_position);
    }
}

/// Generate a random search target for when NPCs don't know specific locations
fn generate_search_target(current_pos: Vec2, world_width: f32, world_height: f32, _search_type: &str) -> Vec2 {
    use rand::prelude::*;
    let mut rng = rand::rng();

    let margin = 100.0;
    let half_width = world_width / 2.0 - margin;
    let half_height = world_height / 2.0 - margin;

    // Generate a target not too close to current position
    let min_distance = 150.0;
    let mut attempts = 0;

    loop {
        let target = Vec2::new(
            rng.random_range(-half_width..=half_width),
            rng.random_range(-half_height..=half_height),
        );

        if current_pos.distance(target) >= min_distance || attempts >= 10 {
            return target;
        }
        attempts += 1;
    }
}
