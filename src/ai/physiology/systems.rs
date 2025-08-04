use crate::ai::physiology::needs::BasicNeeds;
// Updated imports to use new domain structure
use crate::core::constants::GameConstants;
use crate::core::entities::Npc;
use bevy::prelude::*;

/// System implementing homeostatic need decay over time
/// System based on Homeostatic Drive Theory - maintains internal physiological balance
/// FIXED: All needs now use "higher = better satisfied" semantics
pub fn decay_basic_needs(
    mut query: Query<(Entity, &mut BasicNeeds), With<Npc>>,
    game_constants: Res<GameConstants>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (_entity, mut needs) in query.iter_mut() {
        // Decay each need over time based on biological requirements
        // Based on Sterling (2012) allostasis principles
        needs.hunger = (needs.hunger - game_constants.hunger_decay * delta_time).max(0.0);
        needs.thirst = (needs.thirst - game_constants.thirst_decay * delta_time).max(0.0);
        needs.rest = (needs.rest - game_constants.fatigue_regen * delta_time).max(0.0); // Using fatigue_regen for rest decay
        needs.safety = (needs.safety - game_constants.safety_decay * delta_time).max(0.0);
        needs.social = (needs.social - game_constants.social_decay * delta_time).max(0.0);
    }
}
