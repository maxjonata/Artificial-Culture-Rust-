use bevy::math::Vec2;
use bevy::prelude::{Entity, Local, Query, Res, ResMut, Sprite, Time, Transform, Window, With};
use rand::prelude::IteratorRandom;
use rand::{rng, Rng};
use crate::components::default::{KnowledgeBase, Npc, Personality, RumorTimer, Velocity};
use crate::{NPC_RADIUS, NPC_SPEED, RED, SOCIAL_DISTANCE};

// Sistema de injeção de rumor
pub(crate) fn inject_rumor_system(
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

// Sistema de propagação
pub(crate) fn rumor_propagation_system(
    mut query: Query<(Entity, &Transform, &mut KnowledgeBase, &Personality)>,
    mut contact_history: Local<std::collections::HashMap<(Entity, Entity), bool>>,
) {
    // Collect all NPCs that know the rumor
    let mut rumor_spreaders = Vec::new();
    for (entity, transform, knowledge, _) in query.iter() {
        if knowledge.knows_rumor {
            rumor_spreaders.push((entity, transform.translation));
        }
    }

    // Track current contacts
    let mut current_contacts = std::collections::HashSet::new();

    // Track which NPCs should learn the rumor
    let mut to_learn_rumor = Vec::new();

    // First pass: check distances and collect information
    for (spreader_entity, spreader_pos) in &rumor_spreaders {
        for (target_entity, transform, knowledge, personality) in query.iter() {
            if *spreader_entity == target_entity { continue; }
            if knowledge.knows_rumor { continue; }

            let contact_key = (*spreader_entity, target_entity);
            let distance = spreader_pos.distance(transform.translation);

            if distance < SOCIAL_DISTANCE {
                // They are in contact now
                current_contacts.insert(contact_key);

                // Check if they were previously out of contact (coming back into range)
                let was_in_contact = contact_history.get(&contact_key).copied().unwrap_or(false);

                if !was_in_contact {
                    // Only attempt to spread rumor if they're coming back into range
                    let mut rng = rng();
                    if rng.random::<f32>() < personality.openness {
                        to_learn_rumor.push(target_entity);
                    }
                }
            }
        }
    }

    // Second pass: update knowledge for NPCs that should learn the rumor
    for target_entity in to_learn_rumor {
        if let Ok((_, _, mut knowledge, _)) = query.get_mut(target_entity) {
            knowledge.knows_rumor = true;
            println!("PROPAGAÇÃO: Um novo NPC aprendeu o rumor!");
        }
    }

    // Collect contacts that are no longer in range
    let contacts_out_of_range: Vec<_> = contact_history.keys()
        .filter(|key| !current_contacts.contains(key))
        .cloned()
        .collect();

    // Update contact history
    for key in contacts_out_of_range {
        contact_history.insert(key, false);
    }

    // Add new contacts to history
    for key in current_contacts {
        contact_history.insert(key, true);
    }
}

// Sistema de cores
pub(crate) fn color_system(
    mut query: Query<(&KnowledgeBase, &mut Sprite), With<Npc>>,
) {
    for (knowledge, mut sprite) in query.iter_mut() {
        if knowledge.knows_rumor {
            // Modificamos a propriedade .color diretamente no componente Sprite.
            sprite.color = RED;
        }
    }
}

// Sistema de movimento
pub(crate) fn movement_system(
    mut query: Query<(&mut Transform, &mut Velocity), With<Npc>>,
    time: Res<Time>,
    windows: Query<&Window>,
) {
    if let Some(window) = windows.iter().next() {
        let bounds = Vec2::new(window.width() / 2.0 - NPC_RADIUS, window.height() / 2.0 - NPC_RADIUS);
        for (mut transform, mut velocity) in query.iter_mut() {
            transform.translation += velocity.0.extend(0.0) * NPC_SPEED * time.delta_secs();
            if transform.translation.x < -bounds.x {
                transform.translation.x = -bounds.x;
                velocity.0.x *= -1.0;
            } else if transform.translation.x > bounds.x {
                transform.translation.x = bounds.x;
                velocity.0.x *= -1.0;
            }
            if transform.translation.y < -bounds.y {
                transform.translation.y = -bounds.y;
                velocity.0.y *= -1.0;
            } else if transform.translation.y > bounds.y {
                transform.translation.y = bounds.y;
                velocity.0.y *= -1.0;
            }
        }
    }
}