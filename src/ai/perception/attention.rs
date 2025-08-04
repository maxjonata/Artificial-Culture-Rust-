use crate::ai::cognition::memory::CognitiveWorkingMemory;
use crate::ai::physiology::needs::BasicNeeds;
use bevy::prelude::*;
use std::collections::HashMap;

/// Component managing selective attention and cognitive focus
/// Based on Broadbent's Filter Theory and Kahneman's Attention Theory
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Attention {
    /// Current focus targets with attention weights (0.0-1.0)
    pub focus_targets: HashMap<Entity, f32>,
    /// Maximum attention capacity (cognitive load limit)
    pub attention_capacity: f32,
    /// Current total attention being used
    pub current_attention_load: f32,
    /// Attention shift rate (how quickly focus can change)
    pub shift_rate: f32,
    /// Minimum attention threshold to maintain focus
    pub focus_threshold: f32,
}

/// Component tracking what entities are currently perceived
/// Links perception to attention - what we notice vs what we attend to
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PerceivedEntities {
    /// All entities currently in perceptual range
    pub visible_entities: Vec<Entity>,
    /// Entities that have captured attention this frame
    pub attended_entities: Vec<Entity>,
    /// Salience scores for visible entities (how attention-grabbing they are)
    pub salience_scores: HashMap<Entity, f32>,
    /// Maximum entities that can be tracked simultaneously
    pub max_tracked_entities: u8,
}

/// Event fired when attention shifts to a new target
#[derive(Event, Debug)]
pub struct AttentionShifted {
    pub agent: Entity,
    pub previous_target: Option<Entity>,
    pub new_target: Entity,
    pub attention_weight: f32,
}

/// Event fired when an entity enters/exits perceptual range
#[derive(Event, Debug)]
pub struct PerceptionChanged {
    pub observer: Entity,
    pub perceived_entity: Entity,
    pub entered: bool, // true if entered range, false if exited
    pub salience: f32,
}

impl Default for Attention {
    fn default() -> Self {
        Self {
            focus_targets: HashMap::new(),
            attention_capacity: 1.0, // Normalized attention capacity
            current_attention_load: 0.0,
            shift_rate: 2.0, // Can shift attention 2 times per second
            focus_threshold: 0.1,
        }
    }
}

impl Default for PerceivedEntities {
    fn default() -> Self {
        Self {
            visible_entities: Vec::new(),
            attended_entities: Vec::new(),
            salience_scores: HashMap::new(),
            max_tracked_entities: 10, // Cognitive limit on tracked entities
        }
    }
}

/// Core attention management system
/// Based on Cognitive Load Theory and Selective Attention Models
pub fn attention_system(
    mut query: Query<(Entity, &mut Attention, &mut PerceivedEntities, &BasicNeeds, &CognitiveWorkingMemory)>,
    mut attention_events: EventWriter<AttentionShifted>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (entity, mut attention, mut perceived, needs, working_memory) in query.iter_mut() {
        // Calculate salience scores for all perceived entities
        update_salience_scores(&mut perceived, needs, working_memory);

        // Determine what deserves attention based on salience and current focus
        let attention_candidates = select_attention_candidates(&perceived, &attention);

        // Update attention allocation
        update_attention_allocation(
            &mut attention,
            &attention_candidates,
            delta_time,
            entity,
            &mut attention_events,
        );

        // Update the list of attended entities
        update_attended_entities(&mut perceived, &attention);

        // Enforce cognitive limits
        enforce_attention_limits(&mut attention, &mut perceived);
    }
}

/// Calculate salience scores for perceived entities
/// Based on need-driven attention and cognitive relevance
fn update_salience_scores(
    perceived: &mut PerceivedEntities,
    needs: &BasicNeeds,
    _working_memory: &CognitiveWorkingMemory,
) {
    perceived.salience_scores.clear();

    for entity in &perceived.visible_entities {
        let salience = calculate_entity_salience(*entity, needs);
        perceived.salience_scores.insert(*entity, salience);
    }
}

/// Calculate how salient (attention-grabbing) an entity is
/// Based on current needs and entity properties
fn calculate_entity_salience(_entity: Entity, needs: &BasicNeeds) -> f32 {
    // Base salience - all entities have some minimal attention value
    let mut salience = 0.1;

    // Need-driven salience boosts
    // Note: In a full implementation, this would query the entity's components
    // to determine what type of Thing it is and boost salience accordingly

    // Thirst drives attention to water sources
    if needs.thirst > 0.5 {
        salience += needs.thirst * 0.8; // High priority for water
    }

    // Hunger drives attention to food sources
    if needs.hunger > 0.4 {
        salience += needs.hunger * 0.6;
    }

    // Rest drives attention to rest areas
    if needs.rest > 0.6 {
        salience += needs.rest * 0.5;
    }

    // Social needs drive attention to other agents
    if needs.social > 0.3 {
        salience += needs.social * 0.4;
    }

    // Movement and novelty can boost salience
    // (This would be enhanced with velocity/animation components)

    salience.min(1.0)
}

/// Select which entities deserve attention allocation
fn select_attention_candidates(
    perceived: &PerceivedEntities,
    attention: &Attention,
) -> Vec<(Entity, f32)> {
    let mut candidates: Vec<(Entity, f32)> = perceived
        .salience_scores
        .iter()
        .map(|(&entity, &salience)| (entity, salience))
        .collect();

    // Sort by salience (highest first)
    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Consider current attention state - ongoing focus has some persistence
    for (entity, salience) in candidates.iter_mut() {
        if let Some(current_attention) = attention.focus_targets.get(entity) {
            // Boost salience for entities we're already attending to (attention persistence)
            *salience += current_attention * 0.3;
        }
    }

    candidates
}

/// Update attention allocation based on salience and capacity constraints
fn update_attention_allocation(
    attention: &mut Attention,
    candidates: &[(Entity, f32)],
    delta_time: f32,
    agent_entity: Entity,
    attention_events: &mut EventWriter<AttentionShifted>,
) {
    let max_attention_change = attention.shift_rate * delta_time;

    // Decay existing attention weights
    for (_, weight) in attention.focus_targets.iter_mut() {
        *weight = (*weight - 0.5 * delta_time).max(0.0);
    }

    // Remove entities below focus threshold
    let entities_to_remove: Vec<Entity> = attention.focus_targets
        .iter()
        .filter(|(_, weight)| **weight < attention.focus_threshold)
        .map(|(&entity, _)| entity)
        .collect();

    for entity in entities_to_remove {
        attention.focus_targets.remove(&entity);
    }

    // Allocate attention to high-salience entities
    let mut remaining_capacity = attention.attention_capacity -
        attention.focus_targets.values().sum::<f32>();

    for (entity, salience) in candidates {
        if remaining_capacity <= 0.0 {
            break;
        }

        let desired_attention = (*salience).min(remaining_capacity);

        if desired_attention > attention.focus_threshold {
            let old_attention = attention.focus_targets.get(entity).copied().unwrap_or(0.0);
            let attention_change = (desired_attention - old_attention).min(max_attention_change);
            let new_attention = old_attention + attention_change;

            if (new_attention - old_attention).abs() > 0.05 {
                attention_events.write(AttentionShifted {
                    agent: agent_entity,
                    previous_target: if old_attention > 0.0 { Some(*entity) } else { None },
                    new_target: *entity,
                    attention_weight: new_attention,
                });
            }

            attention.focus_targets.insert(*entity, new_attention);
            remaining_capacity -= attention_change;
        }
    }

    // Update current attention load
    attention.current_attention_load = attention.focus_targets.values().sum();
}

/// Update the list of entities currently being attended to
fn update_attended_entities(perceived: &mut PerceivedEntities, attention: &Attention) {
    perceived.attended_entities.clear();

    for (&entity, &weight) in &attention.focus_targets {
        if weight >= attention.focus_threshold {
            perceived.attended_entities.push(entity);
        }
    }
}

/// Enforce cognitive limits on attention and perception
fn enforce_attention_limits(attention: &mut Attention, perceived: &mut PerceivedEntities) {
    // Limit the number of tracked entities
    if perceived.visible_entities.len() > perceived.max_tracked_entities as usize {
        // Keep the most salient entities
        let mut entities_with_salience: Vec<(Entity, f32)> = perceived.visible_entities
            .iter()
            .map(|&entity| {
                let salience = perceived.salience_scores.get(&entity).copied().unwrap_or(0.0);
                (entity, salience)
            })
            .collect();

        entities_with_salience.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        perceived.visible_entities = entities_with_salience
            .into_iter()
            .take(perceived.max_tracked_entities as usize)
            .map(|(entity, _)| entity)
            .collect();
    }

    // Ensure attention doesn't exceed capacity
    if attention.current_attention_load > attention.attention_capacity {
        let scale_factor = attention.attention_capacity / attention.current_attention_load;
        for (_, weight) in attention.focus_targets.iter_mut() {
            *weight *= scale_factor;
        }
        attention.current_attention_load = attention.attention_capacity;
    }
}
