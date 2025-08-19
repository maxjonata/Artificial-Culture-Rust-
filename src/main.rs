// Minimal bootstrap after cleanup: only plugin skeletons
mod ai;
mod core;
mod world;
mod presentation;

use ai::AiPlugin;
use bevy::prelude::*;
use core::CorePlugin;
use presentation::PresentationPlugin;
use world::WorldPlugin;

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
