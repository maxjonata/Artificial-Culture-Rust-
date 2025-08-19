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
            .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_systems(
                Startup,
                (
                    profiler::spawn_perf_ui,
                ),
            );
    }
}
