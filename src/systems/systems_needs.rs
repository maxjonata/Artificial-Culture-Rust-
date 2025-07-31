use crate::components::components_needs::{BasicNeeds, CurrentDesire, Desire, DesireThresholds};
use crate::components::{components_constants::GameConstants, components_npc::Npc};
use crate::systems::events::events_needs::{
    CurrentDesireSet, DecisionTrigger, DesireChangeEvent, DesireChangeReason,
    DesireFulfillmentAttemptEvent, EvaluateDecision, NeedChangeEvent, NeedDecayEvent,
    NeedSatisfactionEvent, NeedType, SocialInteractionEvent, ThresholdCrossedEvent, ThresholdDirection,
};
use crate::utils::helpers::needs_helpers::{
    calculate_desire_utility, decay_needs, evaluate_most_urgent_desire, get_satisfaction_level,
    increase_social_satisfaction, should_activate_desire, should_deactivate_desire,
};
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

/// System implementing homeostatic need decay over time
/// System based on Homeostatic Drive Theory - maintains internal physiological balance
/// Now fires NeedChangeEvent for event-driven threshold monitoring
/// FIXED: All needs now use "higher = better satisfied" semantics
pub fn decay_basic_needs(
    mut query: Query<(Entity, &mut BasicNeeds), With<Npc>>,
    game_constants: Res<GameConstants>,
    mut need_decay_events: EventWriter<NeedDecayEvent>,
    mut need_change_events: EventWriter<NeedChangeEvent>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (entity, mut needs) in query.iter_mut() {
        let old_needs = *needs; // Capture old values for event firing

        let (hunger_change, thirst_change, rest_change, safety_change, social_change) =
            decay_needs(&mut needs, &game_constants, delta_time);

        // Fire individual need change events for threshold monitoring
        if hunger_change != 0.0 {
            need_change_events.write(NeedChangeEvent {
                entity,
                need_type: NeedType::Hunger,
                old_value: old_needs.hunger,
                new_value: needs.hunger,
                change_amount: hunger_change,
            });
        }

        if thirst_change != 0.0 {
            need_change_events.write(NeedChangeEvent {
                entity,
                need_type: NeedType::Thirst,
                old_value: old_needs.thirst,
                new_value: needs.thirst,
                change_amount: thirst_change,
            });
        }

        if rest_change != 0.0 {
            need_change_events.write(NeedChangeEvent {
                entity,
                need_type: NeedType::Rest,
                old_value: old_needs.rest,
                new_value: needs.rest,
                change_amount: rest_change,
            });
        }

        if safety_change != 0.0 {
            need_change_events.write(NeedChangeEvent {
                entity,
                need_type: NeedType::Safety,
                old_value: old_needs.safety,
                new_value: needs.safety,
                change_amount: safety_change,
            });
        }

        if social_change != 0.0 {
            need_change_events.write(NeedChangeEvent {
                entity,
                need_type: NeedType::Social,
                old_value: old_needs.social,
                new_value: needs.social,
                change_amount: social_change,
            });
        }

        // ML-HOOK: Fire event for quantifiable state change tracking
        need_decay_events.write(NeedDecayEvent {
            entity,
            hunger_change,
            thirst_change,
            rest_change: rest_change,
            safety_change,
            social_change,
        });
    }
}

/// Event-driven system that handles social interactions based on Social Exchange Theory
/// System based on Social Exchange Theory - positive interactions increase social satisfaction
/// Only triggers when collision events occur, not on every frame
/// FIXED: Now uses correct social field and function name
pub fn handle_social_interactions(
    mut collision_events: EventReader<CollisionEvent>,
    mut social_events: EventWriter<SocialInteractionEvent>,
    mut need_change_events: EventWriter<NeedChangeEvent>,
    mut needs_query: Query<&mut BasicNeeds, With<Npc>>,
) {
    const SOCIAL_INTERACTION_BOOST: f32 = 0.1; // Normalized boost for 0.0-1.0 scale

    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            // Try to get both entities' BasicNeeds components
            if let Ok([mut needs1, mut needs2]) = needs_query.get_many_mut([*entity1, *entity2]) {
                let old_social_1 = needs1.social;
                let old_social_2 = needs2.social;

                // Both NPCs gain social satisfaction from the interaction using helper
                let boost1 = increase_social_satisfaction(&mut needs1, SOCIAL_INTERACTION_BOOST);
                let boost2 = increase_social_satisfaction(&mut needs2, SOCIAL_INTERACTION_BOOST);

                // Fire individual need change events for threshold monitoring
                if boost1 > 0.0 {
                    need_change_events.write(NeedChangeEvent {
                        entity: *entity1,
                        need_type: NeedType::Social,
                        old_value: old_social_1,
                        new_value: needs1.social,
                        change_amount: boost1,
                    });
                }

                if boost2 > 0.0 {
                    need_change_events.write(NeedChangeEvent {
                        entity: *entity2,
                        need_type: NeedType::Social,
                        old_value: old_social_2,
                        new_value: needs2.social,
                        change_amount: boost2,
                    });
                }

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

/// Event-driven system that handles desire fulfillment attempts
/// Triggers when NPCs with specific desires attempt to satisfy them
/// Much more performant than polling all NPCs every frame
pub fn desire_fulfillment_system(
    mut desire_events: EventReader<DesireChangeEvent>,
    mut fulfillment_events: EventWriter<DesireFulfillmentAttemptEvent>,
    mut satisfaction_events: EventWriter<NeedSatisfactionEvent>,
    mut need_change_events: EventWriter<NeedChangeEvent>,
    mut needs_query: Query<&mut BasicNeeds>,
) {
    for event in desire_events.read() {
        // Only process desires that indicate seeking behavior
        if matches!(event.new_desire, Desire::FindFood | Desire::FindWater | Desire::Rest | Desire::FindSafety)
            && let (Ok(mut needs)) = (
            needs_query.get_mut(event.entity)
        ) {
            // Simulate resource interaction based on desire type
            let (need_type, satisfaction_amount, success) = match event.new_desire {
                Desire::FindFood => {
                    let old_hunger = needs.hunger;
                    let boost = 0.4;
                    needs.hunger = (needs.hunger + boost).clamp(0.0, 1.0);
                    let actual_boost = needs.hunger - old_hunger;

                    if actual_boost > 0.0 {
                        need_change_events.write(NeedChangeEvent {
                            entity: event.entity,
                            need_type: NeedType::Hunger,
                            old_value: old_hunger,
                            new_value: needs.hunger,
                            change_amount: actual_boost,
                        });
                    }

                    info!("NPC found food! Hunger increased to {:.2}", needs.hunger);
                    (NeedType::Hunger, actual_boost, true)
                }
                Desire::FindWater => {
                    let old_thirst = needs.thirst;
                    let boost = 0.5;
                    needs.thirst = (needs.thirst + boost).clamp(0.0, 1.0);
                    let actual_boost = needs.thirst - old_thirst;

                    if actual_boost > 0.0 {
                        need_change_events.write(NeedChangeEvent {
                            entity: event.entity,
                            need_type: NeedType::Thirst,
                            old_value: old_thirst,
                            new_value: needs.thirst,
                            change_amount: actual_boost,
                        });
                    }

                    info!("NPC found water! Thirst increased to {:.2}", needs.thirst);
                    (NeedType::Thirst, actual_boost, true)
                }
                Desire::Rest => {
                    let old_rest = needs.rest;
                    let recovery = 0.3;
                    needs.rest = (needs.rest + recovery).clamp(0.0, 1.0);
                    let actual_recovery = needs.rest - old_rest;

                    if actual_recovery > 0.0 {
                        need_change_events.write(NeedChangeEvent {
                            entity: event.entity,
                            need_type: NeedType::Rest,
                            old_value: old_rest,
                            new_value: needs.rest,
                            change_amount: actual_recovery,
                        });
                    }

                    info!("NPC is resting! Rest level increased to {:.2}", needs.rest);
                    (NeedType::Rest, actual_recovery, true)
                }
                Desire::FindSafety => {
                    let old_safety = needs.safety;
                    let boost = 0.35;
                    needs.safety = (needs.safety + boost).clamp(0.0, 1.0);
                    let actual_boost = needs.safety - old_safety;

                    if actual_boost > 0.0 {
                        need_change_events.write(NeedChangeEvent {
                            entity: event.entity,
                            need_type: NeedType::Safety,
                            old_value: old_safety,
                            new_value: needs.safety,
                            change_amount: actual_boost,
                        });
                    }

                    info!("NPC found safety! Safety increased to {:.2}", needs.safety);
                    (NeedType::Safety, actual_boost, true)
                }
                _ => return, // Should not happen given our filter above
            };

            // Fire events for ML tracking
            fulfillment_events.write(DesireFulfillmentAttemptEvent {
                entity: event.entity,
                desire: event.new_desire,
                success,
                satisfaction_gained: satisfaction_amount,
            });

            if success && satisfaction_amount > 0.0 {
                satisfaction_events.write(NeedSatisfactionEvent {
                    entity: event.entity,
                    need_type,
                    satisfaction_amount,
                    resource_entity: None, // For now, no specific resource entities
                });
            }
        }
    }
}

/// Event-driven system that fires threshold crossing events when needs change
/// This replaces the polling-based threshold checking for better performance
/// Now uses DualThreshold structure with high/low thresholds for hysteresis
/// FIXED: Updated to use new field names and correct threshold logic
pub fn threshold_monitoring_system(
    mut need_change_events: EventReader<NeedChangeEvent>,
    mut threshold_events: EventWriter<ThresholdCrossedEvent>,
    thresholds_query: Query<&DesireThresholds>,
) {
    for event in need_change_events.read() {
        // Direct entity access - no iteration needed since we have the entity from the event
        if let Ok(thresholds) = thresholds_query.get(event.entity) {
            let dual_threshold = match event.need_type {
                NeedType::Hunger => &thresholds.hunger_threshold,
                NeedType::Thirst => &thresholds.thirst_threshold,
                NeedType::Rest => &thresholds.rest_threshold,
                NeedType::Safety => &thresholds.safety_threshold,
                NeedType::Social => &thresholds.social_threshold,
            };

            // NEW LOGIC: For "higher = better" semantics
            // - Activate desires when satisfaction drops BELOW high_threshold
            // - Start pathfinding when satisfaction drops BELOW low_threshold
            // - Deactivate when satisfaction rises ABOVE high_threshold
            let old_below_high = event.old_value < dual_threshold.high_threshold;
            let new_below_high = event.new_value < dual_threshold.high_threshold;
            let old_below_low = event.old_value < dual_threshold.low_threshold;
            let new_below_low = event.new_value < dual_threshold.low_threshold;

            // Fire events when crossing BELOW high threshold (desire activation)
            if !old_below_high && new_below_high {
                threshold_events.write(ThresholdCrossedEvent {
                    entity: event.entity,
                    need_type: event.need_type,
                    threshold_value: dual_threshold.high_threshold,
                    current_value: event.new_value,
                    crossed_direction: ThresholdDirection::Below,
                    should_trigger_desire: true,
                });
            }

            // Fire events when crossing ABOVE high threshold (desire deactivation)
            if old_below_high && !new_below_high {
                threshold_events.write(ThresholdCrossedEvent {
                    entity: event.entity,
                    need_type: event.need_type,
                    threshold_value: dual_threshold.high_threshold,
                    current_value: event.new_value,
                    crossed_direction: ThresholdDirection::Above,
                    should_trigger_desire: false,
                });
            }

            // Fire events when crossing BELOW low threshold (urgent pathfinding)
            if !old_below_low && new_below_low {
                threshold_events.write(ThresholdCrossedEvent {
                    entity: event.entity,
                    need_type: event.need_type,
                    threshold_value: dual_threshold.low_threshold,
                    current_value: event.new_value,
                    crossed_direction: ThresholdDirection::Below,
                    should_trigger_desire: true, // Even more urgent
                });
            }
        }
    }
}

/// Event-driven system that updates desires based on threshold crossing events
/// Much more performant than polling all NPCs every frame
/// Now uses weighted utility formula and dual threshold logic
/// FIXED: Updated to use correct field names for new need types
pub fn desire_update_system(
    mut threshold_events: EventReader<ThresholdCrossedEvent>,
    mut desire_events: EventWriter<DesireChangeEvent>,
    needs_query: Query<&BasicNeeds>,
    thresholds_query: Query<&DesireThresholds>,
    mut desires_query: Query<&mut Desire>,
) {
    for event in threshold_events.read() {
        // Direct entity access - no iteration needed since we have the entity from the event
        if let (Ok(needs), Ok(thresholds), Ok(mut current_desire)) = (
            needs_query.get(event.entity),
            thresholds_query.get(event.entity),
            desires_query.get_mut(event.entity)
        ) {
            if event.should_trigger_desire {
                // Determine new desire based on need type
                let new_desire = match event.need_type {
                    NeedType::Hunger => Desire::FindFood,
                    NeedType::Thirst => Desire::FindWater,
                    NeedType::Rest => Desire::Rest,
                    NeedType::Safety => Desire::FindSafety,
                    NeedType::Social => Desire::Socialize,
                };

                // Calculate weighted utility score using the new formula
                let utility = calculate_desire_utility(new_desire, &needs, &thresholds);

                if *current_desire != new_desire {
                    info!("NPC desire changed from {:?} to {:?} due to {:?} threshold crossing",
                          *current_desire, new_desire, event.need_type);

                    desire_events.write(DesireChangeEvent {
                        entity: event.entity,
                        old_desire: *current_desire,
                        new_desire,
                        urgency_score: utility,
                        trigger_reason: DesireChangeReason::ThresholdCrossed,
                    });

                    *current_desire = new_desire;
                }
            } else {
                // Use the new helper functions to check if we should deactivate the current desire
                let should_deactivate = should_deactivate_desire(*current_desire, &needs, &thresholds);

                if should_deactivate {
                    desire_events.write(DesireChangeEvent {
                        entity: event.entity,
                        old_desire: *current_desire,
                        new_desire: Desire::Wander,
                        urgency_score: 0.0,
                        trigger_reason: DesireChangeReason::NeedSatisfied,
                    });

                    *current_desire = Desire::Wander;
                }
            }
        }
    }
}

/// System that logs current status of NPCs for debugging
/// Can be disabled in production builds
/// FIXED: Updated to use correct field names
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
                "NPC Status - Desire: {:?}, Hunger: {:.2}, Thirst: {:.2}, Rest: {:.2}, Safety: {:.2}, Social: {:.2}",
                desire, needs.hunger, needs.thirst, needs.rest, needs.safety, needs.social
            );
        }
    }
}

/// The missing decision_making_system from roadmap 1.3.2
/// Event-driven system that evaluates all competing desires and selects the highest utility one
/// Triggered by EvaluateDecision events for better performance than polling
/// Uses the existing evaluate_most_urgent_desire helper function for proper decision-making
pub fn decision_making_system(
    mut evaluation_events: EventReader<EvaluateDecision>,
    mut current_desire_events: EventWriter<CurrentDesireSet>,
    mut desire_change_events: EventWriter<DesireChangeEvent>,
    needs_query: Query<&BasicNeeds>,
    thresholds_query: Query<&DesireThresholds>,
    mut current_desires_query: Query<&mut CurrentDesire>,
    time: Res<Time>,
) {
    for event in evaluation_events.read() {
        // Direct entity access - no iteration needed since we have the entity from the event
        if let (Ok(needs), Ok(thresholds), Ok(mut current_desire)) = (
            needs_query.get(event.entity),
            thresholds_query.get(event.entity),
            current_desires_query.get_mut(event.entity)
        ) {
            // Use the existing helper function that evaluates ALL competing desires
            let (best_desire, utility_score) = evaluate_most_urgent_desire(&needs, &thresholds);

            // ML-HOOK: Calculate utility for all desires for observation space
            let competing_desires = vec![
                (Desire::FindSafety, calculate_desire_utility(Desire::FindSafety, &needs, &thresholds)),
                (Desire::FindWater, calculate_desire_utility(Desire::FindWater, &needs, &thresholds)),
                (Desire::FindFood, calculate_desire_utility(Desire::FindFood, &needs, &thresholds)),
                (Desire::Rest, calculate_desire_utility(Desire::Rest, &needs, &thresholds)),
                (Desire::Socialize, calculate_desire_utility(Desire::Socialize, &needs, &thresholds)),
            ];

            // Only update if the desire actually changed
            if current_desire.desire != best_desire {
                let old_desire = current_desire.desire;

                // Update the CurrentDesire component
                current_desire.desire = best_desire;
                current_desire.utility_score = utility_score;
                current_desire.last_evaluated = time.elapsed_secs();

                // Fire events for system communication and ML tracking
                current_desire_events.write(CurrentDesireSet {
                    entity: event.entity,
                    desire: best_desire,
                    utility_score,
                    competing_desires: competing_desires.clone(),
                });

                desire_change_events.write(DesireChangeEvent {
                    entity: event.entity,
                    old_desire,
                    new_desire: best_desire,
                    urgency_score: utility_score,
                    trigger_reason: match event.trigger_reason {
                        DecisionTrigger::NeedChanged => DesireChangeReason::ThresholdCrossed,
                        _ => DesireChangeReason::ManualOverride,
                    },
                });

                info!("Decision made for NPC: {:?} -> {:?} (utility: {:.2})",
                      old_desire, best_desire, utility_score);
            }
        }
    }
}

/// System that triggers periodic decision re-evaluation
/// Replaces the need to poll all NPCs every frame by firing EvaluateDecision events
/// Based on bounded rationality theory - agents don't constantly re-evaluate
pub fn periodic_decision_trigger_system(
    query: Query<Entity, With<CurrentDesire>>,
    mut evaluation_events: EventWriter<EvaluateDecision>,
    mut last_evaluation: Local<f32>,
    time: Res<Time>,
) {
    const EVALUATION_INTERVAL: f32 = 2.0; // Re-evaluate every 2 seconds

    *last_evaluation += time.delta_secs();
    if *last_evaluation >= EVALUATION_INTERVAL {
        *last_evaluation = 0.0;

        // Trigger evaluation for all agents
        for entity in query.iter() {
            evaluation_events.write(EvaluateDecision {
                entity,
                trigger_reason: DecisionTrigger::Periodic,
            });
        }
    }
}

/// Optimized threshold monitoring system that triggers decision evaluation
/// Instead of directly setting desires, it triggers the decision_making_system
/// This allows for proper utility comparison between all competing desires
pub fn optimized_threshold_monitoring_system(
    mut need_change_events: EventReader<NeedChangeEvent>,
    mut threshold_events: EventWriter<ThresholdCrossedEvent>,
    mut evaluation_events: EventWriter<EvaluateDecision>,
    thresholds_query: Query<&DesireThresholds>,
) {
    for event in need_change_events.read() {
        // Direct entity access - no iteration needed since we have the entity from the event
        if let Ok(thresholds) = thresholds_query.get(event.entity) {
            let dual_threshold = match event.need_type {
                NeedType::Hunger => &thresholds.hunger_threshold,
                NeedType::Thirst => &thresholds.thirst_threshold,
                NeedType::Rest => &thresholds.rest_threshold,
                NeedType::Safety => &thresholds.safety_threshold,
                NeedType::Social => &thresholds.social_threshold,
            };

            // Check if significant threshold crossings occurred
            let old_below_high = event.old_value < dual_threshold.high_threshold;
            let new_below_high = event.new_value < dual_threshold.high_threshold;
            let old_below_low = event.old_value < dual_threshold.low_threshold;
            let new_below_low = event.new_value < dual_threshold.low_threshold;

            let significant_change = old_below_high != new_below_high || old_below_low != new_below_low;

            if significant_change {
                // Fire threshold crossed event for logging/debugging
                threshold_events.write(ThresholdCrossedEvent {
                    entity: event.entity,
                    need_type: event.need_type,
                    threshold_value: dual_threshold.high_threshold,
                    current_value: event.new_value,
                    crossed_direction: if new_below_high {
                        ThresholdDirection::Below
                    } else {
                        ThresholdDirection::Above
                    },
                    should_trigger_desire: new_below_high,
                });

                // Trigger decision evaluation instead of directly setting desires
                evaluation_events.write(EvaluateDecision {
                    entity: event.entity,
                    trigger_reason: DecisionTrigger::NeedChanged,
                });
            }
        }
    }
}
