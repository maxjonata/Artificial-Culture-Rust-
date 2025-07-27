use bevy::prelude::*;

use crate::components::knowledge::KnowledgeBase;
use crate::components::needs::{BasicNeeds, Desire, DesireThresholds, DesirePriorities};
use crate::components::npc::{Npc, Personality};
use crate::components::environment::{Well, Restaurant, Hotel, SafeZone, InteractableResource, ResourceType};
use crate::components::pathfinding::{PathTarget, SteeringBehavior, ResourceMemory};
use crate::components::resources::{RumorTimer, GameConstants, ColorConstants};

/// Plugin for registering all custom components with Bevy's reflection system
pub struct CustomComponentsPlugin;

impl Plugin for CustomComponentsPlugin {
    fn build(&self, app: &mut App) {
        app
            // NPC components
            .register_type::<Npc>()
            .register_type::<Personality>()
            // Knowledge components
            .register_type::<KnowledgeBase>()
            // Needs components
            .register_type::<BasicNeeds>()
            .register_type::<Desire>()
            .register_type::<DesireThresholds>()
            // Environment components
            .register_type::<Well>()
            .register_type::<Restaurant>()
            .register_type::<Hotel>()
            .register_type::<SafeZone>()
            .register_type::<InteractableResource>()
            .register_type::<ResourceType>()
            // Pathfinding components
            .register_type::<PathTarget>()
            .register_type::<SteeringBehavior>()
            .register_type::<ResourceMemory>()
            // Resources
            .register_type::<RumorTimer>()
            .register_type::<GameConstants>()
            .register_type::<ColorConstants>()
        ;
    }
}

// ============================================================================
// DEFAULT IMPLEMENTATIONS FOR ALL COMPONENTS
// ============================================================================

impl Default for KnowledgeBase {
    fn default() -> Self {
        Self {
            knows_rumor: false,
        }
    }
}

impl Default for BasicNeeds {
    fn default() -> Self {
        Self {
            hunger: 80.0,
            thirst: 80.0,
            fatigue: 20.0,
            safety: 90.0,
            social: 60.0,
        }
    }
}

impl Default for DesireThresholds {
    fn default() -> Self {
        Self {
            // Critical thresholds based on psychological research
            hunger_threshold: 30.0,     // Below 30% triggers food seeking
            thirst_threshold: 25.0,     // Below 25% triggers water seeking (more urgent)
            fatigue_threshold: 75.0,    // Above 75% triggers rest seeking
            safety_threshold: 40.0,     // Below 40% triggers safety seeking
            social_threshold: 20.0,     // Below 20% triggers social interaction
            priority_weights: DesirePriorities::default(),
        }
    }
}

impl Default for DesirePriorities {
    fn default() -> Self {
        // Based on Maslow's hierarchy of needs
        Self {
            thirst: 100.0,    // Most critical - dehydration is fastest killer
            hunger: 90.0,     // Second most critical
            safety: 80.0,     // Security comes before comfort
            fatigue: 60.0,    // Important but not immediately life-threatening
            social: 40.0,     // Important for mental health but lowest priority
        }
    }
}

impl Default for Personality {
    fn default() -> Self {
        Self {
            openness: 0.5, // Neutral openness by default
        }
    }
}

impl Default for GameConstants {
    fn default() -> Self {
        Self {
            num_npcs: 20,
            npc_radius: 15.0,
            npc_speed: 200.0,
            social_distance: 100.0,
            hunger_decay: 0.1,
            thirst_decay: 0.1,
            fatigue_regen: 0.1,
            safety_decay: 0.1,
            social_decay: 0.1,
            num_wells: 1,
            num_restaurants: 1,
            num_hotels: 1,
            num_safe_zones: 1,
        }
    }
}

impl Default for ColorConstants {
    fn default() -> Self {
        Self {
            red: Color::srgb(1.0, 0.2, 0.2),
            green: Color::srgb(0.2, 1.0, 0.2),
        }
    }
}

impl Default for Well {
    fn default() -> Self {
        Self {
            water_capacity: 100.0,
            consumption_rate: 15.0,
            regeneration_rate: 2.0,
        }
    }
}

impl Default for Restaurant {
    fn default() -> Self {
        Self {
            food_capacity: 100.0,
            consumption_rate: 20.0,
            regeneration_rate: 1.5,
        }
    }
}

impl Default for Hotel {
    fn default() -> Self {
        Self {
            rest_capacity: 100.0,
            consumption_rate: 25.0,
            regeneration_rate: 3.0,
        }
    }
}

impl Default for SafeZone {
    fn default() -> Self {
        Self {
            safety_level: 95.0,
            effect_radius: 80.0,
        }
    }
}

impl Default for InteractableResource {
    fn default() -> Self {
        Self {
            resource_type: ResourceType::Water,
            availability: 100.0,
            max_interactions: 5,
            current_interactions: 0,
            regeneration_timer: 0.0,
        }
    }
}

impl Default for PathTarget {
    fn default() -> Self {
        Self {
            target_position: Vec2::ZERO,
            target_entity: None,
            arrival_threshold: 30.0,
            has_target: false,
            target_set_time: 0.0,
            max_pursuit_time: 10.0, // 10 seconds max pursuit
        }
    }
}

impl Default for SteeringBehavior {
    fn default() -> Self {
        Self {
            desired_velocity: Vec2::ZERO,
            steering_force: Vec2::ZERO,
            max_steering_force: 500.0,
            seek_weight: 1.0,
            wander_weight: 0.3,
            wander_angle: 0.0,
            wander_angle_change: 0.1,
        }
    }
}

impl Default for ResourceMemory {
    fn default() -> Self {
        Self {
            known_wells: Vec::new(),
            known_restaurants: Vec::new(),
            known_hotels: Vec::new(),
            known_safe_zones: Vec::new(),
            discovery_radius: 100.0,
            memory_decay_rate: 0.01,
        }
    }
}
