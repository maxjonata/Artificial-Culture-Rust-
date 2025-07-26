use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;
use rand::rng;
use crate::components::{Npc, Personality, KnowledgeBase, GameConstants, ColorConstants};

/// Builder function to spawn an NPC entity with all necessary components
pub fn spawn_npc(
    commands: &mut Commands,
    game_constants: &GameConstants,
    color_constants: &ColorConstants,
    position: Vec2,
    npc_id: usize,
) {
    let mut rng = rng();
    
    commands.spawn((
        Npc,
        Personality {
            openness: rng.random_range(0.0..0.5),
        },
        KnowledgeBase { knows_rumor: false },
        Sprite {
            color: color_constants.green,
            custom_size: Some(Vec2::splat(game_constants.npc_radius * 2.0)),
            ..default()
        },
        Name::new(format!("NPC {}", npc_id + 1)),
        Transform::from_xyz(position.x, position.y, 0.0),
        RigidBody::Dynamic,
        GravityScale(0.0), // Disable gravity for the top-down view
        Collider::ball(game_constants.npc_radius),
        Restitution::coefficient(0.7),
        Friction::coefficient(0.2),
        // Use Rapier's Velocity component instead of ExternalForce
        Velocity {
            linvel: Vec2::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0)).normalize_or_zero() * game_constants.npc_speed * 0.01,
            angvel: 0.0,
        },
        Damping { linear_damping: 0.5, angular_damping: 0.5 }, // Add damping to prevent excessive movement
        ActiveEvents::COLLISION_EVENTS,
    ));
}

/// Builder function to spawn multiple NPCs for testing purposes
pub fn spawn_test_npcs(
    commands: &mut Commands,
    game_constants: &GameConstants,
    color_constants: &ColorConstants,
) {
    let mut rng = rng();
    
    for i in 0..game_constants.num_npcs {
        let position = Vec2::new(
            rng.random_range(-400.0..400.0),
            rng.random_range(-300.0..300.0)
        );
        
        spawn_npc(commands, game_constants, color_constants, position, i);
    }
    
    println!("Simulação iniciada com {} NPCs.", game_constants.num_npcs);
}