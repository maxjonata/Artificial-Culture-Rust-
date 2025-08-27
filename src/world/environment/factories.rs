use crate::world::environment::bundles::{EnvironmentBoundsBundle, EnvironmentFeatureBundle, WallBundle};
use crate::world::environment::components::{ColliderType, EnvironmentBounds, EnvironmentFeature};
use crate::world::environment::helpers;
use bevy::asset::AssetServer;
use bevy::log::info;
use bevy::math::{Quat, Vec2};
use bevy::prelude::{default, Commands, Entity, GlobalTransform, InheritedVisibility, Res, Sprite, Transform, ViewVisibility, Visibility};
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::{Collider, Sensor};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

/// Spawn environment boundaries with collision walls.
pub fn spawn_bounds(commands: &mut Commands, size: Vec2, wall_thickness: f32) -> Entity {
    let half_size = size * 0.5;
    let half_thickness = wall_thickness * 0.5;

    commands.spawn(EnvironmentBoundsBundle {
        bounds: EnvironmentBounds {
            size,
            wall_thickness,
        },
        transform: Transform::default(),
        global_transform: GlobalTransform::default(),
        rigid_body: RigidBody::Fixed,
        collider: Collider::compound(vec![
            // Top wall
            (Vec2::new(0.0, half_size.y + half_thickness), 0.0,
             Collider::cuboid(half_size.x + wall_thickness, half_thickness)),
            // Bottom wall
            (Vec2::new(0.0, -half_size.y - half_thickness), 0.0,
             Collider::cuboid(half_size.x + wall_thickness, half_thickness)),
            // Left wall
            (Vec2::new(-half_size.x - half_thickness, 0.0), 0.0,
             Collider::cuboid(half_thickness, half_size.y)),
            // Right wall
            (Vec2::new(half_size.x + half_thickness, 0.0), 0.0,
             Collider::cuboid(half_thickness, half_size.y)),
        ]),
    }).id()
}

/// Spawn PNG-based environment feature.
pub fn spawn_feature(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec2,
    texture_path: &str,
    collider_type: ColliderType,
    scale: Vec2,
) -> Entity {
    let texture = asset_server.load(texture_path);

    let collider = match collider_type {
        ColliderType::Solid => Collider::cuboid(scale.x * 0.5, scale.y * 0.5),
        ColliderType::Sensor => {
            // For sensors, we'll add the Sensor component separately
            Collider::cuboid(scale.x * 0.5, scale.y * 0.5)
        }
        ColliderType::None => Collider::cuboid(0.1, 0.1), // Minimal collider
    };

    let mut entity_commands = commands.spawn(EnvironmentFeatureBundle {
        feature: EnvironmentFeature {
            texture_path: texture_path.to_string(),
            collider_type: collider_type.clone(),
        },
        sprite: Sprite {
            image: texture,
            custom_size: Some(scale),
            ..default()
        },
        transform: Transform::from_translation(position.extend(0.0)),
        global_transform: GlobalTransform::default(),
        visibility: Visibility::Visible,
        inherited_visibility: InheritedVisibility::default(),
        view_visibility: ViewVisibility::default(),
        rigid_body: RigidBody::Fixed,
        collider,
    });

    // Add sensor component if needed
    if matches!(collider_type, ColliderType::Sensor) {
        entity_commands.insert(Sensor);
    }

    entity_commands.id()
}

/// Spawn random walls within bounds.
pub fn spawn_random_walls(
    commands: &mut Commands,
    bounds: Vec2,
    count: u32,
    length_range: (f32, f32),
    seed: u64,
) {
    let mut rng = StdRng::seed_from_u64(seed);
    let half_bounds = bounds * 0.4; // Stay away from edges

    for _ in 0..count {
        let length = rng.random_range(length_range.0..=length_range.1);
        let angle = rng.random_range(0.0..std::f32::consts::TAU);
        let center = Vec2::new(
            rng.random_range(-half_bounds.x..=half_bounds.x),
            rng.random_range(-half_bounds.y..=half_bounds.y),
        );

        commands.spawn(WallBundle::new(center, angle, length));
    }
}

/// Generate simple labyrinth with recursive backtracking.
pub fn spawn_labyrinth(
    commands: &mut Commands,
    bounds: Vec2,
    cell_size: f32,
    seed: u64,
) {
    let grid_width = (bounds.x / cell_size) as usize;
    let grid_height = (bounds.y / cell_size) as usize;

    let walls = helpers::generate_maze_walls(grid_width, grid_height, cell_size, bounds, seed);

    for (start, end) in walls {
        let center = (start + end) * 0.5;
        let direction = end - start;
        let length = direction.length();
        let angle = direction.angle_to(Vec2::X);

        commands.spawn(WallBundle::new(center, angle, length));
    }
}

/// Create obstacle course with random walls and features.
pub fn create_obstacle_course_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    spawn_bounds(&mut commands, Vec2::new(600.0, 400.0), 5.0);
    spawn_random_walls(&mut commands, Vec2::new(600.0, 400.0), 15, (30.0, 80.0), 42);

    // Add some test features (replace with actual asset paths)
    for i in 0..3 {
        spawn_feature(
            &mut commands,
            &asset_server,
            Vec2::new(i as f32 * 100.0 - 100.0, 50.0),
            "textures/tree.png", // Replace with actual asset
            ColliderType::Solid,
            Vec2::new(32.0, 32.0),
        );
    }

    info!("Created obstacle course (600x400) with 15 walls and 3 features");
}

/// Create labyrinth environment.
pub fn create_labyrinth_system(mut commands: Commands) {
    spawn_bounds(&mut commands, Vec2::new(500.0, 500.0), 5.0);
    spawn_labyrinth(&mut commands, Vec2::new(450.0, 450.0), 32.0, 123);
    info!("Created labyrinth (500x500) with 32px cells");
}