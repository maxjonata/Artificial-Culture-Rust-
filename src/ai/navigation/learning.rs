use crate::ai::navigation::mapping::{CognitiveMap, SpatialNavigationNetwork};
use bevy::prelude::*;
use std::collections::HashMap;

/// Component managing synaptic plasticity for spatial learning
/// Based on Hebbian Learning and Long-Term Potentiation (LTP) research
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SynapticPlasticity {
    /// Learning rate for spatial associations
    pub learning_rate: f32,
    /// Decay rate for unused connections
    pub decay_rate: f32,
    /// Synaptic weights between spatial concepts
    pub spatial_weights: HashMap<SynapticConnection, f32>,
    /// Recent activity traces for Hebbian learning
    pub activity_traces: HashMap<Entity, f32>,
    /// Long-term potentiation threshold
    pub ltp_threshold: f32,
}

/// Represents a synaptic connection between spatial concepts
#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SynapticConnection {
    /// Pre-synaptic neuron (source concept)
    pub pre_synaptic: Entity,
    /// Post-synaptic neuron (target concept)
    pub post_synaptic: Entity,
}

/// Component for path learning and optimization
/// Based on Reinforcement Learning and Temporal Difference Learning
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PathLearning {
    /// Learned path values (how good different paths are)
    pub path_values: HashMap<PathSegment, f32>,
    /// Recent path experiences for learning
    pub path_memory: Vec<PathExperience>,
    /// Learning rate for path optimization
    pub path_learning_rate: f32,
    /// Discount factor for future rewards
    pub discount_factor: f32,
}

/// Represents a segment of a learned path
#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PathSegment {
    /// Starting landmark
    pub from_landmark: Entity,
    /// Destination landmark
    pub to_landmark: Entity,
}

/// Experience from traversing a path
#[derive(Debug, Reflect, Clone)]
pub struct PathExperience {
    /// Path segment that was traversed
    pub segment: PathSegment,
    /// Time taken to traverse
    pub traversal_time: f32,
    /// Success/failure of reaching destination
    pub success: bool,
    /// Reward received for this path
    pub reward: f32,
    /// When this experience occurred
    pub timestamp: f32,
}

/// Component for adaptive navigation behavior
/// Implements exploration vs exploitation tradeoffs
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AdaptiveBehavior {
    /// Exploration tendency (0.0 = pure exploitation, 1.0 = pure exploration)
    pub exploration_rate: f32,
    /// Confidence in current navigation strategy
    pub strategy_confidence: f32,
    /// Recent navigation success rate
    pub success_rate: f32,
    /// Number of recent navigation attempts
    pub recent_attempts: u16,
    /// Number of recent successes
    pub recent_successes: u16,
}

/// Event fired when synaptic learning occurs
#[derive(Event, Debug)]
pub struct SynapticLearningEvent {
    pub agent: Entity,
    pub connection: SynapticConnection,
    pub old_weight: f32,
    pub new_weight: f32,
    pub learning_type: LearningType,
}

/// Types of learning that can occur
#[derive(Debug, Clone)]
pub enum LearningType {
    /// Hebbian learning - "neurons that fire together, wire together"
    Hebbian,
    /// Long-term potentiation
    LongTermPotentiation,
    /// Synaptic decay
    Decay,
    /// Reinforcement-based learning
    Reinforcement { reward: f32 },
}

impl Default for SynapticPlasticity {
    fn default() -> Self {
        Self {
            learning_rate: 0.01,
            decay_rate: 0.001,
            spatial_weights: HashMap::new(),
            activity_traces: HashMap::new(),
            ltp_threshold: 0.8,
        }
    }
}

impl Default for PathLearning {
    fn default() -> Self {
        Self {
            path_values: HashMap::new(),
            path_memory: Vec::new(),
            path_learning_rate: 0.1,
            discount_factor: 0.9,
        }
    }
}

impl Default for AdaptiveBehavior {
    fn default() -> Self {
        Self {
            exploration_rate: 0.2, // 20% exploration by default
            strategy_confidence: 0.5,
            success_rate: 0.5,
            recent_attempts: 0,
            recent_successes: 0,
        }
    }
}

/// Core synaptic plasticity system implementing Hebbian learning
/// Based on "neurons that fire together, wire together" principle
pub fn synaptic_plasticity_system(
    mut query: Query<(Entity, &mut SynapticPlasticity, &CognitiveMap, &SpatialNavigationNetwork)>,
    mut learning_events: EventWriter<SynapticLearningEvent>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (entity, mut plasticity, cognitive_map, nav_network) in query.iter_mut() {
        // Update activity traces based on current neural activity
        update_activity_traces(&mut plasticity, &nav_network, delta_time);

        // Apply Hebbian learning rule
        apply_hebbian_learning(
            &mut plasticity,
            &cognitive_map,
            entity,
            &mut learning_events,
        );

        // Apply synaptic decay
        apply_synaptic_decay(&mut plasticity, delta_time, entity, &mut learning_events);

        // Check for long-term potentiation
        check_long_term_potentiation(
            &mut plasticity,
            entity,
            &mut learning_events,
        );
    }
}

/// Update activity traces for all spatial concepts
fn update_activity_traces(
    plasticity: &mut SynapticPlasticity,
    nav_network: &SpatialNavigationNetwork,
    delta_time: f32,
) {
    // Decay existing traces
    for (_, trace) in plasticity.activity_traces.iter_mut() {
        *trace *= 0.9_f32.powf(delta_time); // Exponential decay
    }

    // Update traces based on current place cell activity
    for place_cell in &nav_network.place_cells {
        if place_cell.current_activation > 0.1 {
            // Create a synthetic entity ID for this place cell
            let place_entity = Entity::from_raw(
                (place_cell.center.x as u32) * 1000 + (place_cell.center.y as u32)
            );

            plasticity.activity_traces.insert(place_entity, place_cell.current_activation);
        }
    }

    // Remove very weak traces
    plasticity.activity_traces.retain(|_, trace| *trace > 0.01);
}

/// Apply Hebbian learning rule to strengthen co-active connections
fn apply_hebbian_learning(
    plasticity: &mut SynapticPlasticity,
    _cognitive_map: &CognitiveMap,
    agent_entity: Entity,
    learning_events: &mut EventWriter<SynapticLearningEvent>,
) {
    // Find pairs of active concepts for Hebbian learning
    let active_concepts: Vec<(Entity, f32)> = plasticity.activity_traces
        .iter()
        .filter(|(_, activity)| **activity > 0.2)
        .map(|(&entity, &activity)| (entity, activity))
        .collect();

    // Apply Hebbian rule: strengthen connections between co-active concepts
    for i in 0..active_concepts.len() {
        for j in (i + 1)..active_concepts.len() {
            let (concept_a, activity_a) = active_concepts[i];
            let (concept_b, activity_b) = active_concepts[j];

            let connection = SynapticConnection {
                pre_synaptic: concept_a,
                post_synaptic: concept_b,
            };

            // Calculate Hebbian learning: Δw = η * activity_a * activity_b
            let learning_strength = plasticity.learning_rate * activity_a * activity_b;

            let old_weight = plasticity.spatial_weights.get(&connection).copied().unwrap_or(0.0);
            let new_weight = (old_weight + learning_strength).min(1.0);

            if (new_weight - old_weight).abs() > 0.001 {
                plasticity.spatial_weights.insert(connection, new_weight);

                learning_events.write(SynapticLearningEvent {
                    agent: agent_entity,
                    connection,
                    old_weight,
                    new_weight,
                    learning_type: LearningType::Hebbian,
                });
            }
        }
    }
}

/// Apply synaptic decay to unused connections
fn apply_synaptic_decay(
    plasticity: &mut SynapticPlasticity,
    delta_time: f32,
    agent_entity: Entity,
    learning_events: &mut EventWriter<SynapticLearningEvent>,
) {
    let decay_amount = plasticity.decay_rate * delta_time;
    let mut connections_to_remove = Vec::new();

    for (connection, weight) in plasticity.spatial_weights.iter_mut() {
        let old_weight = *weight;
        *weight = (*weight - decay_amount).max(0.0);

        if *weight == 0.0 {
            connections_to_remove.push(*connection);
        } else if (old_weight - *weight).abs() > 0.001 {
            learning_events.write(SynapticLearningEvent {
                agent: agent_entity,
                connection: *connection,
                old_weight,
                new_weight: *weight,
                learning_type: LearningType::Decay,
            });
        }
    }

    // Remove connections that have decayed to zero
    for connection in connections_to_remove {
        plasticity.spatial_weights.remove(&connection);
    }
}

/// Check for long-term potentiation when activity exceeds threshold
fn check_long_term_potentiation(
    plasticity: &mut SynapticPlasticity,
    agent_entity: Entity,
    learning_events: &mut EventWriter<SynapticLearningEvent>,
) {
    // LTP occurs when pre-synaptic and post-synaptic activity are both high
    for (connection, weight) in plasticity.spatial_weights.iter_mut() {
        let pre_activity = plasticity.activity_traces.get(&connection.pre_synaptic).copied().unwrap_or(0.0);
        let post_activity = plasticity.activity_traces.get(&connection.post_synaptic).copied().unwrap_or(0.0);

        // LTP threshold: both neurons must be highly active
        if pre_activity > plasticity.ltp_threshold && post_activity > plasticity.ltp_threshold {
            let old_weight = *weight;
            let ltp_boost = 0.1; // Strong potentiation
            *weight = (*weight + ltp_boost).min(1.0);

            if *weight != old_weight {
                learning_events.write(SynapticLearningEvent {
                    agent: agent_entity,
                    connection: *connection,
                    old_weight,
                    new_weight: *weight,
                    learning_type: LearningType::LongTermPotentiation,
                });
            }
        }
    }
}

/// System for learning optimal paths through reinforcement learning
/// Based on Temporal Difference Learning and Q-Learning
pub fn path_learning_system(
    mut query: Query<(Entity, &mut PathLearning, &mut AdaptiveBehavior, &CognitiveMap)>,
    mut learning_events: EventWriter<SynapticLearningEvent>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();

    for (entity, mut path_learning, mut adaptive_behavior, _cognitive_map) in query.iter_mut() {
        // Update path values based on recent experiences
        update_path_values(&mut path_learning, current_time, entity, &mut learning_events);

        // Update exploration/exploitation balance
        update_adaptive_behavior(&mut adaptive_behavior, &path_learning);

        // Clean up old experiences
        path_learning.path_memory.retain(|exp| current_time - exp.timestamp < 60.0);
    }
}

/// Update path values using temporal difference learning
fn update_path_values(
    path_learning: &mut PathLearning,
    current_time: f32,
    agent_entity: Entity,
    learning_events: &mut EventWriter<SynapticLearningEvent>,
) {
    // Process recent experiences to update path values
    for experience in &path_learning.path_memory {
        if current_time - experience.timestamp < 5.0 { // Only recent experiences
            let old_value = path_learning.path_values.get(&experience.segment).copied().unwrap_or(0.0);

            // Temporal difference learning: V(s) = V(s) + α[r + γV(s') - V(s)]
            let td_error = experience.reward - old_value;
            let new_value = old_value + path_learning.path_learning_rate * td_error;

            path_learning.path_values.insert(experience.segment, new_value);

            if (new_value - old_value).abs() > 0.01 {
                // Create a synthetic connection for this path learning
                let connection = SynapticConnection {
                    pre_synaptic: experience.segment.from_landmark,
                    post_synaptic: experience.segment.to_landmark,
                };

                learning_events.write(SynapticLearningEvent {
                    agent: agent_entity,
                    connection,
                    old_weight: old_value,
                    new_weight: new_value,
                    learning_type: LearningType::Reinforcement { reward: experience.reward },
                });
            }
        }
    }
}

/// Update exploration/exploitation balance based on recent performance
fn update_adaptive_behavior(
    adaptive_behavior: &mut AdaptiveBehavior,
    _path_learning: &PathLearning,
) {
    // Calculate recent success rate
    if adaptive_behavior.recent_attempts > 0 {
        adaptive_behavior.success_rate =
            adaptive_behavior.recent_successes as f32 / adaptive_behavior.recent_attempts as f32;
    }

    // Adjust exploration rate based on performance
    if adaptive_behavior.success_rate > 0.8 {
        // High success rate - reduce exploration
        adaptive_behavior.exploration_rate = (adaptive_behavior.exploration_rate * 0.95).max(0.05);
        adaptive_behavior.strategy_confidence = (adaptive_behavior.strategy_confidence + 0.01).min(1.0);
    } else if adaptive_behavior.success_rate < 0.4 {
        // Low success rate - increase exploration
        adaptive_behavior.exploration_rate = (adaptive_behavior.exploration_rate * 1.05).min(0.5);
        adaptive_behavior.strategy_confidence = (adaptive_behavior.strategy_confidence - 0.01).max(0.1);
    }

    // Reset counters periodically
    if adaptive_behavior.recent_attempts > 20 {
        adaptive_behavior.recent_attempts = 0;
        adaptive_behavior.recent_successes = 0;
    }
}

/// System to consolidate learning during rest periods
/// Based on sleep-dependent memory consolidation research
pub fn memory_consolidation_system(
    mut query: Query<(&mut SynapticPlasticity, &mut PathLearning, &mut CognitiveMap)>,
    _time: Res<Time>,
) {
    // This system would typically run during "rest" periods
    // For simplicity, we'll run it occasionally
    if rand::random::<f32>() > 0.01 { // 1% chance per frame
        return;
    }

    for (mut plasticity, mut path_learning, mut cognitive_map) in query.iter_mut() {
        // Consolidate strong synaptic connections
        consolidate_synaptic_connections(&mut plasticity);

        // Consolidate successful path memories
        consolidate_path_memories(&mut path_learning);

        // Update cognitive map based on learned associations
        update_map_from_learning(&mut cognitive_map, &plasticity);
    }
}

/// Strengthen important synaptic connections during consolidation
fn consolidate_synaptic_connections(plasticity: &mut SynapticPlasticity) {
    // Identify strong connections for consolidation
    let strong_connections: Vec<(SynapticConnection, f32)> = plasticity.spatial_weights
        .iter()
        .filter(|(_, weight)| **weight > 0.7)
        .map(|(&conn, &weight)| (conn, weight))
        .collect();

    // Strengthen these connections slightly
    for (connection, _) in strong_connections {
        if let Some(weight) = plasticity.spatial_weights.get_mut(&connection) {
            *weight = (*weight + 0.01).min(1.0);
        }
    }
}

/// Consolidate successful path memories
fn consolidate_path_memories(path_learning: &mut PathLearning) {
    // Identify highly valued paths
    let good_paths: Vec<(PathSegment, f32)> = path_learning.path_values
        .iter()
        .filter(|(_, value)| **value > 0.7)
        .map(|(&segment, &value)| (segment, value))
        .collect();

    // Slightly boost values of good paths during consolidation
    for (segment, _) in good_paths {
        if let Some(value) = path_learning.path_values.get_mut(&segment) {
            *value = (*value + 0.02).min(1.0);
        }
    }
}

/// Update cognitive map based on learned synaptic associations
fn update_map_from_learning(
    cognitive_map: &mut CognitiveMap,
    plasticity: &SynapticPlasticity,
) {
    // Use strong synaptic connections to refine spatial relationships
    for spatial_connection in cognitive_map.spatial_connections.iter_mut() {
        let synaptic_conn = SynapticConnection {
            pre_synaptic: spatial_connection.landmark_a,
            post_synaptic: spatial_connection.landmark_b,
        };

        if let Some(&synaptic_weight) = plasticity.spatial_weights.get(&synaptic_conn) {
            // Use synaptic strength to influence confidence in spatial connection
            spatial_connection.confidence = (spatial_connection.confidence + synaptic_weight * 0.01).min(1.0);
        }
    }
}
