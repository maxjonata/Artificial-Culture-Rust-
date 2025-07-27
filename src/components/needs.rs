use bevy::prelude::*;

/// Component representing an NPC's basic needs
#[derive(Component, Reflect, PartialEq, Debug)]
#[reflect(Component)]
pub struct BasicNeeds {
    /// Hunger level
    pub hunger: f32,
    /// Thirst level
    pub thirst: f32,
    /// Fatigue level
    pub fatigue: f32,
    /// Safety level
    pub safety: f32,
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