pub mod pathfinding;
pub mod mapping;      // Cognitive mapping and spatial memory
pub mod learning;     // Synaptic plasticity and path learning

use bevy::prelude::*;

pub struct NavigationPlugin;  // Renamed from MovementPlugin to align with neuro-inspired navigation

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register components from this domain
            .register_type::<pathfinding::PathTarget>()
            .register_type::<pathfinding::SteeringBehavior>()
            .register_type::<pathfinding::SpatialThingMemory>()  // Fixed naming convention
            .register_type::<mapping::CognitiveMap>()
            .register_type::<mapping::SpatialNavigationNetwork>()
            .register_type::<learning::SynapticPlasticity>()
            .register_type::<learning::PathLearning>()
            .register_type::<learning::AdaptiveBehavior>()
            // Register events
            .add_event::<mapping::LandmarkDiscovered>()
            .add_event::<mapping::SpatialMapUpdated>()
            .add_event::<learning::SynapticLearningEvent>()
            // Add systems from this domain
            .add_systems(Update, (
                pathfinding::basic_movement_system,
                pathfinding::pathfinding_system,
                pathfinding::desire_to_pathfinding_system, // Bridge cognition to navigation
                mapping::cognitive_mapping_system,
                learning::synaptic_plasticity_system,
                learning::path_learning_system,
                learning::memory_consolidation_system,
            ));
    }
}
