use crate::core::constants::GameConstants;
use crate::world::environment::thing::{Hotel, Restaurant, SafeZone, Thing, ThingType, Well};
use bevy::prelude::*;
use rand::prelude::*;

/// System to spawn environmental Things at startup
pub fn spawn_environment_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _game_constants: Res<GameConstants>,
    windows: Query<&Window>,
) {
    let mut rng = rand::rng();

    // Calculate spawn boundaries (leave some margin from edges)
    let (min_x, max_x, min_y, max_y) = if let Ok(window) = windows.single() {
        let margin = 50.0;
        (
            -window.width() / 2.0 + margin,
            window.width() / 2.0 - margin,
            -window.height() / 2.0 + margin,
            window.height() / 2.0 - margin,
        )
    } else {
        (-350.0, 350.0, -250.0, 250.0) // Default bounds
    };

    // Spawn Wells (3-5 wells)
    let num_wells = rng.random_range(3..=5);
    for _ in 0..num_wells {
        let position = Vec2::new(
            rng.random_range(min_x..=max_x),
            rng.random_range(min_y..=max_y),
        );

        commands.spawn((
            Well {
                water_level: 1.0,
                max_water: 1.0,
                regeneration_rate: 0.02,
            },
            Thing {
                thing_type: ThingType::Water,
                availability: 1.0,
                max_interactions: 5,
                current_interactions: 0,
                regeneration_rate: 0.02,
                regeneration_timer: 0.0,
            },
            Sprite {
                image: asset_server.load("well.png"),
                ..default()
            },
            Transform::from_translation(position.extend(0.0)),
            Name::new("Well"),
        ));
    }

    // Spawn Restaurants (2-4 restaurants)
    let num_restaurants = rng.random_range(2..=4);
    for _ in 0..num_restaurants {
        let position = Vec2::new(
            rng.random_range(min_x..=max_x),
            rng.random_range(min_y..=max_y),
        );

        commands.spawn((
            Restaurant {
                food_level: 1.0,
                max_food: 1.0,
                regeneration_rate: 0.01,
            },
            Thing {
                thing_type: ThingType::Food,
                availability: 1.0,
                max_interactions: 6,
                current_interactions: 0,
                regeneration_rate: 0.01,
                regeneration_timer: 0.0,
            },
            Sprite {
                image: asset_server.load("restaurant.png"),
                ..default()
            },
            Transform::from_translation(position.extend(0.0)),
            Name::new("Restaurant"),
        ));
    }

    // Spawn Hotels (1-3 hotels)
    let num_hotels = rng.random_range(1..=3);
    for _ in 0..num_hotels {
        let position = Vec2::new(
            rng.random_range(min_x..=max_x),
            rng.random_range(min_y..=max_y),
        );

        commands.spawn((
            Hotel {
                rest_capacity: 1.0,
                available_beds: 10,
                max_beds: 10,
                comfort_level: 0.8,
            },
            Thing {
                thing_type: ThingType::Rest,
                availability: 1.0,
                max_interactions: 8,
                current_interactions: 0,
                regeneration_rate: 0.025,
                regeneration_timer: 0.0,
            },
            Sprite {
                image: asset_server.load("hotel.png"),
                ..default()
            },
            Transform::from_translation(position.extend(0.0)),
            Name::new("Hotel"),
        ));
    }

    // Spawn Safe Zones (1-2 safe zones)
    let num_safe_zones = rng.random_range(1..=2);
    for _ in 0..num_safe_zones {
        let position = Vec2::new(
            rng.random_range(min_x..=max_x),
            rng.random_range(min_y..=max_y),
        );

        commands.spawn((
            SafeZone {
                safety_level: 0.9,
                influence_radius: 50.0,
                capacity: 15,
                current_occupancy: 0,
            },
            Sprite {
                image: asset_server.load("safezone.png"),
                ..default()
            },
            Transform::from_translation(position.extend(0.0)),
            Name::new("Safe Zone"),
        ));
    }

    println!("Environmental resources spawned: {} wells, {} restaurants, {} hotels, {} safe zones",
             num_wells, num_restaurants, num_hotels, num_safe_zones);
}
