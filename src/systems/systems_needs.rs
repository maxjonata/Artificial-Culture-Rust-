use crate::components::components_needs::{BasicNeeds, Desire, DesireThresholds};
use crate::components::{components_constants::GameConstants, components_npc::Npc};
use crate::systems::events::events_needs::{
    DesireChangeEvent, DesireChangeReason, DesireFulfillmentAttemptEvent, NeedChangeEvent,
    NeedDecayEvent, NeedSatisfactionEvent, NeedType, SocialInteractionEvent,
    ThresholdCrossedEvent, ThresholdDirection,
};
use crate::utils::helpers::{apply_needs_decay, evaluate_most_urgent_desire, get_satisfaction_level, increase_social_need};
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

/// System implementing homeostatic need decay over time
/// System based on Homeostatic Drive Theory - maintains internal physiological balance
/// Now fires NeedChangeEvent for event-driven threshold monitoring
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

        let (hunger_change, thirst_change, fatigue_change, safety_change, social_change) =
            apply_needs_decay(&mut needs, &game_constants, delta_time);

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

        if fatigue_change != 0.0 {
            need_change_events.write(NeedChangeEvent {
                entity,
                need_type: NeedType::Fatigue,
                old_value: old_needs.fatigue,
                new_value: needs.fatigue,
                change_amount: fatigue_change,
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
            fatigue_change,
            safety_change,
            social_change,
        });
    }
}

/// Event-driven system that handles social interactions based on Social Exchange Theory
/// System based on Social Exchange Theory - positive interactions increase social satisfaction
/// Only triggers when collision events occur, not on every frame
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
                let boost1 = increase_social_need(&mut needs1, SOCIAL_INTERACTION_BOOST);
                let boost2 = increase_social_need(&mut needs2, SOCIAL_INTERACTION_BOOST);

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
                    let old_fatigue = needs.fatigue;
                    let recovery = 0.3;
                    needs.fatigue = (needs.fatigue - recovery).clamp(0.0, 1.0);
                    let actual_recovery = old_fatigue - needs.fatigue;

                    if actual_recovery > 0.0 {
                        need_change_events.write(NeedChangeEvent {
                            entity: event.entity,
                            need_type: NeedType::Fatigue,
                            old_value: old_fatigue,
                            new_value: needs.fatigue,
                            change_amount: -actual_recovery, // Negative because fatigue decreased
                        });
                    }

                    info!("NPC is resting! Fatigue decreased to {:.2}", needs.fatigue);
                    (NeedType::Fatigue, actual_recovery, true)
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
/// Optimized to avoid entity iteration by using direct entity access
pub fn threshold_monitoring_system(
    mut need_change_events: EventReader<NeedChangeEvent>,
    mut threshold_events: EventWriter<ThresholdCrossedEvent>,
    thresholds_query: Query<&DesireThresholds>,
) {
    for event in need_change_events.read() {
        // Direct entity access - no iteration needed since we have the entity from the event
        if let Ok(thresholds) = thresholds_query.get(event.entity) {
            let threshold_value = match event.need_type {
                NeedType::Hunger => thresholds.hunger_threshold,
                NeedType::Thirst => thresholds.thirst_threshold,
                NeedType::Fatigue => thresholds.fatigue_threshold,
                NeedType::Safety => thresholds.safety_threshold,
                NeedType::Social => thresholds.social_threshold,
            };

            // Check if we crossed the threshold
            let old_below = event.old_value < threshold_value;
            let new_below = event.new_value < threshold_value;

            // For fatigue, we check if we're above threshold (fatigue accumulates)
            let (old_triggered, new_triggered) = if event.need_type == NeedType::Fatigue {
                (event.old_value > threshold_value, event.new_value > threshold_value)
            } else {
                (old_below, new_below)
            };

            if old_triggered != new_triggered {
                let direction = if new_triggered {
                    ThresholdDirection::Below
                } else {
                    ThresholdDirection::Above
                };

                threshold_events.write(ThresholdCrossedEvent {
                    entity: event.entity,
                    need_type: event.need_type,
                    threshold_value,
                    current_value: event.new_value,
                    crossed_direction: direction,
                    should_trigger_desire: new_triggered,
                });
            }
        }
    }
}

/// Event-driven system that updates desires based on threshold crossing events
/// Much more performant than polling all NPCs every frame
/// Optimized to avoid entity iteration by using direct entity access
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
                // Determine new desire based on need type and urgency
                let new_desire = match event.need_type {
                    NeedType::Hunger => Desire::FindFood,
                    NeedType::Thirst => Desire::FindWater,
                    NeedType::Fatigue => Desire::Rest,
                    NeedType::Safety => Desire::FindSafety,
                    NeedType::Social => Desire::Socialize,
                };

                // Calculate urgency score for ML tracking
                let urgency = match event.need_type {
                    NeedType::Hunger => (thresholds.hunger_threshold - needs.hunger) * thresholds.priority_weights.hunger,
                    NeedType::Thirst => (thresholds.thirst_threshold - needs.thirst) * thresholds.priority_weights.thirst,
                    NeedType::Fatigue => (needs.fatigue - thresholds.fatigue_threshold) * thresholds.priority_weights.fatigue,
                    NeedType::Safety => (thresholds.safety_threshold - needs.safety) * thresholds.priority_weights.safety,
                    NeedType::Social => (thresholds.social_threshold - needs.social) * thresholds.priority_weights.social,
                };

                if *current_desire != new_desire {
                    info!("NPC desire changed from {:?} to {:?} due to {:?} threshold crossing",
                          *current_desire, new_desire, event.need_type);

                    desire_events.write(DesireChangeEvent {
                        entity: event.entity,
                        old_desire: *current_desire,
                        new_desire,
                        urgency_score: urgency,
                        trigger_reason: DesireChangeReason::ThresholdCrossed,
                    });

                    *current_desire = new_desire;
                }
            } else {
                // Threshold crossed upward - check if we should stop seeking
                let should_stop_seeking = match event.need_type {
                    NeedType::Hunger => *current_desire == Desire::FindFood && needs.hunger > 0.7,
                    NeedType::Thirst => *current_desire == Desire::FindWater && needs.thirst > 0.75,
                    NeedType::Fatigue => *current_desire == Desire::Rest && needs.fatigue < 0.4,
                    NeedType::Safety => *current_desire == Desire::FindSafety && needs.safety > 0.8,
                    NeedType::Social => *current_desire == Desire::Socialize && needs.social > 0.5,
                };

                if should_stop_seeking {
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
                "NPC Status - Desire: {:?}, Hunger: {:.2}, Thirst: {:.2}, Fatigue: {:.2}, Safety: {:.2}, Social: {:.2}",
                desire, needs.hunger, needs.thirst, needs.fatigue, needs.safety, needs.social
            );
        }
    }
}
