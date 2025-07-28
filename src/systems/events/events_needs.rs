use crate::components::components_needs::Desire;
use bevy::prelude::{Entity, Event};

// ML-HOOK: Events for quantifiable behavior tracking and reward calculation
#[derive(Event)]
pub struct NeedDecayEvent {
    pub entity: Entity,
    pub hunger_change: f32,
    pub thirst_change: f32,
    pub fatigue_change: f32,
    pub safety_change: f32,
    pub social_change: f32,
}

#[derive(Event)]
pub struct DesireChangeEvent {
    pub entity: Entity,
    pub old_desire: Desire,
    pub new_desire: Desire,
    pub urgency_score: f32, // ML-HOOK: Quantifiable urgency for reward calculation
}

#[derive(Event)]
pub struct SocialInteractionEvent {
    pub entity_1: Entity,
    pub entity_2: Entity,
    pub social_boost: f32,
}