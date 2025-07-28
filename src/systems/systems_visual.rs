use bevy::prelude::*;

use crate::components::components_knowledge::KnowledgeBase;
use crate::components::components_npc::Npc;

/// System for updating NPC sprites based on rumor knowledge
/// System based on Visual Information Theory - visual cues affect social perception
pub fn color_system(
    mut query: Query<(&KnowledgeBase, &mut Sprite), With<Npc>>,
    asset_server: Res<AssetServer>,
) {
    for (knowledge, mut sprite) in query.iter_mut() {
        if knowledge.knows_rumor {
            // ML-HOOK: Visual state change provides quantifiable feedback for RL agents
            // Change to contaminated person sprite when NPC knows the rumor
            sprite.image = asset_server.load("contamined_person.png");
        }
        // else {
        //     // Use normal person sprite when NPC doesn't know the rumor
        //     sprite.image = asset_server.load("person.png");
        // }
    }
}