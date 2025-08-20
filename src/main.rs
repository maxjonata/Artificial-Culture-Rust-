//! Minimal bootstrap after cleanup: only plugin skeletons
mod ai;
mod core;
mod world;
mod presentation;
mod utils;

use bevy::prelude::*;

use ai::AiPlugin;
use core::CorePlugin;
use presentation::PresentationPlugin;
use world::WorldPlugin;

/// Entry point for the Artificial Society simulation.
///
/// Sets up the Bevy application with all necessary plugins for the agent-based simulation.
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CorePlugin,
            AiPlugin,
            WorldPlugin,
            PresentationPlugin,
        ))
        .run();
}
