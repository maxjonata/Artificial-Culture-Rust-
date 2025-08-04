use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

/// System to spawn NPCs at startup based on game constants
pub fn spawn_npcs_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
) {
    let mut rng = rand::rng();

    // Calculate spawn boundaries (leave some margin from edges)
    let (min_x, max_x, min_y, max_y) = if let Ok(window) = windows.single() {
        let margin = 100.0;
        (
            -window.width() / 2.0 + margin,
            window.width() / 2.0 - margin,
            -window.height() / 2.0 + margin,
            window.height() / 2.0 - margin,
        )
    } else {
        (-300.0, 300.0, -200.0, 200.0) // Default bounds
    };

    // Spawn NPCs with essential components first
    let num_npcs = 15; // Starting with a reasonable number for testing
    for i in 0..num_npcs {
        let position = Vec2::new(
            rng.random_range(min_x..=max_x),
            rng.random_range(min_y..=max_y),
        );

        // First spawn with core components (under 10 component limit)
        let entity_id = commands.spawn((
            // Core NPC identifier
            crate::core::entities::Npc { id: i },

            // Transform and rendering
            Sprite {
                image: asset_server.load("person.png"),
                ..default()
            },
            Transform::from_translation(position.extend(0.0)),

            // Basic physics for top-down movement
            RigidBody::Dynamic,
            Collider::ball(8.0), // 8 pixel radius for NPCs
            Velocity::default(),

            // Debug information
            Name::new(format!("NPC_{}", i)),
        )).id();

        // Then insert additional physics components to configure top-down behavior
        commands.entity(entity_id).insert((
            // Zero gravity for top-down view - configured per entity now
            GravityScale(0.0),
            // Lock Z rotation to prevent spinning
            LockedAxes::ROTATION_LOCKED,
            // Add damping to prevent excessive sliding
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
        ));

        // Insert AI perception components
        commands.entity(entity_id).insert((
            crate::ai::perception::vision::VisionCone::default(),
            crate::ai::perception::vision::VisuallyPerceived::default(),
            crate::ai::perception::vision::VisualApparentState::default(),
        ));

        // Insert AI cognition and behavior components
        commands.entity(entity_id).insert((
            crate::ai::physiology::needs::BasicNeeds::default(),
            crate::ai::cognition::decision::DesireDecision::default(),
            crate::ai::cognition::memory::CognitiveWorkingMemory::default(),
        ));

        // Insert navigation components
        commands.entity(entity_id).insert((
            crate::ai::navigation::pathfinding::PathTarget::default(),
            crate::ai::navigation::pathfinding::SteeringBehavior::default(),
            crate::ai::navigation::pathfinding::SpatialThingMemory::default(),
        ));

        println!("Spawned NPC_{} at position {:?} with proper top-down physics", i, position);
    }

    println!("Spawned {} NPCs.", num_npcs);
}
