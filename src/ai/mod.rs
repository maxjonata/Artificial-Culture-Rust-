//! AI system infrastructure for the Artificial Society simulation.
//!
//! This module contains the core artificial intelligence systems that drive agent behavior.
//! Following the "Brain as a Society of Systems" paradigm, each agent's mind consists of
//! multiple competing and cooperating sub-systems (physiology, cognition, perception, etc.).
//!
//! # Architecture
//!
//! The AI module is organized into domain-specific sub-modules:
//! - `cognition/`: Decision-making, memory, and belief systems
//! - `physiology/`: Needs, health, and biological drives
//! - `perception/`: Sensory input processing and world model updates
//! - `social/`: Interpersonal interactions and communication
//!
//! Each domain implements the Plugin pattern for modular integration.

mod cognition;
mod perception;
mod physiology;
mod social;

use bevy::prelude::*;

/// Main AI plugin that orchestrates all artificial intelligence systems.
///
/// This plugin serves as the central coordinator for agent intelligence,
/// integrating multiple cognitive and behavioral sub-systems into a cohesive
/// artificial mind following neuro-computational principles.
pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                cognition::AiCognitionPlugin,
                perception::AiPerceptionPlugin,
                physiology::AiPhysiologyPlugin,
                social::AiSocialPlugin,
            ));
    }
}
