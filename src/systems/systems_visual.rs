use bevy::prelude::*;

use crate::components::components_resources::ColorConstants;
use crate::components::components_knowledge::KnowledgeBase;
use crate::components::components_npc::Npc;

/// System for updating NPC colors based on rumor knowledge
pub fn color_system(
    mut query: Query<(&KnowledgeBase, &mut Sprite), With<Npc>>,
    color_constants: Res<ColorConstants>,
) {
    for (knowledge, mut sprite) in query.iter_mut() {
        if knowledge.knows_rumor {
            // Update sprite color to RED if NPC knows the rumor
            sprite.color = color_constants.red;
        }
    }
}