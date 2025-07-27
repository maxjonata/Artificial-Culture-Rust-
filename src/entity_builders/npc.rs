use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;
use crate::components::{Npc, Personality, KnowledgeBase, GameConstants, ColorConstants};
use crate::components::needs::{BasicNeeds, Desire, DesireThresholds};
use crate::components::pathfinding::{PathTarget, SteeringBehavior, ResourceMemory};

/// Builder function to spawn an NPC entity with all necessary Components
pub fn spawn_npc(
    commands: &mut Commands,
    game_constants: &GameConstants,
    color_constants: &ColorConstants,
    position: Vec2,
    npc_id: usize,
) {
    let mut rng = rand::rng();

    // Generate random initial velocity for autonomous movement
    let initial_velocity = Vec2::new(
        rng.random_range(-1.0..=1.0),
        rng.random_range(-1.0..=1.0)
    ).normalize_or_zero() * game_constants.npc_speed;

    // Create the base entity
    let entity = commands.spawn_empty().id();

    // Core NPC attributes - custom components for behavior and state
    commands.entity(entity).insert((
        Npc,
        Personality {
            openness: rng.random_range(0.0..=1.0), // Full range of openness
        },
        KnowledgeBase { knows_rumor: false },
    ));

    // Needs system - physiological and psychological drives
    commands.entity(entity).insert((
        BasicNeeds::default(),
        Desire::default(), // Starts with Wander
        DesireThresholds::default(),
    ));

    // Pathfinding system - navigation and movement AI
    commands.entity(entity).insert((
        PathTarget::default(),
        SteeringBehavior::default(),
        ResourceMemory::default(),
    ));

    // Visual components - rendering and identification
    commands.entity(entity).insert((
        Sprite {
            color: color_constants.green,
            custom_size: Some(Vec2::splat(game_constants.npc_radius * 2.0)),
            ..default()
        },
        Name::new(format!("NPC {}", npc_id + 1)),
        Transform::from_xyz(position.x, position.y, 0.0),
    ));

    // Physics components - collision and movement dynamics
    commands.entity(entity).insert((
        RigidBody::Dynamic,
        GravityScale(0.0), // Disable gravity for the top-down view
        Collider::ball(game_constants.npc_radius),
        Restitution::coefficient(0.7),
        Friction::coefficient(0.3),
        ActiveEvents::COLLISION_EVENTS,
    ));

    // Movement components - velocity and damping for autonomous behavior
    commands.entity(entity).insert((
        Velocity {
            linvel: initial_velocity,
            angvel: 0.0,
        },
        Damping {
            linear_damping: 0.1, // Reduced damping for better movement
            angular_damping: 0.8
        },
    ));
}

/// Builder function to spawn multiple NPCs for testing purposes
pub fn spawn_test_npcs(
    commands: &mut Commands,
    game_constants: &GameConstants,
    color_constants: &ColorConstants,
) {
    let mut rng = rand::rng();

    for i in 0..game_constants.num_npcs {
        let position = Vec2::new(
            rng.random_range(-400.0..=400.0),
            rng.random_range(-300.0..=300.0)
        );
        
        spawn_npc(commands, game_constants, color_constants, position, i);
    }
    
    println!("Simulação iniciada com {} NPCs.", game_constants.num_npcs);
}