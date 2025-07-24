use bevy::prelude::*;
use bevy::reflect::TypeRegistry;

pub struct CustomComponentsPlugin;

impl Plugin for CustomComponentsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Npc>()
            .register_type::<Personality>()
            .register_type::<KnowledgeBase>()
            .register_type::<Velocity>()
            .register_type::<RumorTimer>()
        ;
    }
}


// --- Componentes ---
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Npc;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Personality {
    pub openness: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct KnowledgeBase {
    pub knows_rumor: bool,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Velocity(pub(crate) Vec2);

// --- Recurso ---
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct RumorTimer(pub Timer);