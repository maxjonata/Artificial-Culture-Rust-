//! Game constants and configuration parameters.

use crate::core::types::Normalized;
use bevy::prelude::*;

/// Central configuration for simulation parameters.
/// Reference: Sterling & Eyer allostasis, Kahneman dual-process, Miller's Rule.
#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct GameConstants {
    pub num_npcs: u32,
    pub time_step: f32,

    pub hunger_decay_rate: Normalized,
    pub energy_decay_rate: Normalized,
    pub social_decay_rate: Normalized,

    pub allostasis_threshold: Normalized,
    pub trauma_threshold: Normalized,

    pub system1_response_time: f32,
    pub system2_response_time: f32,
    pub working_memory_capacity: u8,

    pub max_perceived_agents: u8,
    pub belief_decay_rate: Normalized,

    pub target_frame_time_ms: f32,
    pub max_agents_per_worker: u16,
}

impl Default for GameConstants {

    fn default() -> Self {
        Self {
            num_npcs: 1,
            time_step: 1.0 / 60.0,

            hunger_decay_rate: Normalized::new(0.05),
            energy_decay_rate: Normalized::new(0.08),
            social_decay_rate: Normalized::new(0.03),

            allostasis_threshold: Normalized::new(0.3),
            trauma_threshold: Normalized::new(0.7),

            system1_response_time: 0.1,
            system2_response_time: 1.5,
            working_memory_capacity: 5,

            max_perceived_agents: 12,
            belief_decay_rate: Normalized::new(0.02),

            target_frame_time_ms: 16.67,
            max_agents_per_worker: 1000,
        }
    }
}