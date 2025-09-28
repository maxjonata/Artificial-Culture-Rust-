use bevy::prelude::{Component, Plugin, Reflect, ReflectComponent};

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct AgentEvent {
    description: String,
    impact: f32,
}

pub struct SocialComponentsPlugin;

impl Plugin for SocialComponentsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .register_type::<AgentEvent>();
    }
}