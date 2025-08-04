use bevy::prelude::*;
use std::collections::HashMap;

/// Component representing an agent's social interaction state and capabilities
/// Based on Social Psychology and Interpersonal Theory
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SocialInteractionState {
    /// Current social interactions in progress
    pub active_interactions: Vec<SocialInteraction>,
    /// Social energy/willingness to interact (0.0-1.0)
    pub social_energy: f32,
    /// Preferred interaction distance in pixels
    pub interaction_distance: f32,
    /// Maximum number of simultaneous interactions
    pub max_concurrent_interactions: u8,
    /// Time since last social interaction
    pub time_since_last_interaction: f32,
}

/// Represents an ongoing social interaction between agents
#[derive(Debug, Reflect, Clone)]
pub struct SocialInteraction {
    /// The other agent in this interaction
    pub other_agent: Entity,
    /// Type of interaction taking place
    pub interaction_type: InteractionType,
    /// How long this interaction has been ongoing
    pub duration: f32,
    /// Satisfaction level from this interaction (0.0-1.0)
    pub satisfaction: f32,
    /// Interaction progress (0.0 = just started, 1.0 = complete)
    pub progress: f32,
}

/// Types of social interactions possible between agents
/// Based on Social Exchange Theory and Interpersonal Circumplex
#[derive(Debug, Reflect, Clone, Copy, PartialEq)]
pub enum InteractionType {
    /// Simple acknowledgment/greeting
    Greeting,
    /// Casual conversation
    Conversation,
    /// Sharing information about Things or locations
    InformationSharing,
    /// Collaborative activity
    Cooperation,
    /// Giving assistance to another agent
    Helping,
    /// Emotional support interaction
    EmotionalSupport,
}

/// Component tracking social relationships with other agents
/// Based on Social Network Theory and Trust Models
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct SocialRelationshipNetwork {
    /// Relationships with other agents
    pub relationships: HashMap<Entity, Relationship>,
    /// Social memory - remembers past interactions
    pub interaction_history: Vec<SocialInteractionMemory>,
    /// Maximum relationships to actively maintain
    pub max_relationships: u16,
    /// Relationship decay rate when not maintained
    pub relationship_decay_rate: f32,
}

/// Represents a relationship with another agent
#[derive(Debug, Reflect, Clone)]
pub struct Relationship {
    /// How well this agent knows the other (0.0-1.0)
    pub familiarity: f32,
    /// Trust level in the other agent (0.0-1.0)
    pub trust: f32,
    /// Emotional affinity towards the other agent (-1.0 to 1.0)
    pub affinity: f32,
    /// Number of successful interactions
    pub positive_interactions: u16,
    /// Number of negative/failed interactions
    pub negative_interactions: u16,
    /// Last time interacted with this agent
    pub last_interaction_time: f32,
}

/// Memory of past social interactions for relationship building
#[derive(Debug, Reflect, Clone)]
pub struct SocialInteractionMemory {
    /// The other agent involved
    pub other_agent: Entity,
    /// Type of interaction that occurred
    pub interaction_type: InteractionType,
    /// How positive/negative the interaction was (-1.0 to 1.0)
    pub outcome_valence: f32,
    /// When this interaction occurred
    pub timestamp: f32,
    /// Duration of the interaction
    pub duration: f32,
}

/// Event fired when agents start interacting
#[derive(Event, Debug)]
pub struct InteractionStarted {
    pub agent_a: Entity,
    pub agent_b: Entity,
    pub interaction_type: InteractionType,
    pub initiated_by: Entity,
}

/// Event fired when interactions complete
#[derive(Event, Debug)]
pub struct InteractionCompleted {
    pub agent_a: Entity,
    pub agent_b: Entity,
    pub interaction_type: InteractionType,
    pub satisfaction_a: f32,
    pub satisfaction_b: f32,
    pub duration: f32,
}

impl Default for SocialInteractionState {
    fn default() -> Self {
        Self {
            active_interactions: Vec::new(),
            social_energy: 0.8, // Start with good social energy
            interaction_distance: 40.0, // 40 pixels interaction range
            max_concurrent_interactions: 3,
            time_since_last_interaction: 0.0,
        }
    }
}

impl Default for SocialRelationshipNetwork {
    fn default() -> Self {
        Self {
            relationships: HashMap::new(),
            interaction_history: Vec::new(),
            max_relationships: 20, // Dunbar's number approximation for small groups
            relationship_decay_rate: 0.01, // 1% decay per day without interaction
        }
    }
}

/// System managing social interactions between agents
/// Based on Proximity and Social Facilitation Theory
pub fn social_interaction_system(
    mut query: Query<(Entity, &mut SocialInteractionState, &mut SocialRelationshipNetwork, &Transform)>,
    mut interaction_started_events: EventWriter<InteractionStarted>,
    mut interaction_completed_events: EventWriter<InteractionCompleted>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();
    let current_time = time.elapsed_secs();

    // Collect all agents and their positions for proximity calculations
    let agents: Vec<(Entity, Vec3, f32)> = query
        .iter()
        .map(|(entity, social_state, _, transform)| {
            (entity, transform.translation, social_state.interaction_distance)
        })
        .collect();

    // Update existing interactions and check for new ones
    for (entity, mut social_state, mut social_network, transform) in query.iter_mut() {
        // Update time tracking
        social_state.time_since_last_interaction += delta_time;

        // Process ongoing interactions
        update_ongoing_interactions(
            &mut social_state,
            &mut social_network,
            delta_time,
            current_time,
            entity,
            &mut interaction_completed_events,
        );

        // Look for new interaction opportunities
        if social_state.active_interactions.len() < social_state.max_concurrent_interactions as usize
            && social_state.social_energy > 0.3 {
            check_for_new_interactions(
                entity,
                &mut social_state,
                &social_network,
                transform.translation,
                &agents,
                &mut interaction_started_events,
            );
        }

        // Regenerate social energy over time
        if social_state.time_since_last_interaction > 10.0 {
            social_state.social_energy = (social_state.social_energy + 0.1 * delta_time).min(1.0);
        }
    }
}

/// Update all ongoing interactions for an agent
fn update_ongoing_interactions(
    social_state: &mut SocialInteractionState,
    social_network: &mut SocialRelationshipNetwork,
    delta_time: f32,
    current_time: f32,
    agent_entity: Entity,
    interaction_completed_events: &mut EventWriter<InteractionCompleted>,
) {
    let mut completed_interactions = Vec::new();

    for (index, interaction) in social_state.active_interactions.iter_mut().enumerate() {
        interaction.duration += delta_time;

        // Update interaction progress based on type
        let progress_rate = match interaction.interaction_type {
            InteractionType::Greeting => 2.0,        // Quick greeting
            InteractionType::Conversation => 0.5,    // Longer conversation
            InteractionType::InformationSharing => 1.0,
            InteractionType::Cooperation => 0.3,     // Takes time to cooperate
            InteractionType::Helping => 0.8,
            InteractionType::EmotionalSupport => 0.4,
        };

        interaction.progress = (interaction.progress + progress_rate * delta_time).min(1.0);

        // Calculate satisfaction based on interaction quality
        interaction.satisfaction = calculate_interaction_satisfaction(
            interaction.interaction_type,
            interaction.duration,
            social_network.relationships.get(&interaction.other_agent),
        );

        // Check if interaction is complete
        if interaction.progress >= 1.0 {
            completed_interactions.push((index, interaction.clone()));
        }
    }

    // Process completed interactions
    for (index, completed_interaction) in completed_interactions.into_iter().rev() {
        social_state.active_interactions.remove(index);

        // Update relationship based on interaction outcome
        update_relationship_from_interaction(
            social_network,
            &completed_interaction,
            current_time,
        );

        // Consume social energy
        social_state.social_energy = (social_state.social_energy - 0.1).max(0.0);
        social_state.time_since_last_interaction = 0.0;

        // Record interaction in memory
        let interaction_memory = SocialInteractionMemory {
            other_agent: completed_interaction.other_agent,
            interaction_type: completed_interaction.interaction_type,
            outcome_valence: completed_interaction.satisfaction,
            timestamp: current_time,
            duration: completed_interaction.duration,
        };

        social_network.interaction_history.push(interaction_memory);

        // Send completion event
        interaction_completed_events.write(InteractionCompleted {
            agent_a: agent_entity,
            agent_b: completed_interaction.other_agent,
            interaction_type: completed_interaction.interaction_type,
            satisfaction_a: completed_interaction.satisfaction,
            satisfaction_b: completed_interaction.satisfaction, // Simplified - assume mutual
            duration: completed_interaction.duration,
        });
    }
}

/// Check for opportunities to start new interactions
fn check_for_new_interactions(
    agent_entity: Entity,
    social_state: &mut SocialInteractionState,
    social_network: &SocialRelationshipNetwork,
    agent_position: Vec3,
    all_agents: &[(Entity, Vec3, f32)],
    interaction_started_events: &mut EventWriter<InteractionStarted>,
) {
    for (other_entity, other_position, _) in all_agents {
        if *other_entity == agent_entity {
            continue;
        }

        // Check if already interacting with this agent
        if social_state.active_interactions.iter()
            .any(|interaction| interaction.other_agent == *other_entity) {
            continue;
        }

        // Check proximity
        let distance = agent_position.distance(*other_position);
        if distance <= social_state.interaction_distance {
            // Determine interaction type based on relationship and random factors
            let interaction_type = determine_interaction_type(
                social_network.relationships.get(other_entity),
                social_state.social_energy,
            );

            if let Some(interaction_type) = interaction_type {
                // Start new interaction
                let new_interaction = SocialInteraction {
                    other_agent: *other_entity,
                    interaction_type,
                    duration: 0.0,
                    satisfaction: 0.5, // Neutral starting satisfaction
                    progress: 0.0,
                };

                social_state.active_interactions.push(new_interaction);

                interaction_started_events.write(InteractionStarted {
                    agent_a: agent_entity,
                    agent_b: *other_entity,
                    interaction_type,
                    initiated_by: agent_entity,
                });

                break; // Only start one new interaction per frame
            }
        }
    }
}

/// Calculate satisfaction from an interaction based on various factors
fn calculate_interaction_satisfaction(
    interaction_type: InteractionType,
    duration: f32,
    relationship: Option<&Relationship>,
) -> f32 {
    let mut satisfaction = 0.5; // Base satisfaction

    // Relationship quality affects satisfaction
    if let Some(rel) = relationship {
        satisfaction += rel.trust * 0.3 + rel.affinity * 0.2;
    }

    // Interaction type affects satisfaction
    satisfaction += match interaction_type {
        InteractionType::Greeting => 0.1,
        InteractionType::Conversation => 0.2,
        InteractionType::InformationSharing => 0.3,
        InteractionType::Cooperation => 0.4,
        InteractionType::Helping => 0.5,
        InteractionType::EmotionalSupport => 0.4,
    };

    // Optimal duration varies by interaction type
    let optimal_duration = match interaction_type {
        InteractionType::Greeting => 1.0,
        InteractionType::Conversation => 10.0,
        InteractionType::InformationSharing => 5.0,
        InteractionType::Cooperation => 15.0,
        InteractionType::Helping => 8.0,
        InteractionType::EmotionalSupport => 12.0,
    };

    // Satisfaction decreases if interaction goes too long or too short
    let duration_factor = 1.0 - (duration - optimal_duration).abs() / optimal_duration;
    satisfaction *= duration_factor.max(0.5);

    satisfaction.clamp(0.0, 1.0)
}

/// Determine what type of interaction to start based on context
fn determine_interaction_type(
    relationship: Option<&Relationship>,
    social_energy: f32,
) -> Option<InteractionType> {
    if social_energy < 0.2 {
        return None; // Too tired to interact
    }

    match relationship {
        Some(rel) if rel.familiarity > 0.7 => {
            // Close relationship - more intimate interactions possible
            if social_energy > 0.8 {
                Some(InteractionType::Conversation)
            } else {
                Some(InteractionType::Greeting)
            }
        }
        Some(_) => {
            // Known but not close - moderate interactions
            Some(InteractionType::InformationSharing)
        }
        None => {
            // Stranger - start with greeting
            Some(InteractionType::Greeting)
        }
    }
}

/// Update relationship data based on interaction outcome
fn update_relationship_from_interaction(
    social_network: &mut SocialRelationshipNetwork,
    interaction: &SocialInteraction,
    current_time: f32,
) {
    let relationship = social_network.relationships
        .entry(interaction.other_agent)
        .or_insert(Relationship {
            familiarity: 0.0,
            trust: 0.5,
            affinity: 0.0,
            positive_interactions: 0,
            negative_interactions: 0,
            last_interaction_time: current_time,
        });

    // Update familiarity
    relationship.familiarity = (relationship.familiarity + 0.1).min(1.0);

    // Update based on satisfaction
    if interaction.satisfaction > 0.6 {
        relationship.positive_interactions += 1;
        relationship.trust = (relationship.trust + 0.05).min(1.0);
        relationship.affinity = (relationship.affinity + 0.1).min(1.0);
    } else if interaction.satisfaction < 0.4 {
        relationship.negative_interactions += 1;
        relationship.trust = (relationship.trust - 0.1).max(0.0);
        relationship.affinity = (relationship.affinity - 0.05).max(-1.0);
    }

    relationship.last_interaction_time = current_time;
}
