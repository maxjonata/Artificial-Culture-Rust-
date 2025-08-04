pub mod environment;
pub mod thing;

use bevy::prelude::*;

/// Plugin for world management (environment, physics, etc.)
/// Separate from AI intelligence - handles the physical world
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add world management systems
            .register_type::<thing::Thing>()           // Fixed naming convention
            .register_type::<thing::ThingType>()       // Fixed naming convention
            .register_type::<thing::ThingOwnership>()  // Fixed naming convention
            .register_type::<thing::InteractableThing>() // Fixed naming convention
            // Legacy components - will be phased out
            .register_type::<thing::Well>()
            .register_type::<thing::Restaurant>()
            .register_type::<thing::Hotel>()
            .register_type::<thing::SafeZone>()
            // Add spawning systems
            .add_systems(Startup, environment::spawn_environment_system);
    }
}