//! Presentation layer for the Artificial Society simulation.
//!
//! This module handles all visualization, debugging, and user interface systems.
//! Following the architectural separation of concerns, presentation systems are
//! purely for human observation and must not interact with the AI decision-making
//! loop to maintain simulation integrity.
//!
//! # Sub-modules
//!
//! - `debug_ui/`: Developer debugging interfaces and agent state inspection
//! - `visualization/`: Agent behavior visualization and world state rendering

mod profiler;

use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::*;
use iyes_perf_ui::PerfUiPlugin;

/// Main presentation plugin that orchestrates all visualization systems.
///
/// This plugin manages the human-facing aspects of the simulation including
/// debug overlays, performance monitoring, and visual representations of
/// agent behavior. All systems in this plugin are read-only observers that
/// do not influence the simulation state.
pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add debug UI for agent state inspection
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_plugins(EguiPlugin::default())
            .add_plugins(WorldInspectorPlugin::default())
            .add_systems(
                Startup,
                (
                    setup_camera_and_background,
                    profiler::spawn_perf_ui,
                ),
            );
    }
}

/// Sets up the camera and basic visual elements for the simulation.
///
/// This system creates the main camera required for rendering and adds
/// a basic background to provide visual context for the debug overlays.
/// Without a camera, no UI elements would be visible on screen.
fn setup_camera_and_background(mut commands: Commands) {
    // Spawn the main 2D camera for rendering UI and world elements
    commands.spawn(Camera2d);

    // Add a subtle background color so we can see the debug UI
    commands.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)));
}
