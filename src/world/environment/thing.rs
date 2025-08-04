use bevy::prelude::*;

/// Unified Thing component representing any type of collectable item in the environment
/// Based on Environmental Psychology - item availability affects behavior
/// All values normalized to 0.0-1.0 for ML compatibility and consistency
#[derive(Component, Debug, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct Thing {
    /// Type of Thing this entity provides
    pub thing_type: ThingType,
    /// Current availability (0.0 = depleted, 1.0 = full capacity) - only for consumable items
    pub availability: f32,
    /// Maximum number of simultaneous interactions (data-oriented: prefer u8 for memory efficiency)
    pub max_interactions: u8,
    /// Current number of NPCs interacting with this Thing
    pub current_interactions: u8,
    /// Regeneration rate per second (0.0-1.0 normalized) - only for consumable items
    pub regeneration_rate: f32,
    /// Timer for regeneration calculations (internal system use)
    pub regeneration_timer: f32,
}

/// Thing types that satisfy different needs
/// Based on Maslow's hierarchy - different items satisfy different need levels
#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThingType {
    /// Satisfies thirst need - highest priority survival Thing
    #[default]
    Water,
    /// Satisfies hunger need - essential survival Thing
    Food,
    /// Satisfies fatigue need - physiological restoration
    Rest,
    /// Satisfies safety need - security and protection
    Safety,
    /// Social gathering spaces - satisfies social by providing social interaction
    Social,
}

/// Component for entities that can own and transfer Things
/// Based on Economic Theory - Thing ownership and exchange
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct ThingOwnership {
    /// Things currently owned by this entity (data-oriented: use Entity IDs for O(1) access)
    pub owned_things: Vec<Entity>,
    /// Maximum Things this entity can own (u8 for memory efficiency)
    pub max_ownership_capacity: u8,
    /// Current Thing transfer in progress (None if no active transfer)
    pub active_transfer: Option<ThingTransfer>,
}

/// Thing transfer data structure
/// Based on Economic Exchange Theory - formalized transfer mechanisms
#[derive(Debug, Reflect, Clone)]
pub struct ThingTransfer {
    /// Entity giving the Thing
    pub from_entity: Entity,
    /// Entity receiving the Thing
    pub to_entity: Entity,
    /// The Thing being transferred
    pub thing_entity: Entity,
    /// Transfer completion progress (0.0-1.0)
    pub progress: f32,
    /// Time remaining for transfer completion
    pub time_remaining: f32,
}

/// Component marking entities as interactable Thing sources
/// This replaces the specific Well, Restaurant, etc. components with a unified system
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct InteractableThing {
    /// Type of Thing this interaction provides
    pub thing_type: ThingType,
    /// Interaction range in pixels (how close NPCs need to be)
    pub interaction_range: f32,
    /// Whether this Thing source is currently available for interaction
    pub is_available: bool,
    /// Time required to complete one interaction (in seconds)
    pub interaction_duration: f32,
}

// ==========================================
// LEGACY COMPONENTS - TO BE PHASED OUT
// ==========================================
// These will be phased out in favor of the unified Thing system

/// Legacy Well component - to be replaced by Thing system
/// Based on Thing Economics - water as a finite renewable Thing
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Well {
    pub water_level: f32,
    pub max_water: f32,
    pub regeneration_rate: f32,
}

/// Legacy Restaurant component - to be replaced by Thing system
/// Based on Thing Economics - food as a finite renewable Thing
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Restaurant {
    pub food_level: f32,
    pub max_food: f32,
    pub regeneration_rate: f32,
}

/// Legacy Hotel component - to be replaced by Thing system
/// Based on Environmental Psychology - hotels as safe restoration zones
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Hotel {
    /// Rest capacity - always 1.0 for hotels (unlimited rest)
    pub rest_capacity: f32,
    /// Number of available beds (u8 for memory efficiency)
    pub available_beds: u8,
    /// Maximum bed capacity
    pub max_beds: u8,
    pub comfort_level: f64,
}

/// Legacy SafeZone component - to be replaced by Thing system
/// Based on Environmental Psychology - safe spaces for security needs
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct SafeZone {
    /// Safety level provided (0.0-1.0 normalized)
    pub safety_level: f32,
    /// Radius of safety influence
    pub influence_radius: f32,
    pub capacity: i32,
    pub current_occupancy: i32,
}
