use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::IteratorRandom;
use rand::{rng, Rng};

use crate::components::knowledge::KnowledgeBase;
use crate::components::npc::Npc;
use crate::components::npc::Personality;
use crate::components::resources::RumorTimer;

/// System for injecting a rumor into a random NPC
pub fn inject_rumor_system(
    mut q_npcs: Query<&mut KnowledgeBase, With<Npc>>,
    time: Res<Time>,
    mut timer: ResMut<RumorTimer>,
    mut injected: Local<bool>,
) {
    if *injected { return; }
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rng();
        if let Some(mut knowledge) = q_npcs.iter_mut().choose(&mut rng) {
            knowledge.knows_rumor = true;
            *injected = true;
            println!("RUMOR INJETADO! Um NPC agora sabe o segredo.");
        }
    }
}

/// System for handling rumor spread during collisions
pub fn handle_rumor_spread(
    mut collision_event: EventReader<CollisionEvent>,
    mut query: Query<(&mut KnowledgeBase, &Personality), With<Npc>>
) {
    for collision_event in collision_event.read() {
        if let CollisionEvent::Started(entity1, entity2, _flags) = collision_event {
            let mut entities = query.get_many_mut([*entity1, *entity2]).unwrap();
            let (entity_1, entity_2) = entities.split_at_mut(1);

            if first_knows_rumor_and_has_bigger_personality(&mut entity_1[0], &mut entity_2[0]) {
                entity_2[0].0.knows_rumor = true;
            } else if first_knows_rumor_and_has_bigger_personality(&mut entity_2[0], &mut entity_1[0]) {
                entity_1[0].0.knows_rumor = true;
            }
        }
    }
}

/// Helper function to determine if rumor should spread from first entity to second
fn first_knows_rumor_and_has_bigger_personality(
    first: &mut (Mut<KnowledgeBase>, &Personality), 
    second: &mut (Mut<KnowledgeBase>, &Personality)
) -> bool {
    (first.0.knows_rumor)
    && (!second.0.knows_rumor)
    && (rng().random_range(0.0 ..1.0) < second.1.openness)
}