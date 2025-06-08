use crate::components::physiological::{BasicNeeds, BodyInjuries};
use crate::utils::generics::UniqueIdentity;
use bevy_ecs::prelude::*;

pub(crate) fn basic_needs_decrease_system(mut query: Query<&mut BasicNeeds>) {
    query.iter_mut().for_each(|mut basic_needs| {
        for (need, value) in basic_needs.needs.iter_mut() {
            *value = (*value - 0.1).clamp(0.0, 1.0);
            if *value < 0.2 {
                eprintln!("Warning: {:#?} is critically low!", need);
            }
        }
    })
}

pub(crate) fn check_injuries_system(query: Query<(&BodyInjuries, &UniqueIdentity)>) {
    query.iter().for_each(|(body_injuries, unique_identity)| {
        for (body_part, injuries) in body_injuries.injuries.iter() {
            for (injury_type, severity) in injuries.iter() {
                if *severity < 0.5 {
                    eprintln!(
                        "Warning: {} has a severe injury of type {:?} on {:?}!",
                        unique_identity.name, injury_type, body_part
                    );
                }
            }
        }
    });
}
