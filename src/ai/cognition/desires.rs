use bevy::prelude::*;

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

/// Component that stores an agent's current decision/desire after evaluation
/// This is the missing component from roadmap 1.3.2 that gets set by the decision_making_system
#[derive(Component, Debug, Reflect, Default, Clone, Copy)]
#[reflect(Component)]
pub struct CurrentDesire {
    /// The agent's current active desire/goal
    pub desire: Desire,
    /// The utility score of this desire when it was selected
    /// ML-HOOK: Quantifiable decision strength for observation space
    pub utility_score: f32,
    /// Timestamp when this desire was last evaluated
    pub last_evaluated: f32,
    /// NEW: Failure tracking for adaptive behavior
    pub failure_count: u8,
    /// NEW: Time when the current attempt started
    pub attempt_start_time: f32,
    /// NEW: Maximum time to spend on this desire before considering it failed
    pub timeout_duration: f32,
    /// NEW: Last known target entity (resource, NPC, etc.) for this desire
    pub last_target: Option<Entity>,
}
