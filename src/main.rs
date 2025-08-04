// Domain-driven module structure
mod ai;
mod core;
mod utils;
mod world;
mod presentation;

use crate::ai::AiPlugin;
use crate::core::constants::{ColorConstants, GameConstants, RumorTimer};
use crate::core::CorePlugin;
use crate::presentation::PresentationPlugin;
use crate::world::WorldPlugin;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin,
    quick::WorldInspectorPlugin,
};
use bevy_rapier2d::prelude::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            // Domain-driven plugins for the refactored architecture
            CorePlugin,
            AiPlugin,
            WorldPlugin, // This includes RapierPhysicsPlugin via PhysicsPlugin
            PresentationPlugin,
        ))
        // Resources initialization
        .insert_resource(RumorTimer(Timer::from_seconds(3.0, TimerMode::Once)))
        .insert_resource(GameConstants::default())
        .insert_resource(ColorConstants::default())

        // Register Rapier debug render context for inspector control
        .register_type::<DebugRenderContext>()

        // Startup systems - camera only, entities are spawned by domain plugins
        .add_systems(Startup, setup_camera)
        .run();
}
