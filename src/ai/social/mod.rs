pub mod interaction;   // Social interactions and relationship building
pub mod reputation;    // Community reputation and gossip systems  
pub mod systems;

use bevy::prelude::*;

pub struct SocialPlugin;

impl Plugin for SocialPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register components from this domain
            .register_type::<interaction::SocialInteractionState>()
            .register_type::<interaction::SocialRelationshipNetwork>()
            .register_type::<reputation::Reputation>()
            .register_type::<reputation::CommunityMemory>()
            // Register events
            .add_event::<interaction::InteractionStarted>()
            .add_event::<interaction::InteractionCompleted>()
            .add_event::<reputation::ReputationChanged>()
            // Add systems from this domain
            .add_systems(Update, (
                interaction::social_interaction_system,
                reputation::reputation_update_system,
                reputation::gossip_propagation_system,
            ));
    }
}
