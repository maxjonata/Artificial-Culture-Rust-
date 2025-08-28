mod components;

use bevy::prelude::*;

/// Decision-making, memory, and belief systems for agent cognition.
/// Implements executive functions and rational thought processes.
/// Reference: Kahneman dual-process theory, Baddeley working memory model.
pub struct AiCognitionPlugin;

impl Plugin for AiCognitionPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Add domain-specific AI plugins as they are implemented
        // Future systems to be added:
        // - Working memory management system
        // - Long-term memory consolidation system
        // - Belief updating system
        // - Decision-making system
        // - Goal planning system
        // - Attention allocation system
        app
            .add_plugins(components::RegisterCognitionComponentsPlugin);
    }
}