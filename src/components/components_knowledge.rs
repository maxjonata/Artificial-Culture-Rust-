use bevy::prelude::*;

/// Component representing an NPC's knowledge base
#[derive(Component, Reflect, PartialEq, Debug)]
#[reflect(Component)]
pub struct KnowledgeBase {
    /// Whether the NPC knows the rumor
    pub knows_rumor: bool,
}