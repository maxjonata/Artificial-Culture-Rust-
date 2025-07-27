mod components;
mod systems;
mod entity_builders;

use bevy::input::common_conditions::input_toggle_active;
// Usando Bevy 0.13.2
use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin,
    quick::WorldInspectorPlugin,
};
use bevy_rapier2d::prelude::*;
// Removed conflicting import
use crate::components::*;
use crate::components::default::CustomComponentsPlugin;
use crate::components::resources::{GameConstants, ColorConstants};
use crate::systems::rumor::{inject_rumor_system, handle_rumor_spread};
use crate::systems::movement::movement_system;
use crate::systems::visual::color_system;
use crate::systems::needs::{decay_basic_needs, handle_social_interactions};
use crate::entity_builders::spawn_test_npcs;

// --- Sistemas ---

fn setup_simulation(
    mut commands: Commands,
    game_constants: Res<GameConstants>,
    color_constants: Res<ColorConstants>,
) {
    commands.spawn(Camera2d);

    spawn_test_npcs(&mut commands, &game_constants, &color_constants);
}



// --- Função Principal ---
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            CustomComponentsPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(RumorTimer(Timer::from_seconds(3.0, TimerMode::Once)))
        .insert_resource(GameConstants::default())
        .insert_resource(ColorConstants::default())
        .add_systems(Startup, setup_simulation)
        .add_systems(Update, (
            inject_rumor_system,
            (handle_rumor_spread, handle_social_interactions, color_system).chain(),
            movement_system,
            decay_basic_needs,
        ))
        .run();
}
