use crate::ai::cognition::memory::{CognitiveSemanticMemory, ConceptLink, RelationType};
use bevy::prelude::*;
use std::collections::HashMap;

/// Component representing the agent's belief system about the world
/// Based on Bayesian Inference and Belief Revision Theory
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Belief {
    /// Probabilistic beliefs about world states (Entity -> confidence level)
    pub world_beliefs: HashMap<Entity, f32>,
    /// Beliefs about other agents' intentions
    pub agent_beliefs: HashMap<Entity, AgentBelief>,
    /// Confidence threshold for acting on beliefs
    pub confidence_threshold: f32,
    /// Learning rate for belief updates
    pub belief_learning_rate: f32,
}

/// Beliefs about another agent's state and intentions
#[derive(Debug, Reflect, Clone)]
pub struct AgentBelief {
    /// Believed goal of the other agent
    pub perceived_goal: Option<Entity>,
    /// Confidence in this belief (0.0-1.0)
    pub confidence: f32,
    /// Trustworthiness of this agent (0.0-1.0)
    pub trustworthiness: f32,
    /// Last time this belief was updated
    pub last_updated: f32,
}

/// Component for logical inference capabilities
/// Based on Cognitive Science models of human reasoning
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InferenceEngine {
    /// Maximum number of inference steps per update
    pub max_inference_steps: u8,
    /// Current inference queue (premises to process)
    pub inference_queue: Vec<InferencePremise>,
    /// Strength threshold for making inferences
    pub inference_threshold: f32,
}

/// A premise used for logical inference
#[derive(Debug, Reflect, Clone)]
pub struct InferencePremise {
    /// Subject of the inference
    pub subject: Entity,
    /// Predicate/property being inferred
    pub predicate: Entity,
    /// Confidence in this premise (0.0-1.0)
    pub confidence: f32,
    /// Source of this premise (observation, memory, inference)
    pub source: PremiseSource,
}

/// Source of an inference premise
#[derive(Debug, Reflect, Clone, PartialEq)]
pub enum PremiseSource {
    /// Direct observation through perception
    Observation,
    /// Retrieved from memory
    Memory,
    /// Inferred from other premises
    Inference,
    /// Communication from another agent
    Communication,
}

/// Event fired when new beliefs are formed
#[derive(Event, Debug)]
pub struct BeliefUpdated {
    pub agent: Entity,
    pub subject: Entity,
    pub old_confidence: f32,
    pub new_confidence: f32,
    pub source: PremiseSource,
}

/// System that performs Bayesian belief updates
/// Based on Bayesian Cognitive Models and Pearl's Belief Networks
pub fn belief_update_system(
    mut query: Query<(Entity, &mut Belief)>,
    _belief_events: EventWriter<BeliefUpdated>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();

    for (_entity, mut belief) in query.iter_mut() {
        // Update agent beliefs with temporal decay
        let mut agents_to_remove = Vec::new();

        for (agent_id, agent_belief) in belief.agent_beliefs.iter_mut() {
            let time_since_update = current_time - agent_belief.last_updated;

            // Beliefs decay over time without reinforcement
            let decay_rate = 0.01; // 1% decay per second
            agent_belief.confidence = (agent_belief.confidence - decay_rate * time_since_update).max(0.0);

            // Mark very weak beliefs for removal
            if agent_belief.confidence < 0.1 {
                agents_to_remove.push(*agent_id);
            }
        }

        // Remove weak beliefs after iteration to avoid borrowing conflicts
        for agent_id in agents_to_remove {
            belief.agent_beliefs.remove(&agent_id);
        }

        // Decay world beliefs similarly
        belief.world_beliefs.retain(|_, confidence| {
            *confidence = (*confidence - 0.005 * time.delta_secs()).max(0.0);
            *confidence > 0.05
        });
    }
}

/// System that performs logical inference using semantic memory
/// Based on Collins & Quillian's Spreading Activation Theory
pub fn inference_system(
    mut query: Query<(Entity, &mut InferenceEngine, &mut Belief, &CognitiveSemanticMemory)>,
    mut belief_events: EventWriter<BeliefUpdated>,
) {
    for (entity, mut inference_engine, mut belief, semantic_memory) in query.iter_mut() {
        let mut inferences_made = 0;

        // Process inference queue
        while !inference_engine.inference_queue.is_empty()
            && inferences_made < inference_engine.max_inference_steps {
            if let Some(premise) = inference_engine.inference_queue.pop() {
                // Only process premises above threshold confidence
                if premise.confidence >= inference_engine.inference_threshold {
                    process_inference_premise(&premise, &mut belief, semantic_memory, &mut belief_events, entity);
                    inferences_made += 1;
                }
            }
        }
    }
}

/// Process a single inference premise using semantic memory
fn process_inference_premise(
    premise: &InferencePremise,
    belief: &mut Belief,
    semantic_memory: &CognitiveSemanticMemory,
    belief_events: &mut EventWriter<BeliefUpdated>,
    agent_entity: Entity,
) {
    // Look for semantic associations that might support this premise
    if let Some(associations) = semantic_memory.concept_associations.get(&premise.subject) {
        for link in associations {
            if link.target == premise.predicate {
                // Found supporting semantic knowledge
                let old_confidence = belief.world_beliefs.get(&premise.subject).copied().unwrap_or(0.0);

                // Combine premise confidence with semantic link weight
                let inference_confidence = premise.confidence * link.weight * 0.8; // Slight discount for inference
                let new_confidence = (old_confidence + inference_confidence).min(1.0);

                // Update belief if confidence increased significantly
                if new_confidence > old_confidence + 0.05 {
                    belief.world_beliefs.insert(premise.subject, new_confidence);

                    belief_events.write(BeliefUpdated {
                        agent: agent_entity,
                        subject: premise.subject,
                        old_confidence,
                        new_confidence,
                        source: PremiseSource::Inference,
                    });
                }
                break;
            }
        }
    }
}

/// Apply inheritance-based inference (Is-A relationships)
fn apply_inheritance_inference(
    premise: &InferencePremise,
    concept_link: &ConceptLink,
    belief: &mut Belief,
    belief_events: &mut EventWriter<BeliefUpdated>,
    agent_entity: Entity,
) {
    // Confidence propagates but weakens through inheritance chains
    let inherited_confidence = premise.confidence * concept_link.weight * 0.8;

    if inherited_confidence > 0.2 {
        let old_confidence = belief.world_beliefs.get(&concept_link.target).copied().unwrap_or(0.0);

        // Combine with existing belief using Bayesian update
        let new_confidence = bayesian_combine(old_confidence, inherited_confidence);
        belief.world_beliefs.insert(concept_link.target, new_confidence);

        belief_events.write(BeliefUpdated {
            agent: agent_entity,
            subject: concept_link.target,
            old_confidence,
            new_confidence,
            source: PremiseSource::Inference,
        });
    }
}

/// Apply property-based inference
fn apply_property_inference(
    premise: &InferencePremise,
    concept_link: &ConceptLink,
    belief: &mut Belief,
    belief_events: &mut EventWriter<BeliefUpdated>,
    agent_entity: Entity,
) {
    let property_confidence = premise.confidence * concept_link.weight;

    if property_confidence > 0.3 {
        let old_confidence = belief.world_beliefs.get(&concept_link.target).copied().unwrap_or(0.0);
        let new_confidence = bayesian_combine(old_confidence, property_confidence);

        belief.world_beliefs.insert(concept_link.target, new_confidence);

        belief_events.write(BeliefUpdated {
            agent: agent_entity,
            subject: concept_link.target,
            old_confidence,
            new_confidence,
            source: PremiseSource::Inference,
        });
    }
}

/// Apply provision-based inference (X provides Y)
fn apply_provision_inference(
    premise: &InferencePremise,
    concept_link: &ConceptLink,
    belief: &mut Belief,
    belief_events: &mut EventWriter<BeliefUpdated>,
    agent_entity: Entity,
) {
    // If we believe X is available, we can infer Y is obtainable
    let provision_confidence = premise.confidence * concept_link.weight * 0.9;

    if provision_confidence > 0.25 {
        let old_confidence = belief.world_beliefs.get(&concept_link.target).copied().unwrap_or(0.0);
        let new_confidence = bayesian_combine(old_confidence, provision_confidence);

        belief.world_beliefs.insert(concept_link.target, new_confidence);

        belief_events.write(BeliefUpdated {
            agent: agent_entity,
            subject: concept_link.target,
            old_confidence,
            new_confidence,
            source: PremiseSource::Inference,
        });
    }
}

/// Apply causal inference (X causes Y)
fn apply_causal_inference(
    premise: &InferencePremise,
    concept_link: &ConceptLink,
    belief: &mut Belief,
    belief_events: &mut EventWriter<BeliefUpdated>,
    agent_entity: Entity,
) {
    // Causal relationships are strong predictors
    let causal_confidence = premise.confidence * concept_link.weight;

    if causal_confidence > 0.2 {
        let old_confidence = belief.world_beliefs.get(&concept_link.target).copied().unwrap_or(0.0);
        let new_confidence = bayesian_combine(old_confidence, causal_confidence);

        belief.world_beliefs.insert(concept_link.target, new_confidence);

        belief_events.write(BeliefUpdated {
            agent: agent_entity,
            subject: concept_link.target,
            old_confidence,
            new_confidence,
            source: PremiseSource::Inference,
        });
    }
}

/// Combine two confidence values using Bayesian probability
fn bayesian_combine(prior: f32, evidence: f32) -> f32 {
    // Simplified Bayesian update: P(H|E) ∝ P(E|H) * P(H)
    // Assuming P(E|H) = evidence strength
    let combined = prior + evidence * (1.0 - prior);
    combined.min(1.0)
}

impl Default for Belief {
    fn default() -> Self {
        Self {
            world_beliefs: HashMap::new(),
            agent_beliefs: HashMap::new(),
            confidence_threshold: 0.6,
            belief_learning_rate: 0.1,
        }
    }
}

impl Default for InferenceEngine {
    fn default() -> Self {
        Self {
            max_inference_steps: 5,
            inference_queue: Vec::new(),
            inference_threshold: 0.5,
        }
    }
}

/// System that handles the decay of beliefs over time
/// Weakening beliefs that are not reinforced by evidence or inference
pub fn belief_decay_system(
    mut query: Query<(Entity, &mut Belief)>,
    _belief_events: EventWriter<BeliefUpdated>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();

    for (_entity, mut belief) in query.iter_mut() {
        // Update agent beliefs with temporal decay
        let mut agents_to_remove = Vec::new();

        for (agent_id, agent_belief) in belief.agent_beliefs.iter_mut() {
            let time_since_update = current_time - agent_belief.last_updated;

            // Beliefs decay over time without reinforcement
            let decay_rate = 0.01; // 1% decay per second
            agent_belief.confidence = (agent_belief.confidence - decay_rate * time_since_update).max(0.0);

            // Mark very weak beliefs for removal
            if agent_belief.confidence < 0.1 {
                agents_to_remove.push(*agent_id);
            }
        }

        // Remove weak beliefs after iteration
        for agent_id in agents_to_remove {
            belief.agent_beliefs.remove(&agent_id);
        }

        // Decay world beliefs similarly
        belief.world_beliefs.retain(|_, confidence| {
            *confidence = (*confidence - 0.005 * time.delta_secs()).max(0.0);
            *confidence > 0.05
        });
    }
}
