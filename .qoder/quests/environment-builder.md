# Environment Builder Design Document

## Overview

Environment Builder uses **standard Bevy ECS patterns** - no custom builders, no state machines, no overengineering. Pure Bevy conventions with component bundles, factory functions, and direct spawning.

Physics configured for top-down movement with no gravity via `bevy_rapier2d`.

## Technology Stack & Dependencies

- **Bevy ECS Framework**: Standard Commands::spawn() patterns
- **bevy_rapier2d**: Physics (no gravity, top-down)
- **PNG Image Loading**: Bevy's AssetServer for texture-based colliders

## Standard Bevy Components

```rust
#[derive(Component, Debug, Clone, Reflect, Default)]
pub struct EnvironmentBounds {
    pub size: Vec2,
    pub wall_thickness: f32,
}

#[derive(Component, Debug, Clone, Reflect, Default)]
pub struct EnvironmentFeature {
    pub texture_path: String,
    pub collider_type: ColliderType,
}

#[derive(Component, Debug, Clone, Reflect, Default)]
pub struct WallGenerator {
    pub pattern: WallPattern,
    pub seed: u64,
}

#[derive(Debug, Clone, Reflect)]
pub enum ColliderType {
    Solid,
    Sensor,
    None,
}

#[derive(Debug, Clone, Reflect)]
pub enum WallPattern {
    Random { count: u32, length_range: (f32, f32) },
    Labyrinth { cell_size: f32 },
}
```

### Bundle Definitions

```rust
#[derive(Bundle)]
pub struct EnvironmentBoundsBundle {
    pub bounds: EnvironmentBounds,
    pub transform: Transform,
    pub rigid_body: RigidBody,
    pub collider: Collider,
}

#[derive(Bundle)]
pub struct EnvironmentFeatureBundle {
    pub feature: EnvironmentFeature,
    pub sprite: SpriteBundle,
    pub rigid_body: RigidBody,
    pub collider: Collider,
}

#[derive(Bundle)]
pub struct WallBundle {
    pub transform: Transform,
    pub rigid_body: RigidBody,
    pub collider: Collider,
}
```

### Factory Functions (Standard Bevy Convention)

```rust
// Environment bounds factory
pub fn spawn_bounds(commands: &mut Commands, size: Vec2) -> Entity {
    let half_size = size * 0.5;
    commands.spawn(EnvironmentBoundsBundle {
        bounds: EnvironmentBounds {
            size,
            wall_thickness: 5.0,
        },
        transform: Transform::default(),
        rigid_body: RigidBody::Fixed,
        collider: Collider::compound(vec![
            (Vec2::new(0.0, half_size.y), 0.0, Collider::cuboid(half_size.x, 2.5)),
            (Vec2::new(0.0, -half_size.y), 0.0, Collider::cuboid(half_size.x, 2.5)),
            (Vec2::new(-half_size.x, 0.0), 0.0, Collider::cuboid(2.5, half_size.y)),
            (Vec2::new(half_size.x, 0.0), 0.0, Collider::cuboid(2.5, half_size.y)),
        ]),
    }).id()
}

// PNG feature factory
pub fn spawn_feature(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec2,
    texture_path: &str,
    collider_type: ColliderType,
) -> Entity {
    let texture = asset_server.load(texture_path);
    
    commands.spawn(EnvironmentFeatureBundle {
        feature: EnvironmentFeature {
            texture_path: texture_path.to_string(),
            collider_type: collider_type.clone(),
        },
        sprite: SpriteBundle {
            texture,
            transform: Transform::from_translation(position.extend(0.0)),
            ..default()
        },
        rigid_body: RigidBody::Fixed,
        collider: match collider_type {
            ColliderType::Solid => Collider::cuboid(16.0, 16.0),
            ColliderType::Sensor => Collider::sensor(Collider::cuboid(16.0, 16.0)),
            ColliderType::None => Collider::cuboid(0.0, 0.0),
        },
    }).id()
}

// Random walls factory
pub fn spawn_random_walls(
    commands: &mut Commands,
    bounds: Vec2,
    count: u32,
    seed: u64,
) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    
    for _ in 0..count {
        let length = rng.gen_range(30.0..100.0);
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let center = Vec2::new(
            rng.gen_range(-bounds.x/2.0..bounds.x/2.0),
            rng.gen_range(-bounds.y/2.0..bounds.y/2.0),
        );
        
        commands.spawn(WallBundle {
            transform: Transform::from_translation(center.extend(0.0))
                .with_rotation(Quat::from_rotation_z(angle)),
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(length/2.0, 2.5),
        });
    }
}

// Labyrinth factory
pub fn spawn_labyrinth(
    commands: &mut Commands,
    bounds: Vec2,
    cell_size: f32,
    seed: u64,
) {
    let grid_width = (bounds.x / cell_size) as usize;
    let grid_height = (bounds.y / cell_size) as usize;
    
    // Generate maze using recursive backtracking
    let walls = generate_maze_walls(grid_width, grid_height, cell_size, seed);
    
    for (start, end) in walls {
        let center = (start + end) * 0.5;
        let length = start.distance(end);
        let angle = (end - start).angle_between(Vec2::X);
        
        commands.spawn(WallBundle {
            transform: Transform::from_translation(center.extend(0.0))
                .with_rotation(Quat::from_rotation_z(angle)),
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(length/2.0, 2.5),
        });
    }
}
```

## PNG Collider Generation

```rust
pub fn generate_collider_from_png(
    image: &Image,
    scale: Vec2,
) -> Collider {
    // Extract alpha channel from PNG
    let pixels = &image.data;
    let width = image.texture_descriptor.size.width as usize;
    let height = image.texture_descriptor.size.height as usize;
    
    // Create collision mesh from non-transparent pixels
    let mut vertices = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let alpha = pixels[(y * width + x) * 4 + 3];
            if alpha > 128 {
                vertices.push(Vec2::new(
                    (x as f32 - width as f32 / 2.0) * scale.x / width as f32,
                    (y as f32 - height as f32 / 2.0) * scale.y / height as f32,
                ));
            }
        }
    }
    
    if vertices.is_empty() {
        Collider::cuboid(scale.x/2.0, scale.y/2.0)
    } else {
        Collider::convex_hull(&vertices).unwrap_or_else(|| 
            Collider::cuboid(scale.x/2.0, scale.y/2.0)
        )
    }
}
```

## Usage Examples

### Standard Bevy Spawning

```rust
// Empty room - direct spawning
fn create_empty_room(mut commands: Commands) {
    spawn_bounds(&mut commands, Vec2::new(400.0, 300.0));
}

// Obstacle course - multiple spawns
fn create_obstacle_course(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let bounds = spawn_bounds(&mut commands, Vec2::new(600.0, 400.0));
    spawn_random_walls(&mut commands, Vec2::new(600.0, 400.0), 15, 42);
    
    // Add some PNG features
    for i in 0..5 {
        spawn_feature(
            &mut commands,
            &asset_server,
            Vec2::new(i as f32 * 50.0 - 100.0, 0.0),
            "assets/textures/tree.png",
            ColliderType::Solid,
        );
    }
}

// Labyrinth - maze generation
fn create_labyrinth(mut commands: Commands) {
    spawn_bounds(&mut commands, Vec2::new(500.0, 500.0));
    spawn_labyrinth(&mut commands, Vec2::new(450.0, 450.0), 32.0, 123);
}
```

