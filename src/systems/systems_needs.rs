use crate::components::components_needs::{BasicNeeds, Desire, DesireThresholds};
use crate::components::{components_npc::Npc, components_resources::GameConstants};
use crate::systems::events::events_needs::{DesireChangeEvent, NeedDecayEvent, SocialInteractionEvent};
use bevy::ecs::event::EventWriter;
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

/// Utility function to apply decay to needs based on homeostatic drive theory
/// System based on Homeostatic Drive Theory - organisms maintain internal balance
fn apply_needs_decay(needs: &mut BasicNeeds, game_constants: &GameConstants) -> (f32, f32, f32, f32, f32) {
    let hunger_change = -game_constants.hunger_decay;
    let thirst_change = -game_constants.thirst_decay;
    let fatigue_change = game_constants.fatigue_regen; // Fatigue accumulates over time
    let safety_change = -game_constants.safety_decay;
    let social_change = -game_constants.social_decay;

    needs.hunger = (needs.hunger + hunger_change).clamp(0.0, 100.0);
    needs.thirst = (needs.thirst + thirst_change).clamp(0.0, 100.0);
    needs.fatigue = (needs.fatigue + fatigue_change).clamp(0.0, 100.0);
    needs.safety = (needs.safety + safety_change).clamp(0.0, 100.0);
    needs.social = (needs.social + social_change).clamp(0.0, 100.0);

    (hunger_change, thirst_change, fatigue_change, safety_change, social_change)
}

/// Utility function to increase social need based on Social Exchange Theory
/// System based on Social Exchange Theory - positive interactions increase social satisfaction
fn increase_social_need(needs: &mut BasicNeeds, amount: f32) -> f32 {
    let old_social = needs.social;
    needs.social = (needs.social + amount).clamp(0.0, 100.0);
    needs.social - old_social // Return actual change for ML tracking
}

/// Utility function implementing Maslow's Hierarchy of Needs for desire evaluation
/// System based on Maslow's Hierarchy of Needs and Threshold Psychology
fn evaluate_most_urgent_desire(needs: &BasicNeeds, thresholds: &DesireThresholds) -> (Desire, f32) {
    let mut urgent_desires = Vec::new();

    // ML-HOOK: Each urgency calculation provides quantifiable state for observation space
    if needs.hunger < thresholds.hunger_threshold {
        let urgency = (thresholds.hunger_threshold - needs.hunger) * thresholds.priority_weights.hunger;
        urgent_desires.push((Desire::FindFood, urgency));
    }

    if needs.thirst < thresholds.thirst_threshold {
        let urgency = (thresholds.thirst_threshold - needs.thirst) * thresholds.priority_weights.thirst;
        urgent_desires.push((Desire::FindWater, urgency));
    }

    if needs.fatigue > thresholds.fatigue_threshold {
        let urgency = (needs.fatigue - thresholds.fatigue_threshold) * thresholds.priority_weights.fatigue;
        urgent_desires.push((Desire::Rest, urgency));
    }

    if needs.safety < thresholds.safety_threshold {
        let urgency = (thresholds.safety_threshold - needs.safety) * thresholds.priority_weights.safety;
        urgent_desires.push((Desire::FindSafety, urgency));
    }

    if needs.social < thresholds.social_threshold {
        let urgency = (thresholds.social_threshold - needs.social) * thresholds.priority_weights.social;
        urgent_desires.push((Desire::Socialize, urgency));
    }

    // Return the desire with highest urgency score and the score itself for ML tracking
    urgent_desires
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(desire, urgency)| (desire, urgency))
        .unwrap_or((Desire::Wander, 0.0))
}

/// Utility function to get quantifiable satisfaction levels for ML observation space
/// ML-HOOK: Provides normalized satisfaction metrics for reward calculation
fn get_satisfaction_level(needs: &BasicNeeds, desire: &Desire) -> f32 {
    match desire {
        Desire::FindFood => needs.hunger / 100.0,
        Desire::FindWater => needs.thirst / 100.0,
        Desire::Rest => (100.0 - needs.fatigue) / 100.0,
        Desire::FindSafety => needs.safety / 100.0,
        Desire::Socialize => needs.social / 100.0,
        Desire::Wander => 0.5, // Neutral satisfaction for wandering
    }
}

/// System implementing homeostatic need decay over time
/// System based on Homeostatic Drive Theory - maintains internal physiological balance
pub fn decay_basic_needs(
    mut query: Query<(Entity, &mut BasicNeeds), With<Npc>>,
    game_constants: Res<GameConstants>,
    mut need_decay_events: EventWriter<NeedDecayEvent>,
) {
    for (entity, mut needs) in query.iter_mut() {
        let (hunger_change, thirst_change, fatigue_change, safety_change, social_change) =
            apply_needs_decay(&mut needs, &game_constants);

        // ML-HOOK: Fire event for quantifiable state change tracking
        need_decay_events.write(NeedDecayEvent {
            entity,
            hunger_change,
            thirst_change,
            fatigue_change,
            safety_change,
            social_change,
        });
    }
}

/// System handling social interactions based on Social Exchange Theory
/// System based on Social Exchange Theory - positive interactions increase social satisfaction
pub fn handle_social_interactions(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<&mut BasicNeeds, With<Npc>>,
    mut social_events: EventWriter<SocialInteractionEvent>,
) {
    const SOCIAL_INTERACTION_BOOST: f32 = 10.0; // Based on empirical social psychology research

    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            // Try to get both entities' BasicNeeds components
            if let Ok([mut needs1, mut needs2]) = query.get_many_mut([*entity1, *entity2]) {
                // Both NPCs gain social satisfaction from the interaction
                let boost1 = increase_social_need(&mut needs1, SOCIAL_INTERACTION_BOOST);
                let boost2 = increase_social_need(&mut needs2, SOCIAL_INTERACTION_BOOST);

                // ML-HOOK: Fire events for quantifiable interaction tracking
                social_events.write(SocialInteractionEvent {
                    entity_1: *entity1,
                    entity_2: *entity2,
                    social_boost: (boost1 + boost2) / 2.0, // Average boost for symmetric interaction
                });
            }
        }
    }
}

/// System implementing Maslow's Hierarchy for desire evaluation
/// System based on Maslow's Hierarchy of Needs and Threshold Psychology
pub fn update_desires_from_needs(
    mut query: Query<(Entity, &BasicNeeds, &DesireThresholds, &mut Desire), With<Npc>>,
    mut desire_events: EventWriter<DesireChangeEvent>,
) {
    for (entity, needs, thresholds, mut current_desire) in query.iter_mut() {
        // ML-HOOK: The desire evaluation could be replaced by an RL agent's action selection
        let (new_desire, urgency_score) = evaluate_most_urgent_desire(needs, thresholds);

        // Only update if the desire has changed to avoid unnecessary updates
        if *current_desire != new_desire {
            info!("NPC desire changed from {:?} to {:?} with urgency {:.2}", *current_desire, new_desire, urgency_score);

            // ML-HOOK: Fire event for quantifiable behavior change tracking
            desire_events.write(DesireChangeEvent {
                entity,
                old_desire: current_desire.clone(),
                new_desire: new_desire.clone(),
                urgency_score,
            });

            *current_desire = new_desire;
        }
    }
}

/// System that handles desire satisfaction when NPCs achieve their goals
/// Follows Single Responsibility Principle - only manages desire fulfillment
pub fn fulfill_desires(
    mut query: Query<(&mut BasicNeeds, &mut Desire), With<Npc>>,
) {
    for (mut needs, mut desire) in query.iter_mut() {
        match *desire {
            Desire::FindFood => {
                // Simulate finding and consuming food
                if needs.hunger < 100.0 {
                    needs.hunger = (needs.hunger + 40.0).clamp(0.0, 100.0);
                    info!("NPC found food! Hunger increased to {}", needs.hunger);

                    // Check if hunger is sufficiently satisfied
                    if needs.hunger > 70.0 {
                        *desire = Desire::Wander;
                    }
                }
            }
            Desire::FindWater => {
                // Simulate finding and drinking water
                if needs.thirst < 100.0 {
                    needs.thirst = (needs.thirst + 50.0).clamp(0.0, 100.0);
                    info!("NPC found water! Thirst increased to {}", needs.thirst);

                    // Check if thirst is sufficiently satisfied
                    if needs.thirst > 75.0 {
                        *desire = Desire::Wander;
                    }
                }
            }
            Desire::Rest => {
                // Simulate resting and recovering from fatigue
                if needs.fatigue > 0.0 {
                    needs.fatigue = (needs.fatigue - 30.0).clamp(0.0, 100.0);
                    info!("NPC is resting! Fatigue decreased to {}", needs.fatigue);

                    // Check if fatigue is sufficiently reduced
                    if needs.fatigue < 40.0 {
                        *desire = Desire::Wander;
                    }
                }
            }
            Desire::FindSafety => {
                // Simulate finding a safe location
                if needs.safety < 100.0 {
                    needs.safety = (needs.safety + 35.0).clamp(0.0, 100.0);
                    info!("NPC found safety! Safety increased to {}", needs.safety);

                    // Check if safety is sufficiently restored
                    if needs.safety > 80.0 {
                        *desire = Desire::Wander;
                    }
                }
            }
            Desire::Socialize => {
                // Social interaction is handled by collision system
                // This is just a fallback in case no interactions occur
                if needs.social > 50.0 {
                    *desire = Desire::Wander;
                }
            }
            Desire::Wander => {
                // No specific action needed for wandering
                // This is the default state when no urgent needs exist
            }
        }
    }
}

/// System that logs current status of NPCs for debugging
/// Can be disabled in production builds
#[cfg(debug_assertions)]
pub fn debug_npc_status(
    query: Query<(&BasicNeeds, &Desire), With<Npc>>,
    mut last_debug_time: Local<f32>,
    time: Res<Time>,
) {
    // Log status every 5 seconds to avoid spam
    *last_debug_time += time.delta_secs();
    if *last_debug_time >= 5.0 {
        *last_debug_time = 0.0;
        for (needs, desire) in query.iter() {
            debug!(
                "NPC Status - Desire: {:?}, Hunger: {:.1}, Thirst: {:.1}, Fatigue: {:.1}, Safety: {:.1}, Social: {:.1}",
                desire, needs.hunger, needs.thirst, needs.fatigue, needs.safety, needs.social
            );
        }
    }
}
