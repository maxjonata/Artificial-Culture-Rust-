use bevy::prelude::*;

/// Component representing externally visible state of an entity
/// Based on Theory of Mind - what others can observe about an agent
/// CRITICAL: This contains ONLY externally apparent information, never internal state
#[derive(Component, Reflect, PartialEq, Debug, Clone, Copy)]
#[reflect(Component)]
pub struct VisualApparentState {
    /// Whether the entity appears to be running/moving quickly
    pub is_running: bool,
    /// Whether the entity appears to be carrying an item
    pub is_carrying_item: bool,
    /// General posture/stance of the entity
    pub posture: Posture,
    /// Whether the entity appears to be interacting with something
    pub is_interacting: bool,
}

/// Enum representing observable postures/stances
#[derive(Reflect, PartialEq, Debug, Clone, Copy, Default)]
pub enum Posture {
    /// Neutral, relaxed stance
    #[default]
    Neutral,
    /// Appears alert or vigilant
    Alert,
    /// Appears friendly or approachable
    Friendly,
    /// Appears defensive or wary
    Defensive,
    /// Appears aggressive or threatening
    Aggressive,
}

/// Component storing what an agent currently perceives about other entities
/// Based on Cognitive Psychology - perception is subjective and limited
/// CRITICAL: This is the agent's internal model of the world, not ground truth
#[derive(Component, Reflect, PartialEq, Debug)]
#[reflect(Component)]
pub struct VisuallyPerceived {
    /// Entities currently within visual range and their apparent state
    /// Limited by line-of-sight, distance, and attention
    pub in_sight: Vec<(Entity, VisualApparentState)>,
    /// Maximum number of entities that can be tracked simultaneously
    /// Based on cognitive attention limits (Miller's 7±2 rule)
    pub attention_limit: usize,
}

/// Component defining vision capabilities and parameters
/// Based on Human Visual Perception research and "Mantle of Ignorance" principle
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct VisionCone {
    /// Maximum distance the agent can see clearly (in pixels)
    pub max_range: f32,
    /// Field of view angle in radians (human ~3.14 radians = 180°)
    pub fov_angle: f32,
    /// Current facing direction in radians
    pub facing_direction: f32,
    /// Visual acuity - how well details can be discerned (0.0-1.0)
    pub acuity: f32,
    /// Whether vision is currently obstructed
    pub is_obstructed: bool,
}

impl Default for VisualApparentState {
    fn default() -> Self {
        Self {
            is_running: false,
            is_carrying_item: false,
            posture: Posture::default(),
            is_interacting: false,
        }
    }
}

impl Default for VisuallyPerceived {
    fn default() -> Self {
        Self {
            in_sight: Vec::new(),
            attention_limit: 7, // Miller's 7±2 rule
        }
    }
}

impl Default for VisionCone {
    fn default() -> Self {
        Self {
            max_range: 150.0,  // 150 pixel sight range
            fov_angle: std::f32::consts::FRAC_PI_2, // 180-degree field of view
            facing_direction: 0.0,
            acuity: 0.8,  // Good but not perfect vision
            is_obstructed: false,
        }
    }
}

/// System that handles visual perception for all agents
/// Based on "Mantle of Ignorance" - agents only know what they can see
pub fn vision_system(
    mut agents_query: Query<(Entity, &Transform, &VisionCone, &mut VisuallyPerceived)>,
    observable_query: Query<(Entity, &Transform, &VisualApparentState)>,
    mut perception_events: EventWriter<crate::ai::perception::attention::PerceptionChanged>,
) {
    for (agent_entity, agent_transform, vision_cone, mut perceived) in agents_query.iter_mut() {
        let agent_pos = agent_transform.translation.truncate();

        // Clear previous perception data
        perceived.in_sight.clear();

        // Check what this agent can see
        for (target_entity, target_transform, apparent_state) in observable_query.iter() {
            // Skip self-perception
            if target_entity == agent_entity {
                continue;
            }

            let target_pos = target_transform.translation.truncate();
            let distance = agent_pos.distance(target_pos);

            // Check if target is within vision range
            if distance <= vision_cone.max_range && distance > 0.1 { // Avoid division by zero
                // Check if target is within field of view
                let direction_to_target = (target_pos - agent_pos).normalize();
                let facing_vector = Vec2::new(
                    vision_cone.facing_direction.cos(),
                    vision_cone.facing_direction.sin(),
                );

                let angle_to_target = direction_to_target.dot(facing_vector).acos();

                if angle_to_target <= vision_cone.fov_angle / 2.0 {
                    // Target is visible - add to perceived entities
                    if perceived.in_sight.len() < perceived.attention_limit {
                        perceived.in_sight.push((target_entity, *apparent_state));
                    }

                    // Calculate salience for attention system
                    let salience = calculate_visual_salience(
                        distance,
                        vision_cone.max_range,
                        apparent_state,
                        vision_cone.acuity,
                    );

                    // Send perception event for attention system
                    perception_events.send(crate::ai::perception::attention::PerceptionChanged {
                        observer: agent_entity,
                        perceived_entity: target_entity,
                        entered: true,
                        salience,
                    });
                }
            }
        }
    }
}

/// Calculate how visually salient an entity is
/// Based on visual attention research and salience mapping
fn calculate_visual_salience(
    distance: f32,
    max_range: f32,
    apparent_state: &VisualApparentState,
    acuity: f32,
) -> f32 {
    // Base salience decreases with distance
    let distance_factor = 1.0 - (distance / max_range);
    let mut salience = distance_factor * acuity;

    // Movement attracts attention
    if apparent_state.is_running {
        salience += 0.3;
    }

    // Carrying items is noticeable
    if apparent_state.is_carrying_item {
        salience += 0.2;
    }

    // Different postures have different salience
    salience += match apparent_state.posture {
        Posture::Aggressive => 0.5,  // Threatening = high salience
        Posture::Alert => 0.3,
        Posture::Defensive => 0.2,
        Posture::Friendly => 0.1,
        Posture::Neutral => 0.0,
    };

    // Interacting entities draw attention
    if apparent_state.is_interacting {
        salience += 0.2;
    }

    salience.clamp(0.0, 1.0)
}
