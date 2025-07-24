mod components;
mod systems;

use bevy::input::common_conditions::input_toggle_active;
// Usando Bevy 0.13.2
use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin,
    quick::WorldInspectorPlugin,
};
use rand::prelude::*;
use rand::rng;
use crate::components::default::*;
use crate::systems::*;

// --- Constantes da Simulação ---
const NUM_NPCS: usize = 20;
const NPC_RADIUS: f32 = 15.0;
const NPC_SPEED: f32 = 75.0;
const SOCIAL_DISTANCE: f32 = 100.0;

const RED: Color = Color::srgb(1.0, 0.2, 0.2);
const GREEN: Color = Color::srgb(0.2, 1.0, 0.2);

// --- Sistemas ---

fn setup_simulation(mut commands: Commands) {
    commands.spawn(Camera2d);

    let mut rng = rng();
    for _i in 0..NUM_NPCS {
        commands.spawn((
            components::default::Npc,
            Personality {
                openness: rng.random_range(0.0..0.5),
            },
            KnowledgeBase { knows_rumor: false },
            Velocity(Vec2::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0)).normalize_or_zero()),
            Sprite {
                color: GREEN,
                custom_size: Some(Vec2::splat(NPC_RADIUS * 2.0)),
                ..default()
            },
            Name::new(format!("NPC {}", _i + 1)),
        ));
    }
    println!("Simulação iniciada com {NUM_NPCS} NPCs.");
}



// --- Função Principal ---
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            CustomComponentsPlugin
        ))
        .insert_resource(RumorTimer(Timer::from_seconds(3.0, TimerMode::Once)))
        .add_systems(Startup, setup_simulation)
        .add_systems(Update, (
            update::inject_rumor_system,
            (update::rumor_propagation_system, update::color_system).chain(),
            update::movement_system,
        ))
        .run();
}
