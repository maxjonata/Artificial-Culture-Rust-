use crate::ai::cognition::components::{PersonalityDimension, RoleAffinity};
use crate::ai::social::components::AgentEvent;
use crate::core::types::{Normalized, Severity};
use bevy::prelude::{Entity, Event};
// Inputs: PersonalityVector, StressLevel, RecentEvents, RoleAffinities
// Processing: Apply stress-based temporary shifts to personality dimensions using sigmoid curves.
//  High stress increases neuroticism (+0.1-0.3), decreases openness (-0.1-0.2).
//  Traumatic events create permanent small shifts (-0.05 to +0.05).
//  Calculate role affinity changes based on successful/failed leadership attempts.
// Outputs: Modified PersonalityVector, updated RoleAffinities, PersonalityShiftEvent

#[derive(Event)]
pub struct StressThresholdCrossed {
    entity: Entity,
    new_stress_level: Normalized,
    old_stress_level: Normalized,
    stress_threshold: Normalized,
}

#[derive(Event)]
pub struct TraumaticEvent {
    entity: Entity,
    recent_events: Vec<AgentEvent>,
    severity: Severity,
}

#[derive(Event)]
pub struct SuccessfulLeadership {
    entity: Entity,
    success_count: u32,
    failure_count: u32,
    context: AgentEvent,
}

#[derive(Event)]
pub struct PersonalityShiftEvent {
    entity: Entity,
    personality_changes: Vec<(PersonalityDimension, f32)>,
    role_affinity_changes: Vec<(RoleAffinity, f32)>,
}
