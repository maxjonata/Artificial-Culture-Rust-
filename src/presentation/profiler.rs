//! Performance monitoring and profiling systems.
//!
//! This module provides performance monitoring capabilities for the simulation,
//! allowing developers to track frame rates, entity counts, and system performance
//! metrics during development and debugging.

use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

/// Spawns the performance UI overlay for development monitoring.
///
/// This system creates a debug overlay that displays real-time performance
/// metrics including frame rate, entity count, and system execution times.
/// The UI is purely observational and does not affect simulation behavior.
pub fn spawn_perf_ui(mut commands: Commands) {
    commands.spawn(PerfUiAllEntries::default());
}