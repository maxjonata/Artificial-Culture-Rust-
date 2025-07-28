use crate::components::components_environment::ResourceType;
use bevy::prelude::{Entity, Event};

// ML-HOOK: Events for quantifiable resource interaction tracking and reward calculation
#[derive(Event)]
pub struct ResourceInteractionEvent {
    pub npc_entity: Entity,
    pub resource_entity: Entity,
    pub resource_type: ResourceType,
    pub satisfaction_gained: f32,
    pub resource_consumed: f32, // ML-HOOK: Quantifiable resource usage
}

#[derive(Event)]
pub struct ResourceDepletionEvent {
    pub resource_entity: Entity,
    pub resource_type: ResourceType,
    pub remaining_capacity: f32,
}