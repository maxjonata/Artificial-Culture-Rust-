use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::core::constants::GameConstants;

/// Configuration for debug UI features
#[derive(Resource, Debug)]
pub struct DebugUiConfig {
    pub show_world_inspector: bool,
    pub inspector_filter_mode: InspectorFilterMode,
}

#[derive(Debug)]
pub enum InspectorFilterMode {
    All,
    AiComponentsOnly,
    PerformanceOnly,
}

impl Default for DebugUiConfig {
    fn default() -> Self {
        Self {
            show_world_inspector: false, // Start hidden to reduce overhead
            inspector_filter_mode: InspectorFilterMode::AiComponentsOnly,
        }
    }
}

/// Toggle debug UI visibility
pub fn toggle_debug_ui(keyboard: Res<ButtonInput<KeyCode>>, mut config: ResMut<DebugUiConfig>) {
    if keyboard.just_pressed(KeyCode::F1) {
        config.show_world_inspector = !config.show_world_inspector;
        info!(
            "Debug UI toggled: {}",
            if config.show_world_inspector {
                "ON"
            } else {
                "OFF"
            }
        );
    }
}

pub fn time_ui(
    mut contexts: EguiContexts,
    mut time: ResMut<Time<Virtual>>,
    mut constants: ResMut<GameConstants>,
) -> Result {
    egui::Window::new("Time Control").show(contexts.ctx_mut()?, |ui| {
        let hours = time.elapsed_secs() / 3600.0;
        let minutes = (time.elapsed_secs() % 3600.0) / 60.0;
        let seconds = time.elapsed_secs() % 60.0;

        ui.label(format!("Time: [{hours:.0}:{minutes:.0}:{seconds:.0}]"));
        ui.horizontal(|ui| {
            let mut reverse = constants.time_speed < 0.0;
            if ui.checkbox(&mut reverse, "Rev").clicked() {
                constants.time_speed = if reverse { -1.0 } else { 1.0 };
            }
            ui.add(
                egui::Slider::new(&mut constants.time_speed, 0.0..=3.0)
                    .text("Speed")
                    .step_by(1.0),
            );
        });
    });

    time.set_relative_speed(constants.time_speed);
    Ok(())
}

fn time_speed(constants: ResMut<GameConstants>, mut time: ResMut<Time<Virtual>>) {
    time.set_relative_speed(constants.time_speed);
}

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .init_resource::<DebugUiConfig>()
            .add_systems(EguiPrimaryContextPass, time_ui)
            .add_systems(Update, (toggle_debug_ui, time_speed));

        // Only add WorldInspectorPlugin in debug builds to reduce overhead
        #[cfg(debug_assertions)]
        app.add_plugins(WorldInspectorPlugin::default());
    }
}
