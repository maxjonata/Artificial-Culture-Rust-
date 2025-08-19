use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

pub fn spawn_perf_ui(mut commands: Commands) {
    commands.spawn(PerfUiAllEntries::default());
}