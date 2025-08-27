use bevy::math::Vec2;
use bevy::prelude::{App, Component, Plugin, Reflect};

/// Defines environment boundaries with wall collision.
#[derive(Component, Debug, Clone, Reflect, Default)]
pub struct EnvironmentBounds {
    pub size: Vec2,
    pub wall_thickness: f32,
}

/// Environment feature (PNG-based objects).
#[derive(Component, Debug, Clone, Reflect, Default)]
pub struct EnvironmentFeature {
    pub texture_path: String,
    pub collider_type: ColliderType,
}

/// Wall generation configuration.
#[derive(Component, Debug, Clone, Reflect, Default)]
pub struct WallGenerator {
    pub pattern: WallPattern,
    pub seed: u64,
}

/// Collider behavior for environment features.
#[derive(Debug, Clone, Reflect, Default)]
pub enum ColliderType {
    #[default]
    Solid,
    Sensor,
    None,
}

/// Wall generation patterns.
#[derive(Debug, Clone, Reflect)]
pub enum WallPattern {
    Random { count: u32, length_range: (f32, f32) },
    Labyrinth { cell_size: f32 },
}

impl Default for WallPattern {
    fn default() -> Self {
        WallPattern::Random {
            count: 10,
            length_range: (30.0, 80.0),
        }
    }
}

pub struct RegisterEnvironmentComponentsPlugin;

impl Plugin for RegisterEnvironmentComponentsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<EnvironmentBounds>()
            .register_type::<EnvironmentFeature>()
            .register_type::<WallGenerator>()
            .register_type::<ColliderType>()
            .register_type::<WallPattern>();
    }
}