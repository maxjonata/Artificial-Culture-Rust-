pub mod cognition;
pub mod navigation;
pub mod perception;
pub mod physiology;
pub mod social;

use bevy::prelude::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            cognition::CognitionPlugin,
            navigation::NavigationPlugin,
            perception::PerceptionPlugin,
            physiology::PhysiologyPlugin,
            social::SocialPlugin,
        ));
    }
}
