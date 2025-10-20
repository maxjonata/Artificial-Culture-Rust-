use crate::core::types::Normalized;
use bevy::prelude::{Component, Reflect};

#[derive(Component, Debug, Reflect)]
pub struct Hunger {
    pub value: Normalized,
}
