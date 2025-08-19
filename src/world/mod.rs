//! World simulation infrastructure for the Artificial Society.
//!
//! This module contains all systems related to the physical world simulation,
//! including physics, environment management, and spatial relationships.
//! The world serves as the shared context in which all agents operate and interact.
//!
//! # Architecture
//!
//! The world module is organized into several sub-domains:
//! - `physics/`: Physical simulation using bevy_rapier2d
//! - `environment/`: Environmental objects, resources, and spatial structures
//! - `weather/`: Climate and weather systems affecting agent behavior
//!
//! All world systems follow event-driven patterns to maintain performance
//! and enable clean separation of concerns.

pub mod environment;   // Rapier2D physics configuration and utilities
use bevy::prelude::*;

/// Main world plugin that orchestrates all physical world systems.
///
/// This plugin manages the shared physical environment where agents exist
/// and interact. It integrates physics simulation, environmental objects,
/// and spatial queries needed for agent perception and navigation.
pub struct WorldPlugin;

/// Plugin for world management (environment, physics, etc.)
/// Separate from AI intelligence - handles the physical world
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add Environment management systems
            .add_plugins(environment::EnvironmentPlugin);
    }
}
