//! Utility modules and helper functions for the Artificial Society simulation.
//!
//! This module contains reusable utilities, helper functions, and macros that
//! support various aspects of the simulation. The utilities are organized by
//! functional domain to maintain clean separation of concerns.
//!
//! # Sub-modules
//!
//! - `helpers/`: Domain-specific helper functions
//! - `macros/`: Reusable macros for reducing boilerplate code

pub mod helpers;
pub mod macros;

use bevy::prelude::*;

/// Utility plugin that provides common helper systems and resources.
///
/// This plugin registers utility systems that don't belong to any specific
/// domain but are needed across the simulation for performance optimization
/// and code reuse.
pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {}
}
