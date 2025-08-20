//! Game constants and configuration parameters.
//!
//! This module centralizes all configuration values used throughout the simulation.
//! Constants are organized by domain and backed by scientific references where
//! applicable. This approach eliminates magic numbers and provides a single
//! source of truth for simulation parameters.
//!
//! # Scientific Grounding
//!
//! All physiological and psychological constants should reference academic
//! literature from the `/docs` folder to ensure realistic agent behavior.

use bevy::prelude::*;

/// Central configuration resource containing all simulation parameters.
///
/// This resource provides a centralized location for all constants used
/// throughout the simulation, enabling easy tuning and scientific validation
/// of simulation parameters.
#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct GameConstants {
    /// Maximum number of NPCs that can exist simultaneously
    pub num_npcs: u32,
    /// Simulation time step in seconds
    pub time_step: f32,
}

impl Default for GameConstants {
    /// Creates a new GameConstants resource with scientifically-grounded defaults.
    ///
    /// Default values are based on computational feasibility and references
    /// from cognitive science literature in the project documentation.
    fn default() -> Self {
        Self {
            num_npcs: 1, // Reasonable limit for real-time simulation
            time_step: 1.0 / 60.0, // 60 FPS target
        }
    }
}