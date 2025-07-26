mod components;
mod systems;

use bevy::input::common_conditions::input_toggle_active;
// Usando Bevy 0.13.2
use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin,
    quick::WorldInspectorPlugin,
};
use bevy_rapier2d::prelude::*;
// Removed conflicting import
use rand::prelude::*;
use rand::rng;
use crate::components::*;
use crate::components::default::CustomComponentsPlugin;
use crate::components::resources::{GameConstants, ColorConstants};
use crate::systems::rumor::{inject_rumor_system, handle_rumor_spread};
use crate::systems::movement::movement_system;
use crate::systems::visual::color_system;

// --- Sistemas ---

fn setup_simulation(
    mut commands: Commands,
    game_constants: Res<GameConstants>,
    color_constants: Res<ColorConstants>,
) {
    commands.spawn(Camera2d);

    let mut rng = rng();
    for _i in 0..game_constants.num_npcs {
        let position = Vec2::new(
            rng.random_range(-400.0..400.0),
            rng.random_range(-300.0..300.0)
        );
        
        commands.spawn((
            Npc,
            Personality {
                openness: rng.random_range(0.0..0.5),
            },
            KnowledgeBase { knows_rumor: false },
            Sprite {
                color: color_constants.green,
                custom_size: Some(Vec2::splat(game_constants.npc_radius * 2.0)),
                
                ..default()
            },
            Name::new(format!("NPC {}", _i + 1)),
            Transform::from_xyz(position.x, position.y, 0.0),
            RigidBody::Dynamic,
            GravityScale(0.0), // Disable gravity for the top-down view
            Collider::ball(game_constants.npc_radius),
            Restitution::coefficient(0.7),
            Friction::coefficient(0.2),
            // Use Rapier's Velocity component instead of ExternalForce
            Velocity {
                linvel: Vec2::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0)).normalize_or_zero() * game_constants.npc_speed * 0.01,
                angvel: 0.0,
            },
            Damping { linear_damping: 0.5, angular_damping: 0.5 }, // Add damping to prevent excessive movement
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
    println!("Simulação iniciada com {} NPCs.", game_constants.num_npcs);
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
            (handle_rumor_spread, color_system).chain(),
            movement_system,
        ))
        .run();
}
