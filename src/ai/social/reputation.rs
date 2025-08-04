use crate::ai::social::interaction::{InteractionCompleted, SocialRelationshipNetwork};
use bevy::prelude::*;
use std::collections::HashMap;

/// Component tracking an agent's reputation in the community
/// Based on Social Identity Theory and Reputation Systems
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Reputation {
    /// Overall reputation score in the community (-1.0 to 1.0)
    pub overall_score: f32,
    /// Reputation in specific domains
    pub domain_scores: HashMap<ReputationDomain, f32>,
    /// Number of agents who know this agent
    pub visibility: u16,
    /// Recent reputation trend (positive/negative change)
    pub trend: f32,
    /// Time-based decay rate for reputation
    pub decay_rate: f32,
}

/// Different domains where reputation can be earned
/// Based on Social Exchange Theory and Community Psychology
#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReputationDomain {
    /// Trustworthiness in sharing accurate information
    Trustworthiness,
    /// Helpfulness to other agents
    Helpfulness,
    /// Cooperation in group activities
    Cooperation,
    /// Social skills and likability
    Sociability,
    /// Knowledge about Thing locations
    KnowledgeSharing,
}

/// Component for community-wide reputation tracking
/// Implements gossip and reputation propagation
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct CommunityMemory {
    /// Collective memory of agent reputations
    pub agent_reputations: HashMap<Entity, CommunityReputation>,
    /// Gossip propagation rate
    pub gossip_rate: f32,
    /// Maximum distance for reputation propagation
    pub gossip_range: f32,
}

/// Community's view of an agent's reputation
#[derive(Debug, Reflect, Clone)]
pub struct CommunityReputation {
    /// Aggregated reputation scores from multiple sources
    pub aggregated_score: f32,
    /// Number of sources contributing to this reputation
    pub source_count: u16,
    /// Confidence in this reputation assessment
    pub confidence: f32,
    /// Last time this reputation was updated
    pub last_updated: f32,
}

/// Event fired when reputation changes significantly
#[derive(Event, Debug)]
pub struct ReputationChanged {
    pub agent: Entity,
    pub domain: ReputationDomain,
    pub old_score: f32,
    pub new_score: f32,
    pub change_reason: ReputationChangeReason,
}

/// Reasons for reputation changes
#[derive(Debug, Clone)]
pub enum ReputationChangeReason {
    /// Direct interaction with another agent
    DirectInteraction { other_agent: Entity, satisfaction: f32 },
    /// Gossip/hearsay from other agents
    Gossip { source_agent: Entity },
    /// Observed behavior
    ObservedBehavior { behavior_type: String },
    /// Time-based decay
    Decay,
}

impl Default for Reputation {
    fn default() -> Self {
        let mut domain_scores = HashMap::new();
        domain_scores.insert(ReputationDomain::Trustworthiness, 0.5);
        domain_scores.insert(ReputationDomain::Helpfulness, 0.5);
        domain_scores.insert(ReputationDomain::Cooperation, 0.5);
        domain_scores.insert(ReputationDomain::Sociability, 0.5);
        domain_scores.insert(ReputationDomain::KnowledgeSharing, 0.5);

        Self {
            overall_score: 0.5, // Neutral starting reputation
            domain_scores,
            visibility: 0,
            trend: 0.0,
            decay_rate: 0.001, // Very slow decay - reputation persists
        }
    }
}

/// System that updates reputation based on completed interactions
/// Based on Direct Experience and Social Learning Theory
pub fn reputation_update_system(
    mut reputation_query: Query<(Entity, &mut Reputation)>,
    mut community_query: Query<&mut CommunityMemory>,
    mut interaction_events: EventReader<InteractionCompleted>,
    mut reputation_events: EventWriter<ReputationChanged>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();

    // Process completed interactions to update reputations
    for interaction in interaction_events.read() {
        update_reputation_from_interaction(
            &mut reputation_query,
            &mut community_query,
            interaction,
            current_time,
            &mut reputation_events,
        );
    }

    // Apply time-based reputation decay
    apply_reputation_decay(&mut reputation_query, time.delta_secs(), &mut reputation_events);
}

/// Update reputation based on a completed interaction
fn update_reputation_from_interaction(
    reputation_query: &mut Query<(Entity, &mut Reputation)>,
    community_query: &mut Query<&mut CommunityMemory>,
    interaction: &InteractionCompleted,
    current_time: f32,
    reputation_events: &mut EventWriter<ReputationChanged>,
) {
    // Update reputation for both agents based on interaction satisfaction
    update_agent_reputation_from_satisfaction(
        reputation_query,
        interaction.agent_a,
        interaction.satisfaction_a,
        interaction.agent_b,
        current_time,
        reputation_events,
    );

    update_agent_reputation_from_satisfaction(
        reputation_query,
        interaction.agent_b,
        interaction.satisfaction_b,
        interaction.agent_a,
        current_time,
        reputation_events,
    );

    // Update community memory
    for mut community_memory in community_query.iter_mut() {
        update_community_reputation(
            &mut community_memory,
            interaction.agent_a,
            interaction.satisfaction_a,
            current_time,
        );
        update_community_reputation(
            &mut community_memory,
            interaction.agent_b,
            interaction.satisfaction_b,
            current_time,
        );
    }
}

/// Update an individual agent's reputation based on interaction satisfaction
fn update_agent_reputation_from_satisfaction(
    reputation_query: &mut Query<(Entity, &mut Reputation)>,
    target_agent: Entity,
    satisfaction: f32,
    interaction_partner: Entity,
    _current_time: f32,
    reputation_events: &mut EventWriter<ReputationChanged>,
) {
    if let Ok((_, mut reputation)) = reputation_query.get_mut(target_agent) {
        // Calculate reputation change based on satisfaction
        let reputation_change = (satisfaction - 0.5) * 0.1; // Small incremental changes

        // Update domain-specific reputations
        let domains_to_update = determine_affected_domains(satisfaction);

        for domain in domains_to_update {
            let old_score = reputation.domain_scores.get(&domain).copied().unwrap_or(0.5);
            let new_score = (old_score + reputation_change).clamp(0.0, 1.0);

            reputation.domain_scores.insert(domain, new_score);

            // Send reputation change event
            reputation_events.write(ReputationChanged {
                agent: target_agent,
                domain,
                old_score,
                new_score,
                change_reason: ReputationChangeReason::DirectInteraction {
                    other_agent: interaction_partner,
                    satisfaction,
                },
            });
        }

        // Update overall reputation as weighted average of domains
        reputation.overall_score = calculate_overall_reputation(&reputation.domain_scores);

        // Update trend
        reputation.trend = reputation_change;

        // Increase visibility
        reputation.visibility += 1;
    }
}

/// Determine which reputation domains are affected by an interaction
fn determine_affected_domains(satisfaction: f32) -> Vec<ReputationDomain> {
    let mut affected_domains = vec![ReputationDomain::Sociability];

    if satisfaction > 0.7 {
        // High satisfaction interactions boost multiple domains
        affected_domains.push(ReputationDomain::Trustworthiness);
        affected_domains.push(ReputationDomain::Cooperation);
    } else if satisfaction > 0.6 {
        // Moderate satisfaction
        affected_domains.push(ReputationDomain::Helpfulness);
    }
    // Low satisfaction primarily affects sociability (already included)

    affected_domains
}

/// Calculate overall reputation as weighted average of domain scores
fn calculate_overall_reputation(domain_scores: &HashMap<ReputationDomain, f32>) -> f32 {
    let weights = [
        (ReputationDomain::Trustworthiness, 0.3),
        (ReputationDomain::Helpfulness, 0.25),
        (ReputationDomain::Cooperation, 0.2),
        (ReputationDomain::Sociability, 0.15),
        (ReputationDomain::KnowledgeSharing, 0.1),
    ];

    let mut weighted_sum = 0.0;
    let mut total_weight = 0.0;

    for (domain, weight) in weights {
        if let Some(score) = domain_scores.get(&domain) {
            weighted_sum += score * weight;
            total_weight += weight;
        }
    }

    if total_weight > 0.0 {
        weighted_sum / total_weight
    } else {
        0.5 // Default neutral reputation
    }
}

/// Update community-wide reputation tracking
fn update_community_reputation(
    community_memory: &mut CommunityMemory,
    agent: Entity,
    satisfaction: f32,
    current_time: f32,
) {
    let reputation_change = (satisfaction - 0.5) * 0.05; // Community changes more slowly

    let community_rep = community_memory.agent_reputations
        .entry(agent)
        .or_insert(CommunityReputation {
            aggregated_score: 0.5,
            source_count: 0,
            confidence: 0.1,
            last_updated: current_time,
        });

    // Update aggregated score using exponential moving average
    let alpha = 0.1; // Learning rate
    community_rep.aggregated_score =
        alpha * (community_rep.aggregated_score + reputation_change) +
            (1.0 - alpha) * community_rep.aggregated_score;

    community_rep.source_count += 1;
    community_rep.confidence = (community_rep.confidence + 0.01).min(1.0);
    community_rep.last_updated = current_time;
}

/// Apply time-based decay to all reputations
fn apply_reputation_decay(
    reputation_query: &mut Query<(Entity, &mut Reputation)>,
    delta_time: f32,
    reputation_events: &mut EventWriter<ReputationChanged>,
) {
    for (entity, mut reputation) in reputation_query.iter_mut() {
        let decay_amount = reputation.decay_rate * delta_time;

        // Decay domain scores toward neutral (0.5)
        for (domain, score) in reputation.domain_scores.iter_mut() {
            let old_score = *score;

            if *score > 0.5 {
                *score = (*score - decay_amount).max(0.5);
            } else if *score < 0.5 {
                *score = (*score + decay_amount).min(0.5);
            }

            // Send decay event if change is significant
            if (old_score - *score).abs() > 0.01 {
                reputation_events.write(ReputationChanged {
                    agent: entity,
                    domain: *domain,
                    old_score,
                    new_score: *score,
                    change_reason: ReputationChangeReason::Decay,
                });
            }
        }

        // Update overall reputation
        reputation.overall_score = calculate_overall_reputation(&reputation.domain_scores);

        // Decay trend toward zero
        reputation.trend *= 0.99;
    }
}

/// System for gossip propagation - spreading reputation information
/// Based on Social Network Theory and Information Diffusion Models
pub fn gossip_propagation_system(
    mut agents_query: Query<(Entity, &Transform, &mut SocialRelationshipNetwork, &Reputation)>,
    _community_query: Query<&mut CommunityMemory>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    // Collect agent positions for proximity calculations
    let agent_positions: Vec<(Entity, Vec3, f32)> = agents_query
        .iter()
        .map(|(entity, transform, _, reputation)| {
            (entity, transform.translation, reputation.overall_score)
        })
        .collect();

    // Process gossip propagation between nearby agents
    for (agent_entity, agent_transform, mut social_network, _agent_reputation) in agents_query.iter_mut() {
        // Only spread gossip occasionally
        if rand::random::<f32>() > 0.1 * delta_time {
            continue;
        }

        // Find nearby agents to gossip with
        for (other_entity, other_position, other_reputation_score) in &agent_positions {
            if *other_entity == agent_entity {
                continue;
            }

            let distance = agent_transform.translation.distance(*other_position);
            if distance <= 50.0 { // Gossip range
                // Share reputation information
                propagate_reputation_info(
                    &mut social_network,
                    *other_entity,
                    *other_reputation_score,
                );
            }
        }
    }
}

/// Propagate reputation information between agents
fn propagate_reputation_info(
    social_network: &mut SocialRelationshipNetwork,
    other_agent: Entity,
    reputation_score: f32,
) {
    // Simplified gossip: agents learn about others' reputations through social network
    if let Some(relationship) = social_network.relationships.get_mut(&other_agent) {
        // Trust affects how much we believe gossip
        let gossip_credibility = relationship.trust * 0.5;

        // Update our perception of this agent's reputation
        relationship.affinity = (relationship.affinity +
            (reputation_score - 0.5) * gossip_credibility * 0.1).clamp(-1.0, 1.0);
    }
}
