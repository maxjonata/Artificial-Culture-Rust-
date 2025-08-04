pub mod attention;    // Selective attention and cognitive focus
pub mod vision;

use bevy::prelude::*;

pub struct PerceptionPlugin;

impl Plugin for PerceptionPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register components from this domain
            .register_type::<vision::VisionCone>()
            .register_type::<vision::VisualApparentState>()
            .register_type::<vision::VisuallyPerceived>()
            .register_type::<vision::Posture>()
            .register_type::<attention::Attention>()
            .register_type::<attention::PerceivedEntities>()
            // Register events
            .add_event::<attention::AttentionShifted>()
            .add_event::<attention::PerceptionChanged>()
            // Add systems from this domain
            .add_systems(Update, (
                vision::vision_system,
                attention::attention_system,
            ));
    }
}
