//! Core entity definitions.

use bevy::prelude::*;

/// Non-Player Character with cognitive capabilities.
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Npc {
    pub id: u32,
    pub name: String,
}
