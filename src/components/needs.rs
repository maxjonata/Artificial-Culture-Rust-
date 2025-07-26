use bevy::prelude::*;

/// Component representing an NPC's basic needs
#[derive(Component, Reflect, PartialEq, Debug)]
#[reflect(Component)]
pub struct Needs {
    /// Hunger level
    pub fome: f32,
    /// Thirst level
    pub sede: f32,
    /// Fatigue level
    pub fadiga: f32,
    /// Safety level
    pub seguranca: f32,
    /// Social need level
    pub social: f32,
}

/// Enum representing an NPC's current desire/goal
#[derive(Component, Reflect, PartialEq, Debug, Default)]
#[reflect(Component)]
pub enum Desire {
    /// Default desire - wander around
    #[default]
    Wander,
    /// Find food to satisfy hunger
    FindFood,
    /// Find safety when threatened
    FindSafety,
    /// Rest to recover from fatigue
    Rest,
    /// Socialize with other NPCs
    Socialize
}