pub mod needs;        // Physiological needs and homeostatic drive
pub mod states;
pub mod systems;

use bevy::prelude::*;

pub struct PhysiologyPlugin;

impl Plugin for PhysiologyPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register components from this domain
            .register_type::<needs::BasicNeeds>()
            .register_type::<needs::DesireThresholds>()
            // Register events
            .add_event::<needs::NeedThresholdCrossed>()
            // Add systems from this domain
            .add_systems(Update, (
                needs::needs_decay_system,
            ));
    }
}
