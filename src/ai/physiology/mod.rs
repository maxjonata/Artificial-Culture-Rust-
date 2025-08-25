use bevy::prelude::*;

/// Physiological needs, stress responses, and homeostatic regulation.
/// Implements basic biological drives that motivate agent behavior.
/// Reference: Sterling & Eyer allostasis theory, Maslow hierarchy adaptation.
pub struct AiPhysiologyPlugin;

impl Plugin for AiPhysiologyPlugin {

    fn build(&self, app: &mut App) {
        // TODO: Add domain-specific AI plugins as they are implemented
        // Future systems to be added:
        // - Basic needs management system (hunger, thirst, energy, sleep)
        // - Stress and allostatic load system
        // - Health and immune system
        // - Circadian rhythm system
        // - Metabolic and aging system
        // - Emotional regulation system
        // - Pain and nociception system
    }
}
