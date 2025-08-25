//! Performance monitoring and profiling systems.

use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

/// Spawns the performance UI overlay for development monitoring.
pub fn spawn_perf_ui(mut commands: Commands) {
    commands.spawn(PerfUiAllEntries::default());
}

pub struct ProfilerPlugin;

impl Plugin for ProfilerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_systems(Startup, spawn_perf_ui);
    }
}
