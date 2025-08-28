use crate::core::types::Normalized;
use bevy::prelude::{App, Component, Plugin, Reflect, ReflectComponent};

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
struct PersonalityVector {
    openness: Normalized,
    conscientiousness: Normalized,
    extraversion: Normalized,
    agreeableness: Normalized,
    neuroticism: Normalized,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
struct RoleAffinities {
    leadership_tendency: Normalized,
    cooperation_drive: Normalized,
    exploration_urge: Normalized,
    protection_instinct: Normalized,
}

pub struct RegisterCognitionComponentsPlugin;

impl Plugin for RegisterCognitionComponentsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PersonalityVector>()
            .register_type::<RoleAffinities>();
    }
}