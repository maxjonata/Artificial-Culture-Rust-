use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .init_resource::<DebugUiConfig>()
            .add_systems(Update, toggle_debug_ui);

        // Only add WorldInspectorPlugin in debug builds to reduce overhead
        #[cfg(debug_assertions)]
        app.add_plugins(WorldInspectorPlugin::default());
    }
}
