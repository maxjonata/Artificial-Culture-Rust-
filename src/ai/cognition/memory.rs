use bevy::prelude::*;
use std::collections::HashMap;

/// Cognitive Working Memory component for short-term cognitive processing
/// Based on Baddeley's Working Memory Model - the workspace for active thought
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CognitiveWorkingMemory {
    /// Current active thoughts (Entity IDs of concepts being processed)
    pub active_concepts: Vec<Entity>,
    /// Maximum capacity of working memory (Miller's 7±2 rule)
    pub capacity: u8,
    /// Attention weights for active concepts (0.0-1.0)
    pub attention_weights: HashMap<Entity, f32>,
    /// Working memory decay rate per second
    pub decay_rate: f32,
}

/// Cognitive Episodic Memory for storing temporal experiences
/// Based on Tulving's Memory Systems - autobiographical event storage
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct CognitiveEpisodicMemory {
    /// Stored episodes as weighted connections between entities
    pub episodes: Vec<CognitiveMemoryEpisode>,
    /// Maximum number of episodes to retain
    pub max_episodes: u16,
    /// Time-based decay factor for old memories
    pub temporal_decay: f32,
}

/// Individual cognitive memory episode linking entities and contexts
#[derive(Debug, Reflect, Clone)]
pub struct CognitiveMemoryEpisode {
    /// Primary entity this episode is about
    pub subject: Entity,
    /// Related entities involved in the episode
    pub related_entities: Vec<Entity>,
    /// Emotional valence of the memory (-1.0 to 1.0)
    pub emotional_valence: f32,
    /// Strength/vividness of the memory (0.0-1.0)
    pub strength: f32,
    /// When this episode occurred (simulation time)
    pub timestamp: f32,
}

/// Cognitive Semantic Memory for general knowledge about the world
/// Based on Collins & Quillian's Semantic Network Theory
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct CognitiveSemanticMemory {
    /// Learned associations between entity types and properties
    pub concept_associations: HashMap<Entity, Vec<ConceptLink>>,
    /// Learning rate for new associations
    pub learning_rate: f32,
}

/// A weighted link between concepts in semantic memory
#[derive(Debug, Reflect, Clone)]
pub struct ConceptLink {
    /// Target concept entity
    pub target: Entity,
    /// Strength of association (0.0-1.0)
    pub weight: f32,
    /// Type of relationship
    pub relation_type: RelationType,
}

/// Types of relationships between concepts
#[derive(Debug, Reflect, Clone, PartialEq)]
pub enum RelationType {
    /// X is a type of Y
    IsA,
    /// X has property Y
    HasProperty,
    /// X is located at Y
    LocatedAt,
    /// X provides Y
    Provides,
    /// X causes Y
    Causes,
}

impl Default for CognitiveWorkingMemory {
    fn default() -> Self {
        Self {
            active_concepts: Vec::new(),
            capacity: 7, // Miller's magical number 7±2
            attention_weights: HashMap::new(),
            decay_rate: 0.1, // Working memory decays quickly
        }
    }
}

/// System to manage working memory capacity and decay
/// Based on Cognitive Load Theory - limited processing capacity
pub fn working_memory_management_system(
    mut query: Query<&mut CognitiveWorkingMemory>,
    time: Res<Time>,
) {
    for mut working_memory in query.iter_mut() {
        let delta_time = time.delta_secs();
        let decay_rate = working_memory.decay_rate; // Store value to avoid borrowing issues

        // Decay attention weights
        for (_, weight) in working_memory.attention_weights.iter_mut() {
            *weight = (*weight - decay_rate * delta_time).max(0.0);
        }

        // Remove concepts with zero attention
        working_memory.attention_weights.retain(|_, weight| *weight > 0.0);

        // Create a copy of the keys before filtering to avoid borrow checker issues
        let attention_keys: std::collections::HashSet<_> = working_memory.attention_weights.keys().cloned().collect();
        working_memory.active_concepts.retain(|concept| {
            attention_keys.contains(concept)
        });

        // Enforce capacity limit - remove weakest concepts if over capacity
        let capacity = working_memory.capacity as usize;
        if working_memory.active_concepts.len() > capacity {
            // Sort by attention weight and keep only the strongest concepts
            let mut concepts_with_weights: Vec<(Entity, f32)> = working_memory.active_concepts
                .iter()
                .map(|&concept| {
                    let weight = working_memory.attention_weights.get(&concept).copied().unwrap_or(0.0);
                    (concept, weight)
                })
                .collect();

            concepts_with_weights.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

            working_memory.active_concepts = concepts_with_weights
                .into_iter()
                .take(capacity)
                .map(|(concept, _)| concept)
                .collect();
        }
    }
}

/// System to decay episodic memories over time
/// Based on Ebbinghaus Forgetting Curve - exponential memory decay
pub fn episodic_memory_decay_system(
    mut query: Query<&mut CognitiveEpisodicMemory>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();

    for mut episodic_memory in query.iter_mut() {
        let temporal_decay = episodic_memory.temporal_decay; // Store to avoid borrow issues
        let max_episodes = episodic_memory.max_episodes as usize; // Store to avoid borrow issues

        // Decay memory strength over time
        for episode in episodic_memory.episodes.iter_mut() {
            let time_since = current_time - episode.timestamp;
            // Exponential decay based on Ebbinghaus curve
            let decay_factor = (-temporal_decay * time_since).exp();
            episode.strength *= decay_factor;
        }

        // Remove very weak memories to maintain capacity
        episodic_memory.episodes.retain(|episode| episode.strength > 0.1);

        // Enforce maximum capacity - keep strongest memories
        if episodic_memory.episodes.len() > max_episodes {
            episodic_memory.episodes.sort_by(|a, b| {
                b.strength.partial_cmp(&a.strength).unwrap_or(std::cmp::Ordering::Equal)
            });
            episodic_memory.episodes.truncate(max_episodes);
        }
    }
}
