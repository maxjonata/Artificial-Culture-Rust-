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

use crate::core::types::NormalizedFloat;
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

    // === Physiological Constants ===
    // Based on homeostatic research (Sterling, 2012)

    /// Rate at which hunger increases per hour (0.05 = 20 hours to full hunger)
    pub hunger_decay_rate: NormalizedFloat,

    /// Rate at which energy decreases per hour when active
    pub energy_decay_rate: NormalizedFloat,

    /// Rate at which social need increases when isolated (per hour)
    pub social_decay_rate: NormalizedFloat,

    // === Stress System Thresholds ===
    // Based on allostasis research

    /// Chronic stress threshold for Homeostasis → Allostasis transition
    pub allostasis_threshold: NormalizedFloat,

    /// Chronic stress threshold for Allostasis → PostTraumatic transition
    pub trauma_threshold: NormalizedFloat,

    // === Cognitive Performance Constants ===
    // Based on Kahneman's dual-process theory

    /// System 1 response time (fast, emotional reactions)
    pub system1_response_time: f32, // seconds

    /// System 2 response time (deliberate thinking)
    pub system2_response_time: f32, // seconds

    /// Working memory capacity (Miller's 7±2 rule)
    pub working_memory_capacity: u8,

    // === Social Perception Constants ===

    /// Maximum number of agents one can actively track
    pub max_perceived_agents: u8,

    /// Rate at which beliefs fade without reinforcement (per day)
    pub belief_decay_rate: NormalizedFloat,

    // === Performance Limits ===

    /// Target frame time in milliseconds
    pub target_frame_time_ms: f32, // 60 FPS

    /// Maximum agents per simulation worker
    pub max_agents_per_worker: u16,
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

            // === Physiological Constants ===
            hunger_decay_rate: 0.05,
            energy_decay_rate: 0.08,
            social_decay_rate: 0.03,

            // === Stress System Thresholds ===
            allostasis_threshold: 0.3,
            trauma_threshold: 0.7,

            // === Cognitive Performance Constants ===
            system1_response_time: 0.1,
            system2_response_time: 1.5,
            working_memory_capacity: 5,

            // === Social Perception Constants ===
            max_perceived_agents: 12,
            belief_decay_rate: 0.02,

            // === Performance Limits ===
            target_frame_time_ms: 16.67,
            max_agents_per_worker: 1000,
        }
    }
}