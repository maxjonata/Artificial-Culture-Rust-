use bevy::prelude::*;

use crate::components::components_constants::{ColorConstants, GameConstants, RumorTimer};
use crate::components::components_environment::{Hotel, InteractableResource, Resource, ResourceOwnership, ResourceTransfer, ResourceType, Restaurant, SafeZone, Well};
use crate::components::components_knowledge::KnowledgeBase;
use crate::components::components_needs::{BasicNeeds, Desire, DesirePriorities, DesireThresholds};
use crate::components::components_npc::{Npc, Personality};
use crate::components::components_pathfinding::{PathTarget, ResourceMemory, SteeringBehavior};

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
            .register_type::<DesirePriorities>()
            // Environment components - New unified resource system
            .register_type::<Resource>()
            .register_type::<ResourceType>()
            .register_type::<ResourceOwnership>()
            .register_type::<ResourceTransfer>()
            .register_type::<InteractableResource>()
            // Environment components - Legacy (for backward compatibility)
            .register_type::<Well>()
            .register_type::<Restaurant>()
            .register_type::<Hotel>()
            .register_type::<SafeZone>()
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


impl Default for DesireThresholds {
    fn default() -> Self {
        Self {
            // Critical thresholds based on psychological research - normalized 0.0-1.0
            hunger_threshold: 0.3,   // Seek food when hunger drops below 30%
            thirst_threshold: 0.25,  // Seek water when thirst drops below 25% (more urgent)
            fatigue_threshold: 0.2,  // Rest when fatigue drops below 20%
            safety_threshold: 0.4,   // Seek safety when below 40%
            social_threshold: 0.3,   // Socialize when below 30%
            priority_weights: DesirePriorities::default(),
        }
    }
}

impl Default for DesirePriorities {
    fn default() -> Self {
        // Based on Maslow's hierarchy of needs - normalized 0.0-1.0
        Self {
            thirst: 1.0,    // Maximum priority - dehydration is fastest killer
            hunger: 0.85,   // Very high priority - survival need
            safety: 0.75,   // High priority - security need
            fatigue: 0.6,   // Medium priority - physiological need
            social: 0.3,    // Lower priority - social need
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
            // Differentiated decay rates based on physiological urgency
            hunger_decay: 0.008,    // Moderate decay - can survive weeks without food
            thirst_decay: 0.015,    // Faster decay - can only survive days without water
            fatigue_regen: 0.005,   // Slower accumulation - fatigue builds gradually
            safety_decay: 0.012,    // Moderate-fast decay - stress builds quickly in unsafe environments
            social_decay: 0.003,    // Slowest decay - social isolation takes time to affect wellbeing
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

impl Default for InteractableResource {
    fn default() -> Self {
        Self {
            resource_type: ResourceType::Water,
            availability: 1.0,        // Normalized: 1.0 = full availability
            max_interactions: 5,      // Data-oriented: u8 for memory efficiency
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
