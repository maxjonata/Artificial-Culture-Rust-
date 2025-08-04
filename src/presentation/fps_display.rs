use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

/// Component that marks an entity as an FPS display UI element
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct FpsDisplay;

/// Component that stores FPS calculation data
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct FpsData {
    /// Current frames per second
    pub fps: f32,
    /// Current delta time in seconds
    pub delta_time: f32,
    /// Update timer to avoid updating UI every frame
    pub update_timer: Timer,
}

impl Default for FpsData {
    fn default() -> Self {
        Self {
            fps: 0.0,
            delta_time: 0.0,
            // Update the display every 0.1 seconds for smooth but not overwhelming updates
            update_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

/// System to spawn the FPS display UI at startup
/// Based on Human-Computer Interaction principles - non-intrusive performance monitoring
pub fn spawn_fps_display_system(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)), // Semi-transparent black background
            FpsDisplay,
            FpsData::default(),
        ))
        .with_children(|parent| {
            // FPS text display
            parent.spawn((
                Text::new("FPS: --\nΔt: -- ms"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)), // Light gray text
            ));
        });
}

/// System to update the FPS display with current performance metrics
/// Based on Real-time Systems principles - monitoring system performance
pub fn update_fps_display_system(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut fps_query: Query<(&mut FpsData, &Children), With<FpsDisplay>>,
    mut text_query: Query<&mut Text>,
) {
    for (mut fps_data, children) in fps_query.iter_mut() {
        // Update the timer
        fps_data.update_timer.tick(time.delta());

        // Only update display periodically to avoid visual jitter
        if fps_data.update_timer.just_finished() {
            // Get current delta time from Bevy's Time resource
            fps_data.delta_time = time.delta_secs();

            // Calculate FPS from delta time (more accurate than diagnostics for real-time display)
            fps_data.fps = if fps_data.delta_time > 0.0 {
                1.0 / fps_data.delta_time
            } else {
                0.0
            };

            // Alternatively, use Bevy's diagnostics for smoothed FPS if available
            if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(smoothed_fps) = fps_diagnostic.smoothed() {
                    fps_data.fps = smoothed_fps as f32;
                }
            }

            // Update the text display
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.0 = format!(
                        "FPS: {:.1}\nΔt: {:.2} ms",
                        fps_data.fps,
                        fps_data.delta_time * 1000.0 // Convert to milliseconds for better readability
                    );
                }
            }
        }
    }
}
