pub mod decision;     // Decision-making using utility-based AI
pub mod desires;
pub mod inference;    // Bayesian belief systems and logical inference  
pub mod knowledge;
pub mod memory;       // Working memory, episodic memory, semantic memory
pub mod personality;

use bevy::prelude::*;

pub struct CognitionPlugin;

impl Plugin for CognitionPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register components from this domain
            .register_type::<decision::DesireDecision>()
            .register_type::<decision::GoalType>()
            .register_type::<memory::CognitiveWorkingMemory>()
            .register_type::<memory::CognitiveEpisodicMemory>()
            .register_type::<memory::CognitiveSemanticMemory>()
            .register_type::<inference::Belief>()
            .register_type::<inference::InferenceEngine>()
            // Register events
            .add_event::<decision::DecisionMade>()
            .add_event::<inference::BeliefUpdated>()
            // Add systems from this domain
            .add_systems(Update, (
                decision::decision_making_system,
                memory::working_memory_management_system,
                memory::episodic_memory_decay_system,
                inference::belief_update_system,
                inference::inference_system,
            ));
    }
}
