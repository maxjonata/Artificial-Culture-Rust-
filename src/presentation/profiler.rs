//! Performance monitoring and profiling systems.

use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

/// Configuration for the performance UI
#[derive(Resource, Debug)]
pub struct ProfilerConfig {
    pub enabled: bool,
}

impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
            enabled: true, // Enable by default for development
        }
    }
}

/// Spawns a minimal performance UI overlay for development monitoring.
pub fn spawn_perf_ui(mut commands: Commands, config: Res<ProfilerConfig>) {
    if config.enabled {
        commands.spawn(PerfUiAllEntries::default());
        info!("Performance UI spawned - minimal overhead configuration");
    }
}

pub struct ProfilerPlugin;

impl Plugin for ProfilerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .init_resource::<ProfilerConfig>()
            .add_systems(Startup, spawn_perf_ui);
    }
}
