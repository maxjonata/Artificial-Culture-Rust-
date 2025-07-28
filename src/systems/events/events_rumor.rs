use bevy::prelude::{Entity, Event};

// ML-HOOK: Events for quantifiable rumor dynamics tracking and reward calculation
#[derive(Event)]
pub struct RumorInjectionEvent {
    pub target_entity: Entity,
    pub injection_time: f32,
}

#[derive(Event)]
pub struct RumorSpreadEvent {
    pub spreader_entity: Entity,
    pub receiver_entity: Entity,
    pub spreader_openness: f32,
    pub receiver_openness: f32,
    pub spread_probability: f32, // ML-HOOK: Quantifiable spread dynamics
}

#[derive(Event)]
pub struct RumorSpreadAttemptEvent {
    pub spreader_entity: Entity,
    pub receiver_entity: Entity,
    pub success: bool,
    pub calculated_probability: f32, // ML-HOOK: Track all attempts for learning
}