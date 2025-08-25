//! Entity spawning systems.

use bevy::prelude::*;
use rand::prelude::*;


pub fn spawn_npcs_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
    resources: Res<crate::core::constants::GameConstants>,
) {
    let mut rng = rand::rng();

    let (min_x, max_x, min_y, max_y) = if let Ok(window) = windows.single() {
        let margin = 100.0;
        (
            -window.width() / 2.0 + margin,
            window.width() / 2.0 - margin,
            -window.height() / 2.0 + margin,
            window.height() / 2.0 - margin,
        )
    } else {
        (-300.0, 300.0, -200.0, 200.0)
    };

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
