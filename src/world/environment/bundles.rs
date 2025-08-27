use crate::world::environment::components::{
    EnvironmentBounds, 
    EnvironmentFeature
};
use bevy::math::{Quat, Vec2};
use bevy::prelude::{
    Bundle,
    GlobalTransform,
    InheritedVisibility,
    Sprite,
    Transform,
    ViewVisibility,
    Visibility
};
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::Collider;

/// Complete environment bounds entity.
#[derive(Bundle)]
pub struct EnvironmentBoundsBundle {
    pub bounds          : EnvironmentBounds,
    pub transform       : Transform,
    pub global_transform: GlobalTransform,
    pub rigid_body      : RigidBody,
    pub collider        : Collider,
}

/// Environment feature with PNG texture and physics.
#[derive(Bundle)]
pub struct EnvironmentFeatureBundle {
    pub feature             : EnvironmentFeature,
    pub sprite              : Sprite,
    pub transform           : Transform,
    pub global_transform    : GlobalTransform,
    pub visibility          : Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility     : ViewVisibility,
    pub rigid_body          : RigidBody,
    pub collider            : Collider,
}

/// Simple wall entity.
#[derive(Bundle)]
pub struct WallBundle {
    pub transform       : Transform,
    pub global_transform: GlobalTransform,
    pub rigid_body      : RigidBody,
    pub collider        : Collider,
}

impl WallBundle {
    pub fn new(center: Vec2, angle: f32, length: f32) -> Self {
        Self {
            transform                                       : Transform::from_translation(center.extend(0.0))
                                                                        .with_rotation(Quat::from_rotation_z(angle)),
            global_transform                                : GlobalTransform::default(),
            rigid_body                                      : RigidBody::Fixed,
            collider                                        : Collider::cuboid(length * 0.5, 2.5),
        }
    }
}