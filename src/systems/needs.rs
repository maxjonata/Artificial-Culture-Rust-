use bevy::prelude::{EventReader, Query, Res, With};
use bevy_rapier2d::prelude::CollisionEvent;
use crate::components::{GameConstants, Npc};
use crate::components::needs::BasicNeeds;

impl BasicNeeds {
    /// Applies decay to all needs based on game constants
    /// Follows Single Responsibility Principle by encapsulating need decay logic
    fn apply_decay(&mut self, game_constants: &GameConstants) {
        self.hunger = (self.hunger - game_constants.hunger_decay).clamp(0.0, 100.0);
        self.thirst = (self.thirst - game_constants.thirst_decay).clamp(0.0, 100.0);
        self.fatigue = (self.fatigue + game_constants.fatigue_regen).clamp(0.0, 100.0);
        self.safety = (self.safety - game_constants.safety_decay).clamp(0.0, 100.0);
        self.social = (self.social - game_constants.social_decay).clamp(0.0, 100.0);
    }

    /// Increases the social need when interacting with other NPCs
    /// Follows Single Responsibility Principle for social interactions
    pub fn increase_social(&mut self, amount: f32) {
        self.social = (self.social + amount).clamp(0.0, 100.0);
    }
}

/// System that applies decay to all NPC needs over time
/// Follows Single Responsibility Principle by handling only need decay
pub fn decay_basic_needs(
    mut query: Query<&mut BasicNeeds, With<Npc>>,
    game_constants: Res<GameConstants>,
) {
    for mut needs in query.iter_mut() {
        needs.apply_decay(&game_constants);
    }
}

/// System that handles social need satisfaction when NPCs interact through collisions
/// Follows Open/Closed Principle by extending collision behavior without modifying existing code
pub fn handle_social_interactions(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<&mut BasicNeeds, With<Npc>>,
) {
    const SOCIAL_INTERACTION_BOOST: f32 = 10.0;

    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            // Try to get both entities' BasicNeeds components
            if let Ok([mut needs1, mut needs2]) = query.get_many_mut([*entity1, *entity2]) {
                // Both NPCs gain social satisfaction from the interaction
                needs1.increase_social(SOCIAL_INTERACTION_BOOST);
                needs2.increase_social(SOCIAL_INTERACTION_BOOST);
            }
        }
    }
}