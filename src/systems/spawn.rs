use crate::components::physiological::{BasicNeeds, BodyInjuries};
use crate::types::{BodyPart, InjuryType, PhysiologicalNeeds};
use crate::utils::generics::{UniqueIdentity, get_random_weight};
use bevy_ecs::system::Commands;
use rand::Rng;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub(crate) fn spawn_random_being(mut commands: Commands) {
    let mut needs = HashMap::new();
    for physiological_need in PhysiologicalNeeds::iter() {
        needs.insert(physiological_need, get_random_weight());
    }

    let mut injuries = HashMap::new();
    for body_part in BodyPart::iter() {
        let mut injuries_for_each_body_part = HashMap::new();
        for injury_type in InjuryType::iter() {
            injuries_for_each_body_part.insert(injury_type, get_random_weight());
        }
        injuries.insert(body_part, injuries_for_each_body_part);
    }

    let mut rng = rand::rng();
    let unique_id = rng.random();

    let bundle = (
        BasicNeeds { needs },
        BodyInjuries { injuries },
        UniqueIdentity {
            entity_id: unique_id,
            name: unique_id.to_string(),
            description: unique_id.to_string(),
        },
    );
    
    commands.spawn(bundle);
}
