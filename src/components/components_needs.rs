use bevy::prelude::*;

/// Component representing an NPC's basic needs
/// All values are normalized between 0.0-1.0 for ML compatibility
/// Based on Homeostatic Drive Theory - organisms maintain internal balance
#[derive(Component, Debug, Reflect, Default, Clone, Copy)]
#[reflect(Component)]
pub struct BasicNeeds {
    /// Hunger level (0.0 = starving, 1.0 = fully satisfied)
    pub hunger: f32,
    /// Thirst level (0.0 = dehydrated, 1.0 = fully hydrated)
    pub thirst: f32,
    /// Fatigue level (0.0 = exhausted, 1.0 = fully rested)
    pub fatigue: f32,
    /// Safety level (0.0 = very unsafe, 1.0 = completely safe)
    pub safety: f32,
    /// Social need level (0.0 = isolated, 1.0 = socially fulfilled)
    pub social: f32,
}

/// Enum representing an NPC's current desire/goal
#[derive(Component, Reflect, PartialEq, Debug, Default, Clone, Copy, Hash, Eq)]
#[reflect(Component)]
pub enum Desire {
    /// Default desire - wander around
    #[default]
    Wander,
    /// Find food to satisfy hunger
    FindFood,
    /// Find water to satisfy thirst
    FindWater,
    /// Find safety when threatened
    FindSafety,
    /// Rest to recover from fatigue
    Rest,
    /// Socialize with other NPCs
    Socialize,
}

/// Component that defines thresholds for when desires should be activated
/// Follows Single Responsibility Principle - manages only desire thresholds
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct DesireThresholds {
    /// Hunger threshold below which FindFood desire is triggered
    pub hunger_threshold: f32,
    /// Thirst threshold below which FindWater desire is triggered
    pub thirst_threshold: f32,
    /// Fatigue threshold above which Rest desire is triggered
    pub fatigue_threshold: f32,
    /// Safety threshold below which FindSafety desire is triggered
    pub safety_threshold: f32,
    /// Social threshold below which Socialize desire is triggered
    pub social_threshold: f32,
    /// Priority weights for competing desires (higher = more important)
    pub priority_weights: DesirePriorities,
}

/// Priority system for resolving competing desires
/// Based on Maslow's hierarchy with physiological needs having highest priority
#[derive(Reflect, Debug)]
pub struct DesirePriorities {
    pub hunger: f32,    // Highest priority - survival need
    pub thirst: f32,    // Highest priority - survival need
    pub safety: f32,    // High priority - security need
    pub fatigue: f32,   // Medium priority - physiological need
    pub social: f32,    // Lower priority - social need
}
