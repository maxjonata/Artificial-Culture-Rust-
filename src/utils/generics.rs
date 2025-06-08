use bevy_ecs::prelude::*;
use rand::Rng;

pub fn get_random_weight() -> f32 {
    // Generate a random weight between 0.0 and 1.0
    let mut rng = rand::rng();
    rng.random_range(0.0..=1.0)
}

#[derive(Component, Debug, Default)]
pub struct UniqueIdentity {
    pub entity_id: u32,
    pub name: String,
    pub description: String,
}
