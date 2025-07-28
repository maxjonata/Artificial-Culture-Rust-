use bevy::prelude::*;

/// Component marking an entity as a well (water source)
/// System based on Environmental Psychology - resource availability affects behavior
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Well {
    /// Water capacity remaining (0.0 to 100.0)
    pub water_capacity: f32,
    /// Rate at which water is consumed per interaction
    pub consumption_rate: f32,
    /// Rate at which water regenerates over time
    pub regeneration_rate: f32,
}

/// Component marking an entity as a restaurant (food source)
/// System based on Environmental Psychology - resource availability affects behavior
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Restaurant {
    /// Food capacity remaining (0.0 to 100.0)
    pub food_capacity: f32,
    /// Rate at which food is consumed per interaction
    pub consumption_rate: f32,
    /// Rate at which food regenerates over time
    pub regeneration_rate: f32,
}

/// Component marking an entity as a hotel (rest/sleep source)
/// System based on Environmental Psychology - resource availability affects behavior
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Hotel {
    /// Rest capacity remaining (0.0 to 100.0) - represents available beds/rooms
    pub rest_capacity: f32,
    /// Rate at which rest is consumed per interaction
    pub consumption_rate: f32,
    /// Rate at which rest capacity regenerates over time
    pub regeneration_rate: f32,
}

/// Component marking an entity as a safe zone (safety source)
/// System based on Environmental Psychology - secure spaces affect behavior
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SafeZone {
    /// Safety level provided by this zone (0.0 to 100.0)
    pub safety_level: f32,
    /// Radius of the safe zone effect
    pub effect_radius: f32,
}

/// Component for environmental entities that can be interacted with
/// ML-HOOK: Interaction tracking for reward calculation
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractableResource {
    /// Type of resource this entity provides
    pub resource_type: ResourceType,
    /// Current availability (0.0 to 100.0)
    pub availability: f32,
    /// Maximum interactions per time period
    pub max_interactions: u32,
    /// Current interactions this period
    pub current_interactions: u32,
    /// Time until next regeneration cycle
    pub regeneration_timer: f32,
}

/// Enum defining types of environmental resources
#[derive(Debug, Reflect, PartialEq, Clone, Copy)]
pub enum ResourceType {
    Water,
    Food,
    Rest,
    Safety,
}
