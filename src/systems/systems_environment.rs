use crate::components::components_environment::{Hotel, InteractableResource, ResourceType, Restaurant, SafeZone, Well};
use crate::components::components_needs::{BasicNeeds, Desire};
use crate::components::{components_npc::Npc, components_resources::GameConstants};
use crate::systems::events::events_environment::{ResourceDepletionEvent, ResourceInteractionEvent};
use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

/// Utility function to calculate satisfaction gained from resource interaction
/// System based on Diminishing Returns Theory - satisfaction decreases with higher current levels
fn calculate_satisfaction_gain(current_need: f32, base_gain: f32) -> f32 {
    // Diminishing returns: more satisfaction when a need is higher
    let need_factor = (100.0 - current_need) / 100.0; // 0.0 to 1.0
    base_gain * (0.5 + need_factor * 0.5) // Minimum 50% gain, up to 100%
}

/// System handling NPC interactions with wells (water sources)
/// System based on Resource Exchange Theory and Homeostatic Need Satisfaction
pub fn handle_well_interactions(
    mut collision_events: EventReader<CollisionEvent>,
    mut npc_query: Query<(Entity, &mut BasicNeeds, &Desire), With<Npc>>,
    mut well_query: Query<(Entity, &mut Well, &mut InteractableResource), Without<Npc>>,
    mut interaction_events: EventWriter<ResourceInteractionEvent>,
    mut depletion_events: EventWriter<ResourceDepletionEvent>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            // Try to match NPC with Well
            if let (Ok((npc_entity, mut needs, desire)), Ok((well_entity, mut well, mut resource))) =
                (npc_query.get_mut(*entity1), well_query.get_mut(*entity2)) {
                handle_water_interaction(
                    npc_entity, &mut needs, desire, well_entity, &mut well, &mut resource,
                    &mut interaction_events, &mut depletion_events,
                );
            } else if let (Ok((npc_entity, mut needs, desire)), Ok((well_entity, mut well, mut resource))) =
                (npc_query.get_mut(*entity2), well_query.get_mut(*entity1)) {
                handle_water_interaction(
                    npc_entity, &mut needs, desire, well_entity, &mut well, &mut resource,
                    &mut interaction_events, &mut depletion_events,
                );
            }
        }
    }
}

/// Helper function for water interaction logic
fn handle_water_interaction(
    npc_entity: Entity,
    needs: &mut BasicNeeds,
    desire: &Desire,
    well_entity: Entity,
    well: &mut Well,
    resource: &mut InteractableResource,
    interaction_events: &mut EventWriter<ResourceInteractionEvent>,
    depletion_events: &mut EventWriter<ResourceDepletionEvent>,
) {
    // Only interact if NPC desires water and resource is available
    if matches!(desire, Desire::FindWater) && well.water_capacity > 0.0 && resource.current_interactions < resource.max_interactions {
        let base_gain = 25.0; // Base thirst satisfaction
        let satisfaction_gained = calculate_satisfaction_gain(needs.thirst, base_gain);
        let water_consumed = well.consumption_rate.min(well.water_capacity);

        // Apply satisfaction to NPC
        needs.thirst = (needs.thirst + satisfaction_gained).clamp(0.0, 100.0);

        // Consume resource
        well.water_capacity -= water_consumed;
        resource.current_interactions += 1;

        // ML-HOOK: Fire events for quantifiable interaction tracking
        interaction_events.write(ResourceInteractionEvent {
            npc_entity,
            resource_entity: well_entity,
            resource_type: ResourceType::Water,
            satisfaction_gained,
            resource_consumed: water_consumed,
        });

        if well.water_capacity <= 0.0 {
            depletion_events.write(ResourceDepletionEvent {
                resource_entity: well_entity,
                resource_type: ResourceType::Water,
                remaining_capacity: 0.0,
            });
        }

        info!("NPC satisfied thirst at well! Thirst: {:.1}, Well capacity: {:.1}",
              needs.thirst, well.water_capacity);
    }
}

/// System handling NPC interactions with restaurants (food sources)
/// System based on Resource Exchange Theory and Homeostatic Need Satisfaction
pub fn handle_restaurant_interactions(
    mut collision_events: EventReader<CollisionEvent>,
    mut npc_query: Query<(Entity, &mut BasicNeeds, &Desire), With<Npc>>,
    mut restaurant_query: Query<(Entity, &mut Restaurant, &mut InteractableResource), Without<Npc>>,
    mut interaction_events: EventWriter<ResourceInteractionEvent>,
    mut depletion_events: EventWriter<ResourceDepletionEvent>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            // Try to match NPC with Restaurant
            if let (Ok((npc_entity, mut needs, desire)), Ok((restaurant_entity, mut restaurant, mut resource))) =
                (npc_query.get_mut(*entity1), restaurant_query.get_mut(*entity2)) {
                handle_food_interaction(
                    npc_entity, &mut needs, desire, restaurant_entity, &mut restaurant, &mut resource,
                    &mut interaction_events, &mut depletion_events,
                );
            } else if let (Ok((npc_entity, mut needs, desire)), Ok((restaurant_entity, mut restaurant, mut resource))) =
                (npc_query.get_mut(*entity2), restaurant_query.get_mut(*entity1)) {
                handle_food_interaction(
                    npc_entity, &mut needs, desire, restaurant_entity, &mut restaurant, &mut resource,
                    &mut interaction_events, &mut depletion_events,
                );
            }
        }
    }
}

/// Helper function for food interaction logic
fn handle_food_interaction(
    npc_entity: Entity,
    needs: &mut BasicNeeds,
    desire: &Desire,
    restaurant_entity: Entity,
    restaurant: &mut Restaurant,
    resource: &mut InteractableResource,
    interaction_events: &mut EventWriter<ResourceInteractionEvent>,
    depletion_events: &mut EventWriter<ResourceDepletionEvent>,
) {
    // Only interact if NPC desires food and resource is available
    if matches!(desire, Desire::FindFood) && restaurant.food_capacity > 0.0 && resource.current_interactions < resource.max_interactions {
        let base_gain = 30.0; // Base hunger satisfaction
        let satisfaction_gained = calculate_satisfaction_gain(needs.hunger, base_gain);
        let food_consumed = restaurant.consumption_rate.min(restaurant.food_capacity);

        // Apply satisfaction to NPC
        needs.hunger = (needs.hunger + satisfaction_gained).clamp(0.0, 100.0);

        // Consume resource
        restaurant.food_capacity -= food_consumed;
        resource.current_interactions += 1;

        // ML-HOOK: Fire events for quantifiable interaction tracking
        interaction_events.write(ResourceInteractionEvent {
            npc_entity,
            resource_entity: restaurant_entity,
            resource_type: ResourceType::Food,
            satisfaction_gained,
            resource_consumed: food_consumed,
        });

        if restaurant.food_capacity <= 0.0 {
            depletion_events.write(ResourceDepletionEvent {
                resource_entity: restaurant_entity,
                resource_type: ResourceType::Food,
                remaining_capacity: 0.0,
            });
        }

        info!("NPC satisfied hunger at restaurant! Hunger: {:.1}, Restaurant capacity: {:.1}",
              needs.hunger, restaurant.food_capacity);
    }
}

/// System handling NPC interactions with hotels (rest sources)
/// System based on Resource Exchange Theory and Fatigue Recovery Theory
pub fn handle_hotel_interactions(
    mut collision_events: EventReader<CollisionEvent>,
    mut npc_query: Query<(Entity, &mut BasicNeeds, &Desire), With<Npc>>,
    mut hotel_query: Query<(Entity, &mut Hotel, &mut InteractableResource), Without<Npc>>,
    mut interaction_events: EventWriter<ResourceInteractionEvent>,
    mut depletion_events: EventWriter<ResourceDepletionEvent>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            // Try to match NPC with Hotel
            if let (Ok((npc_entity, mut needs, desire)), Ok((hotel_entity, mut hotel, mut resource))) =
                (npc_query.get_mut(*entity1), hotel_query.get_mut(*entity2)) {
                handle_rest_interaction(
                    npc_entity, &mut needs, desire, hotel_entity, &mut hotel, &mut resource,
                    &mut interaction_events, &mut depletion_events,
                );
            } else if let (Ok((npc_entity, mut needs, desire)), Ok((hotel_entity, mut hotel, mut resource))) =
                (npc_query.get_mut(*entity2), hotel_query.get_mut(*entity1)) {
                handle_rest_interaction(
                    npc_entity, &mut needs, desire, hotel_entity, &mut hotel, &mut resource,
                    &mut interaction_events, &mut depletion_events,
                );
            }
        }
    }
}

/// Helper function for rest interaction logic
fn handle_rest_interaction(
    npc_entity: Entity,
    needs: &mut BasicNeeds,
    desire: &Desire,
    hotel_entity: Entity,
    hotel: &mut Hotel,
    resource: &mut InteractableResource,
    interaction_events: &mut EventWriter<ResourceInteractionEvent>,
    depletion_events: &mut EventWriter<ResourceDepletionEvent>,
) {
    // Only interact if NPC desires rest and resource is available
    if matches!(desire, Desire::Rest) && hotel.rest_capacity > 0.0 && resource.current_interactions < resource.max_interactions {
        let base_reduction = 35.0; // Base fatigue reduction
        let fatigue_reduced = calculate_satisfaction_gain(100.0 - needs.fatigue, base_reduction);
        let rest_consumed = hotel.consumption_rate.min(hotel.rest_capacity);

        // Apply fatigue reduction to NPC
        needs.fatigue = (needs.fatigue - fatigue_reduced).clamp(0.0, 100.0);

        // Consume resource
        hotel.rest_capacity -= rest_consumed;
        resource.current_interactions += 1;

        // ML-HOOK: Fire events for quantifiable interaction tracking
        interaction_events.write(ResourceInteractionEvent {
            npc_entity,
            resource_entity: hotel_entity,
            resource_type: ResourceType::Rest,
            satisfaction_gained: fatigue_reduced,
            resource_consumed: rest_consumed,
        });

        if hotel.rest_capacity <= 0.0 {
            depletion_events.write(ResourceDepletionEvent {
                resource_entity: hotel_entity,
                resource_type: ResourceType::Rest,
                remaining_capacity: 0.0,
            });
        }

        info!("NPC rested at hotel! Fatigue: {:.1}, Hotel capacity: {:.1}",
              needs.fatigue, hotel.rest_capacity);
    }
}

/// System for resource regeneration over time
/// System based on Natural Resource Recovery Theory
pub fn regenerate_environmental_resources(
    mut well_query: Query<(&mut Well, &mut InteractableResource), Without<Restaurant>>,
    mut restaurant_query: Query<(&mut Restaurant, &mut InteractableResource), (Without<Well>, Without<Hotel>)>,
    mut hotel_query: Query<(&mut Hotel, &mut InteractableResource), (Without<Well>, Without<Restaurant>)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    // Regenerate wells
    for (mut well, mut resource) in well_query.iter_mut() {
        if well.water_capacity < 100.0 {
            well.water_capacity = (well.water_capacity + well.regeneration_rate * delta_time).clamp(0.0, 100.0);
        }

        // Reset interaction counter over time
        resource.regeneration_timer += delta_time;
        if resource.regeneration_timer >= 5.0 { // Reset every 5 seconds
            resource.current_interactions = 0;
            resource.regeneration_timer = 0.0;
        }
    }

    // Regenerate restaurants
    for (mut restaurant, mut resource) in restaurant_query.iter_mut() {
        if restaurant.food_capacity < 100.0 {
            restaurant.food_capacity = (restaurant.food_capacity + restaurant.regeneration_rate * delta_time).clamp(0.0, 100.0);
        }

        resource.regeneration_timer += delta_time;
        if resource.regeneration_timer >= 5.0 {
            resource.current_interactions = 0;
            resource.regeneration_timer = 0.0;
        }
    }

    // Regenerate hotels
    for (mut hotel, mut resource) in hotel_query.iter_mut() {
        if hotel.rest_capacity < 100.0 {
            hotel.rest_capacity = (hotel.rest_capacity + hotel.regeneration_rate * delta_time).clamp(0.0, 100.0);
        }

        resource.regeneration_timer += delta_time;
        if resource.regeneration_timer >= 5.0 {
            resource.current_interactions = 0;
            resource.regeneration_timer = 0.0;
        }
    }
}
