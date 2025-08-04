pub mod fps_display;
pub mod vision_debug;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use vision_debug::VisionDebugSettings;

use crate::presentation::fps_display::{spawn_fps_display_system, update_fps_display_system, FpsData, FpsDisplay};

/// Plugin for presentation layer - handles UI, debugging, and visualization
/// Separate from game logic - purely for human observation and interaction
pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add Bevy's frame time diagnostics plugin for FPS tracking
            .add_plugins(FrameTimeDiagnosticsPlugin::default())

            // Insert vision debug settings resource
            .insert_resource(VisionDebugSettings::default())

            // Register presentation components
            .register_type::<FpsDisplay>()
            .register_type::<FpsData>()
            // Register components from this domain
            .register_type::<vision_debug::VisionDebugRenderer>()

            // Add presentation systems
            .add_systems(Startup, spawn_fps_display_system)
            .add_systems(Update, (
                update_fps_display_system,
                vision_debug::vision_cone_debug_system,
                vision_debug::update_vision_facing_system,
            ));
    }
}
