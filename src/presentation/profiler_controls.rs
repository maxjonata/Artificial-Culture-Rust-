//! Keyboard controls for the system profiler

use crate::presentation::system_profiler::{SystemProfiler, SystemProfilerEvent};
use bevy::prelude::*;

/// System that handles keyboard controls for the profiler
pub fn profiler_keyboard_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut profiler_events: EventWriter<SystemProfilerEvent>,
    profiler: Res<SystemProfiler>,
) {
    // F4: Print recommendations
    if keyboard.just_pressed(KeyCode::F4) {
        profiler_events.write(SystemProfilerEvent::PrintRecommendations);
    }

    // F5: Generate full report
    if keyboard.just_pressed(KeyCode::F5) {
        profiler_events.write(SystemProfilerEvent::GenerateReport);
    }

    // F6: Clear profiling data
    if keyboard.just_pressed(KeyCode::F6) {
        profiler_events.write(SystemProfilerEvent::ClearData);
    }

    // F7: Toggle profiling on/off
    if keyboard.just_pressed(KeyCode::F7) {
        if profiler.profiling_enabled {
            profiler_events.write(SystemProfilerEvent::StopProfiling);
        } else {
            profiler_events.write(SystemProfilerEvent::StartProfiling);
        }
    }

    // Print help on F8
    if keyboard.just_pressed(KeyCode::F8) {
        info!("=== System Profiler Controls ===");
        info!("F4: Print performance recommendations");
        info!("F5: Generate full performance report");
        info!("F6: Clear all profiling data");
        info!("F7: Toggle profiling on/off");
        info!("F8: Show this help");
    }
}

/// Plugin for profiler controls
pub struct ProfilerControlsPlugin;

impl Plugin for ProfilerControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, profiler_keyboard_controls);
    }
}
