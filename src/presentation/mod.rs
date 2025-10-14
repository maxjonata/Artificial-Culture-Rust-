//! Presentation layer for visualization and debugging.

mod profiler;
pub mod performance_alerts;
pub mod system_profiler;
pub mod profiling_examples;
mod profiler_controls;
mod debug_ui;

use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierDebugRenderPlugin;

/// Main presentation plugin for visualization systems.
pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add debug UI for agent state inspection
            .add_plugins(profiler::ProfilerPlugin)
            .add_plugins(debug_ui::DebugUiPlugin)
            .add_plugins(performance_alerts::PerformanceAlertsPlugin)
            .add_plugins(system_profiler::SystemProfilerPlugin)
            .add_plugins(profiler_controls::ProfilerControlsPlugin)
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(
                Startup, setup_camera_and_background,
            );

        // Profiling examples are now in tests, not in the main app
    }
}


fn setup_camera_and_background(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)));
}
