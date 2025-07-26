use bevy::prelude::*;

use crate::components::knowledge::KnowledgeBase;
use crate::components::needs::{Needs, Desire};
use crate::components::npc::{Npc, Personality};
use crate::components::resources::{RumorTimer, GameConstants, ColorConstants};

/// Plugin for registering all custom components with Bevy's reflection system
pub struct CustomComponentsPlugin;

impl Plugin for CustomComponentsPlugin {
    fn build(&self, app: &mut App) {
        app
            // NPC components
            .register_type::<Npc>()
            .register_type::<Personality>()
            // Knowledge components
            .register_type::<KnowledgeBase>()
            // Needs components
            .register_type::<Needs>()
            .register_type::<Desire>()
            // Resources
            .register_type::<RumorTimer>()
            .register_type::<GameConstants>()
            .register_type::<ColorConstants>()
        ;
    }
}