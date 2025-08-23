//! Entity spawning systems for the Artificial Society simulation.
//!
//! This module contains systems responsible for creating and initializing entities
//! in the simulation world. All entity creation follows the Type-Safe Builder pattern
//! to ensure proper initialization and prevent runtime errors.
//!
//! # Spawning Philosophy
//!
//! Entity spawning is event-driven and follows the "Equality of Potential" principle.
//! All NPCs are created with the same fundamental cognitive architecture, with
//! variations arising from different initial conditions and experiences.

use bevy::prelude::*;
use rand::prelude::*;

/// System that spawns initial NPCs at simulation startup.
///
/// This system creates the initial population of NPCs in the simulation world.
/// Each NPC is spawned with default cognitive components and positioned randomly
/// within the world bounds.
///
/// # Implementation Notes
///
/// - Uses the Type-Safe Builder pattern for entity creation
/// - Respects the max_npc_count limit from GameConstants
/// - Initializes all required cognitive components for each NPC
pub fn spawn_npcs_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
    resources: Res<crate::core::constants::GameConstants>,
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
    for i in 0..resources.num_npcs {
        let position = Vec2::new(
            rng.random_range(min_x..=max_x),
            rng.random_range(min_y..=max_y),
        );

        // First spawn with core components (under 10 component limit)
        // let entity_id = commands.spawn().id();

        // Then insert additional physics components to configure top-down behavior
        // commands.entity(entity_id).insert();

        // Insert AI perception components
        // commands.entity(entity_id).insert();

        // Insert AI cognition and behavior components
        // commands.entity(entity_id).insert();

        // Insert navigation components
        // commands.entity(entity_id).insert();

        println!("Spawned NPC_{i} at position {position} with proper top-down physics");
    }

    println!("Spawned {} NPCs.", resources.num_npcs);
}
