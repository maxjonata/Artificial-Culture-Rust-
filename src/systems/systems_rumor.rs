use bevy::ecs::event::EventWriter;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::components::components_knowledge::KnowledgeBase;
use crate::components::components_npc::Npc;
use crate::components::components_npc::Personality;
use crate::components::components_resources::RumorTimer;
use crate::systems::events::events_rumor::{RumorInjectionEvent, RumorSpreadAttemptEvent, RumorSpreadEvent};

/// Utility function implementing Social Influence Theory for rumor transmission
/// System based on Social Influence Theory and Network Diffusion Models
fn calculate_rumor_spread_probability(spreader_openness: f32, receiver_openness: f32) -> f32 {
    // Based on research in social psychology: openness affects both transmission and reception
    const BASE_TRANSMISSION_RATE: f32 = 0.3; // Baseline probability from diffusion research
    const OPENNESS_MULTIPLIER: f32 = 0.7; // Weight of personality influence

    // ML-HOOK: Quantifiable formula that could be learned/optimized by ML
    let transmission_factor = spreader_openness * 0.3; // Spreader's willingness to share
    let reception_factor = receiver_openness * 0.7; // Receiver's willingness to believe

    (BASE_TRANSMISSION_RATE + (transmission_factor + reception_factor) * OPENNESS_MULTIPLIER).clamp(0.0, 1.0)
}

/// Utility function to determine rumor spread success based on stochastic social dynamics
/// System based on Stochastic Social Network Theory
fn attempt_rumor_spread(
    spreader: &KnowledgeBase,
    receiver: &mut KnowledgeBase,
    spreader_personality: &Personality,
    receiver_personality: &Personality,
) -> (bool, f32) {
    // Only spread if spreader knows and receiver doesn't know
    if !spreader.knows_rumor || receiver.knows_rumor {
        return (false, 0.0);
    }

    // ML-HOOK: Calculate probability using quantifiable personality traits
    let spread_probability = calculate_rumor_spread_probability(
        spreader_personality.openness,
        receiver_personality.openness,
    );

    let mut rng = rand::rng();
    let success = rng.random_bool(spread_probability as f64);

    if success {
        receiver.knows_rumor = true;
    }

    (success, spread_probability)
}

/// System implementing stochastic rumor injection based on epidemiological models
/// System based on Epidemiological Diffusion Models - initial seeding in social networks
pub fn inject_rumor_system(
    mut q_npcs: Query<(Entity, &mut KnowledgeBase), With<Npc>>,
    time: Res<Time>,
    mut timer: ResMut<RumorTimer>,
    mut injected: Local<bool>,
    mut injection_events: EventWriter<RumorInjectionEvent>,
) {
    if *injected { return; }

    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::rng();

        // ML-HOOK: Random selection could be replaced by ML agent choosing optimal target
        if let Some((entity, mut knowledge)) = q_npcs.iter_mut().choose(&mut rng) {
            knowledge.knows_rumor = true;
            *injected = true;

            // ML-HOOK: Fire event for quantifiable injection tracking
            injection_events.write(RumorInjectionEvent {
                target_entity: entity,
                injection_time: time.elapsed_secs(),
            });

            info!("RUMOR INJECTED! NPC entity {:?} now knows the secret at time {:.2}s",
                  entity, time.elapsed_secs());
        }
    }
}

/// System implementing social contagion through collision-based interactions
/// System based on Social Contagion Theory and Proximity-Based Information Diffusion
pub fn handle_rumor_spread(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<(Entity, &mut KnowledgeBase, &Personality), With<Npc>>,
    mut spread_events: EventWriter<RumorSpreadEvent>,
    mut attempt_events: EventWriter<RumorSpreadAttemptEvent>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            // Try to get both entities with error handling
            if let Ok([(entity1_id, mut knowledge1, personality1), (entity2_id, mut knowledge2, personality2)]) =
                query.get_many_mut([*entity1, *entity2]) {

                // ML-HOOK: Bi-directional spread attempts for complete social dynamics modeling

                // Attempt spread from entity1 to entity2
                let (success_1_to_2, prob_1_to_2) = attempt_rumor_spread(
                    &knowledge1, &mut knowledge2, personality1, personality2,
                );

                if knowledge1.knows_rumor && !knowledge2.knows_rumor {
                    attempt_events.write(RumorSpreadAttemptEvent {
                        spreader_entity: entity1_id,
                        receiver_entity: entity2_id,
                        success: success_1_to_2,
                        calculated_probability: prob_1_to_2,
                    });

                    if success_1_to_2 {
                        spread_events.write(RumorSpreadEvent {
                            spreader_entity: entity1_id,
                            receiver_entity: entity2_id,
                            spreader_openness: personality1.openness,
                            receiver_openness: personality2.openness,
                            spread_probability: prob_1_to_2,
                        });
                    }
                }

                // Attempt spread from entity2 to entity1 (if not already spread)
                let (success_2_to_1, prob_2_to_1) = attempt_rumor_spread(
                    &knowledge2, &mut knowledge1, personality2, personality1,
                );

                if knowledge2.knows_rumor && !knowledge1.knows_rumor {
                    attempt_events.write(RumorSpreadAttemptEvent {
                        spreader_entity: entity2_id,
                        receiver_entity: entity1_id,
                        success: success_2_to_1,
                        calculated_probability: prob_2_to_1,
                    });

                    if success_2_to_1 {
                        spread_events.write(RumorSpreadEvent {
                            spreader_entity: entity2_id,
                            receiver_entity: entity1_id,
                            spreader_openness: personality2.openness,
                            receiver_openness: personality1.openness,
                            spread_probability: prob_2_to_1,
                        });
                    }
                }
            }
        }
    }
}

/// System for tracking and analyzing rumor diffusion metrics
/// ML-HOOK: Provides quantifiable metrics for reward calculation and behavior analysis
pub fn analyze_rumor_diffusion(
    query: Query<&KnowledgeBase, With<Npc>>,
    mut last_analysis_time: Local<f32>,
    time: Res<Time>,
) {
    // Analyze every 2 seconds to track diffusion rate
    *last_analysis_time += time.delta_secs();
    if *last_analysis_time >= 2.0 {
        *last_analysis_time = 0.0;

        let total_npcs = query.iter().count();
        let informed_npcs = query.iter().filter(|k| k.knows_rumor).count();
        let diffusion_rate = if total_npcs > 0 {
            informed_npcs as f32 / total_npcs as f32
        } else {
            0.0
        };

        // ML-HOOK: Quantifiable diffusion metrics for observation space
        debug!(
            "Rumor Diffusion Analysis - Time: {:.1}s, Informed: {}/{}, Rate: {:.2}%",
            time.elapsed_secs(),
            informed_npcs,
            total_npcs,
            diffusion_rate * 100.0
        );

        // Log significant milestones for ML reward calculation
        if diffusion_rate >= 0.25 && diffusion_rate < 0.26 {
            info!("MILESTONE: 25% rumor diffusion reached at {:.1}s", time.elapsed_secs());
        } else if diffusion_rate >= 0.50 && diffusion_rate < 0.51 {
            info!("MILESTONE: 50% rumor diffusion reached at {:.1}s", time.elapsed_secs());
        } else if diffusion_rate >= 0.75 && diffusion_rate < 0.76 {
            info!("MILESTONE: 75% rumor diffusion reached at {:.1}s", time.elapsed_secs());
        } else if diffusion_rate >= 0.95 {
            info!("MILESTONE: Near-complete rumor diffusion (95%+) reached at {:.1}s", time.elapsed_secs());
        }
    }
}