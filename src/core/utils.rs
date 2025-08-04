use bevy::prelude::*;

/// Utility functions that are used across multiple domains
/// These are truly generic functions not tied to any specific AI domain
/// Calculate distance between two Vec3 positions
pub fn distance_3d(pos1: Vec3, pos2: Vec3) -> f32 {
    pos1.distance(pos2)
}

/// Calculate distance between two Vec2 positions
pub fn distance_2d(pos1: Vec2, pos2: Vec2) -> f32 {
    pos1.distance(pos2)
}

/// Normalize a value between 0.0 and 1.0
pub fn normalize_value(value: f32, min: f32, max: f32) -> f32 {
    ((value - min) / (max - min)).clamp(0.0, 1.0)
}

/// Linear interpolation between two f32 values
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

/// Convert degrees to radians
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

/// Convert radians to degrees
pub fn rad_to_deg(radians: f32) -> f32 {
    radians * 180.0 / std::f32::consts::PI
}

/// Calculate angle between two 2D points
pub fn angle_between_points(from: Vec2, to: Vec2) -> f32 {
    (to - from).y.atan2((to - from).x)
}

/// Generate a random value between min and max (requires rand crate)
/// Note: Consider using Bevy's built-in random utilities in production
pub fn random_range(min: f32, max: f32) -> f32 {
    use rand::Rng;
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}
