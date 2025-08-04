use crate::ai::perception::vision::{VisionCone, VisualApparentState, VisuallyPerceived};
use crate::core::entities::Npc;
use bevy::prelude::*;

/// Resource to control vision debugging display
#[derive(Resource, Debug)]
pub struct VisionDebugSettings {
    /// Whether to show vision cones
    pub show_vision_cones: bool,
    /// Whether to show sight lines to perceived entities
    pub show_sight_lines: bool,
    /// Whether to show perception range circles
    pub show_range_circles: bool,
    /// Opacity for vision cone rendering (0.0-1.0)
    pub cone_opacity: f32,
    /// Color for vision cones
    pub cone_color: Color,
    /// Color for sight lines
    pub sight_line_color: Color,
}

impl Default for VisionDebugSettings {
    fn default() -> Self {
        Self {
            show_vision_cones: true,
            show_sight_lines: true,
            show_range_circles: false,
            cone_opacity: 0.3,
            cone_color: Color::srgba(0.2, 0.8, 0.2, 0.3), // Semi-transparent green
            sight_line_color: Color::srgba(1.0, 1.0, 0.0, 0.6), // Yellow sight lines
        }
    }
}

/// Component to mark entities as vision debug renderers
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct VisionDebugRenderer {
    pub owner: Entity,
}

/// System to render vision cones for NPCs
/// Based on your scientific grounding principle - visualizes the "Mantle of Ignorance"
pub fn vision_cone_debug_system(
    mut gizmos: Gizmos,
    settings: Res<VisionDebugSettings>,
    npc_query: Query<(Entity, &Transform, &VisionCone, &VisuallyPerceived), With<Npc>>,
    all_entities_query: Query<&Transform>,
) {
    if !settings.show_vision_cones && !settings.show_sight_lines && !settings.show_range_circles {
        return;
    }

    for (_entity, transform, vision_cone, perceived) in npc_query.iter() {
        let position = transform.translation.truncate();

        // Draw vision cone
        if settings.show_vision_cones {
            draw_vision_cone(&mut gizmos, position, vision_cone, &settings);
        }

        // Draw range circle
        if settings.show_range_circles {
            gizmos.circle_2d(position, vision_cone.max_range, settings.cone_color);
        }

        // Draw sight lines to perceived entities
        if settings.show_sight_lines {
            draw_sight_lines(&mut gizmos, position, perceived, &all_entities_query, &settings);
        }

        // Draw facing direction indicator
        let facing_end = position + Vec2::from_angle(vision_cone.facing_direction) * (vision_cone.max_range * 0.3);
        gizmos.line_2d(position, facing_end, Color::srgba(1.0, 0.0, 0.0, 0.8));
    }
}

/// Draw the vision cone using gizmos
fn draw_vision_cone(
    gizmos: &mut Gizmos,
    position: Vec2,
    vision_cone: &VisionCone,
    settings: &VisionDebugSettings,
) {
    let half_fov = vision_cone.fov_angle / 2.0;
    let segments = 16; // Number of line segments to approximate the arc

    // Calculate cone boundary points
    let left_angle = vision_cone.facing_direction - half_fov;
    let right_angle = vision_cone.facing_direction + half_fov;

    // Draw cone edges
    let left_end = position + Vec2::from_angle(left_angle) * vision_cone.max_range;
    let right_end = position + Vec2::from_angle(right_angle) * vision_cone.max_range;

    gizmos.line_2d(position, left_end, settings.cone_color);
    gizmos.line_2d(position, right_end, settings.cone_color);

    // Draw arc
    for i in 0..segments {
        let t1 = i as f32 / segments as f32;
        let t2 = (i + 1) as f32 / segments as f32;

        let angle1 = left_angle + t1 * vision_cone.fov_angle;
        let angle2 = left_angle + t2 * vision_cone.fov_angle;

        let point1 = position + Vec2::from_angle(angle1) * vision_cone.max_range;
        let point2 = position + Vec2::from_angle(angle2) * vision_cone.max_range;

        gizmos.line_2d(point1, point2, settings.cone_color);
    }
}

/// Draw sight lines to all perceived entities
fn draw_sight_lines(
    gizmos: &mut Gizmos,
    position: Vec2,
    perceived: &VisuallyPerceived,
    all_entities_query: &Query<&Transform>,
    settings: &VisionDebugSettings,
) {
    for (perceived_entity, _apparent_state) in &perceived.in_sight {
        // Get the position of the perceived entity
        if let Ok(target_transform) = all_entities_query.get(*perceived_entity) {
            let target_pos = target_transform.translation.truncate();
            // Draw line from observer to perceived entity
            gizmos.line_2d(position, target_pos, settings.sight_line_color);
            // Draw small circle at the perceived entity
            gizmos.circle_2d(target_pos, 3.0, settings.sight_line_color);
        }
    }

    // Indicate that this entity has vision capability with a small circle at the observer
    if !perceived.in_sight.is_empty() {
        gizmos.circle_2d(position, 8.0, Color::srgba(0.0, 1.0, 0.0, 0.8));
    }
}

/// System to update vision cone facing direction based on NPC movement
/// This makes the vision cone follow the NPC's movement direction
pub fn update_vision_facing_system(
    mut npc_query: Query<(&mut VisionCone, &bevy_rapier2d::prelude::Velocity), With<Npc>>,
) {
    for (mut vision_cone, velocity) in npc_query.iter_mut() {
        // Only update facing direction if the NPC is moving
        if velocity.linvel.length() > 10.0 {
            // Face the direction of movement
            vision_cone.facing_direction = velocity.linvel.to_angle();
        }
        // If not moving, keep the last facing direction
    }
}
