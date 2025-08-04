// TODO: Update these imports once the refactoring is complete
use crate::ai::physiology::needs::BasicNeeds;
use crate::core::entities::Npc;
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

/// Event-driven system that handles social interactions based on Social Exchange Theory
/// System based on Social Exchange Theory - positive interactions increase social satisfaction
/// Only triggers when collision events occur, not on every frame
pub fn handle_social_interactions(
    mut collision_events: EventReader<CollisionEvent>,
    mut needs_query: Query<&mut BasicNeeds, With<Npc>>,
) {
    const SOCIAL_INTERACTION_BOOST: f32 = 0.1; // Normalized boost for 0.0-1.0 scale

    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            // Try to get both entities' BasicNeeds components
            if let Ok([mut needs1, mut needs2]) = needs_query.get_many_mut([*entity1, *entity2]) {
                // Both NPCs gain social satisfaction from the interaction
                needs1.social = (needs1.social + SOCIAL_INTERACTION_BOOST).min(1.0);
                needs2.social = (needs2.social + SOCIAL_INTERACTION_BOOST).min(1.0);
            }
        }
    }
}
