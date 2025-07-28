use crate::components::components_environment::{Hotel, InteractableResource, ResourceType, Restaurant, SafeZone, Well};
use crate::components::components_resources::GameConstants;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

/// Builder function to spawn a well (water source) entity
/// System based on Environmental Psychology - resource placement affects accessibility
pub fn spawn_well(
    commands: &mut Commands,
    position: Vec2,
    well_id: usize,
) {
    commands.spawn((
        Well::default(),
        InteractableResource {
            resource_type: ResourceType::Water,
            availability: 100.0,
            max_interactions: 3,
            current_interactions: 0,
            regeneration_timer: 0.0,
        },
        Sprite {
            color: Color::srgb(0.2, 0.6, 1.0), // Blue for water
            custom_size: Some(Vec2::splat(40.0)),
            ..default()
        },
        Name::new(format!("Well {}", well_id + 1)),
        Transform::from_xyz(position.x, position.y, 0.0),
        RigidBody::Fixed, // Wells don't move
        Collider::cuboid(20.0, 20.0), // Square collider
        Sensor, // Allow NPCs to pass through but detect collisions
    ));
}

/// Builder function to spawn a restaurant (food source) entity
/// System based on Environmental Psychology - resource placement affects accessibility
pub fn spawn_restaurant(
    commands: &mut Commands,
    position: Vec2,
    restaurant_id: usize,
) {
    commands.spawn((
        Restaurant::default(),
        InteractableResource {
            resource_type: ResourceType::Food,
            availability: 100.0,
            max_interactions: 4,
            current_interactions: 0,
            regeneration_timer: 0.0,
        },
        Sprite {
            color: Color::srgb(0.8, 0.4, 0.2), // Brown for restaurant
            custom_size: Some(Vec2::splat(50.0)),
            ..default()
        },
        Name::new(format!("Restaurant {}", restaurant_id + 1)),
        Transform::from_xyz(position.x, position.y, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(25.0, 25.0),
        Sensor,
    ));
}

/// Builder function to spawn a hotel (rest/sleep source) entity
/// System based on Environmental Psychology - resource placement affects accessibility
pub fn spawn_hotel(
    commands: &mut Commands,
    position: Vec2,
    hotel_id: usize,
) {
    commands.spawn((
        Hotel::default(),
        InteractableResource {
            resource_type: ResourceType::Rest,
            availability: 100.0,
            max_interactions: 2,
            current_interactions: 0,
            regeneration_timer: 0.0,
        },
        Sprite {
            color: Color::srgb(0.6, 0.3, 0.8), // Purple for hotel
            custom_size: Some(Vec2::splat(60.0)),
            ..default()
        },
        Name::new(format!("Hotel {}", hotel_id + 1)),
        Transform::from_xyz(position.x, position.y, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(30.0, 30.0),
        Sensor,
    ));
}

/// Builder function to spawn a safe zone entity
/// System based on Environmental Psychology - secure spaces affect behavior
pub fn spawn_safe_zone(
    commands: &mut Commands,
    position: Vec2,
    safe_zone_id: usize,
) {
    commands.spawn((
        SafeZone::default(),
        InteractableResource {
            resource_type: ResourceType::Safety,
            availability: 100.0,
            max_interactions: 10, // Many NPCs can feel safe at once
            current_interactions: 0,
            regeneration_timer: 0.0,
        },
        Sprite {
            color: Color::srgb(0.2, 1.0, 0.2), // Bright green for safety
            custom_size: Some(Vec2::splat(70.0)),
            ..default()
        },
        Name::new(format!("Safe Zone {}", safe_zone_id + 1)),
        Transform::from_xyz(position.x, position.y, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(35.0, 35.0),
        Sensor,
    ));
}

/// Spawn environmental resources randomly across the space
/// Based on spatial distribution theory for accessible resource placement
pub fn spawn_environmental_resources(
    commands: &mut Commands,
    game_constants: &GameConstants,
    window_width: f32,
    window_height: f32,
) {
    let mut rng = rand::rng();

    // Calculate spawn boundaries (leaving margin from edges)
    let margin = 100.0;
    let x_range = -window_width / 2.0 + margin..=window_width / 2.0 - margin;
    let y_range = -window_height / 2.0 + margin..=window_height / 2.0 - margin;

    // Spawn wells (water sources)
    for i in 0..game_constants.num_wells {
        let position = Vec2::new(
            rng.random_range(x_range.clone()),
            rng.random_range(y_range.clone()),
        );
        spawn_well(commands, position, i);
    }

    // Spawn restaurants (food sources)
    for i in 0..game_constants.num_restaurants {
        let position = Vec2::new(
            rng.random_range(x_range.clone()),
            rng.random_range(y_range.clone()),
        );
        spawn_restaurant(commands, position, i);
    }

    // Spawn hotels (rest sources)
    for i in 0..game_constants.num_hotels {
        let position = Vec2::new(
            rng.random_range(x_range.clone()),
            rng.random_range(y_range.clone()),
        );
        spawn_hotel(commands, position, i);
    }

    // Spawn safe zones (safety sources)
    for i in 0..game_constants.num_safe_zones {
        let position = Vec2::new(
            rng.random_range(x_range.clone()),
            rng.random_range(y_range.clone()),
        );
        spawn_safe_zone(commands, position, i);
    }

    info!(
        "Environmental resources spawned: {} wells, {} restaurants, {} hotels, {} safe zones",
        game_constants.num_wells, game_constants.num_restaurants, game_constants.num_hotels, game_constants.num_safe_zones
    );
}
