use crate::components::{BasicNeeds, Hotel, Restaurant, Well};
use crate::systems::events::events_environment::{
    ResourceInteractionAttemptEvent, ResourceInteractionSuccessEvent
    , ResourceRegenerationEvent,
};
use crate::systems::events::events_needs::{NeedChangeEvent, NeedType};
use crate::utils::helpers::resource_helpers::{
    apply_satisfaction_to_needs, calculate_consumption_rate,
    calculate_satisfaction_gain, get_need_level_for_resource,
};
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::prelude::*;

/// Event-driven system that handles resource interactions when NPCs desire specific resources
/// Based on Environmental Psychology - resource interaction affects satisfaction
/// Replaces the O(n) polling system with event-driven approach for better performance
pub fn resource_interaction_system(
    mut interaction_events: EventReader<ResourceInteractionAttemptEvent>,
    mut success_events: EventWriter<ResourceInteractionSuccessEvent>,
    mut need_change_events: EventWriter<NeedChangeEvent>,
    mut needs_query: Query<&mut BasicNeeds>,
    mut well_query: Query<&mut Well>,
    mut restaurant_query: Query<&mut Restaurant>,
    mut hotel_query: Query<&mut Hotel>,
) {
    for event in interaction_events.read() {
        if let Ok(mut needs) = needs_query.get_mut(event.npc_entity) {
            let current_need_level = get_need_level_for_resource(&needs, event.resource_type);

            // Skip if need is already satisfied
            if current_need_level >= 0.9 {
                continue;
            }

            // Try to interact with the resource
            let interaction_result = match event.resource_type {
                crate::components::components_environment::ResourceType::Water => {
                    if let Ok(mut well) = well_query.get_mut(event.resource_entity) {
                        if well.water_capacity > 0.1 {
                            let satisfaction = calculate_satisfaction_gain(
                                event.resource_type,
                                well.water_capacity,
                                current_need_level,
                            );
                            let consumption = calculate_consumption_rate(
                                event.resource_type,
                                1.0 - current_need_level,
                            );

                            well.water_capacity = (well.water_capacity - consumption).clamp(0.0, 1.0);
                            Some((satisfaction, well.water_capacity))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                crate::components::components_environment::ResourceType::Food => {
                    if let Ok(mut restaurant) = restaurant_query.get_mut(event.resource_entity) {
                        if restaurant.food_capacity > 0.1 {
                            let satisfaction = calculate_satisfaction_gain(
                                event.resource_type,
                                restaurant.food_capacity,
                                current_need_level,
                            );
                            let consumption = calculate_consumption_rate(
                                event.resource_type,
                                1.0 - current_need_level,
                            );

                            restaurant.food_capacity = (restaurant.food_capacity - consumption).clamp(0.0, 1.0);
                            Some((satisfaction, restaurant.food_capacity))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                crate::components::components_environment::ResourceType::Rest => {
                    if let Ok(_hotel) = hotel_query.get(event.resource_entity) {
                        // Hotels provide unlimited rest - they're safe zones
                        let satisfaction = calculate_satisfaction_gain(
                            event.resource_type,
                            1.0, // Hotels always at full capacity
                            current_need_level,
                        );
                        Some((satisfaction, 1.0))
                    } else {
                        None
                    }
                }
                _ => None, // Other resource types not implemented yet
            };

            // Apply satisfaction if interaction was successful
            if let Some((satisfaction, resource_availability_after)) = interaction_result {
                let old_need_level = current_need_level;
                let actual_change = apply_satisfaction_to_needs(&mut needs, event.resource_type, satisfaction);

                // Fire need change event for threshold monitoring
                if actual_change > 0.0 {
                    let need_type = match event.resource_type {
                        crate::components::components_environment::ResourceType::Water => NeedType::Thirst,
                        crate::components::components_environment::ResourceType::Food => NeedType::Hunger,
                        crate::components::components_environment::ResourceType::Rest => NeedType::Fatigue,
                        crate::components::components_environment::ResourceType::Safety => NeedType::Safety,
                        crate::components::components_environment::ResourceType::Social => NeedType::Social,
                    };

                    need_change_events.write(NeedChangeEvent {
                        entity: event.npc_entity,
                        need_type,
                        old_value: old_need_level,
                        new_value: get_need_level_for_resource(&needs, event.resource_type),
                        change_amount: actual_change,
                    });
                }

                // Fire success event for ML tracking
                success_events.write(ResourceInteractionSuccessEvent {
                    npc_entity: event.npc_entity,
                    resource_entity: event.resource_entity,
                    resource_type: event.resource_type,
                    satisfaction_gained: satisfaction,
                    resource_availability_after,
                });
            }
        }
    }
}

/// System that regenerates resource capacity over time
/// Based on Resource Economics - natural regeneration patterns
/// Uses timer-based approach instead of polling all resources every frame
pub fn resource_regeneration_system(
    time: Res<Time>,
    mut regeneration_events: EventWriter<ResourceRegenerationEvent>,
    mut resource_query: Query<(Entity, &mut crate::components::components_environment::Resource)>,
    mut well_query: Query<&mut Well>,
    mut restaurant_query: Query<&mut Restaurant>,
) {
    let delta_time = time.delta_secs();

    // Regenerate unified resources
    for (entity, mut resource) in resource_query.iter_mut() {
        if resource.availability < 1.0 {
            let old_availability = resource.availability;
            let regeneration_amount = crate::utils::helpers::resource_helpers::calculate_regeneration_amount(&resource, delta_time);

            if regeneration_amount > 0.0 {
                resource.availability = (resource.availability + regeneration_amount).clamp(0.0, 1.0);

                regeneration_events.write(ResourceRegenerationEvent {
                    resource_entity: entity,
                    resource_type: resource.resource_type,
                    availability_before: old_availability,
                    availability_after: resource.availability,
                    regeneration_amount,
                });
            }
        }
    }

    // Regenerate legacy wells
    for mut well in well_query.iter_mut() {
        if well.water_capacity < 1.0 {
            well.water_capacity = (well.water_capacity + 0.02 * delta_time).clamp(0.0, 1.0);
        }
    }

    // Regenerate legacy restaurants
    for mut restaurant in restaurant_query.iter_mut() {
        if restaurant.food_capacity < 1.0 {
            restaurant.food_capacity = (restaurant.food_capacity + 0.015 * delta_time).clamp(0.0, 1.0);
        }
    }
}
