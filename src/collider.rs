use crate::Player;
use crate::constants::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct DynamicPhysicsBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub gravity_scale: GravityScale,
    pub damping: Damping,
}

impl DynamicPhysicsBundle {
    /// Creates a new dynamic bundle with a box collider
    pub fn new_box(half_width: f32, half_height: f32) -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(half_width, half_height),
            gravity_scale: GravityScale(0.0),
            damping: Damping {
                linear_damping: LINER_DAMPING,
                angular_damping: ANGULAR_DAMPING,
            },
        }
    }

    /// Creates a new dynamic bundle with a ball collider
    pub fn new_ball(radius: f32) -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(radius),
            gravity_scale: GravityScale(0.0),
            damping: Damping {
                linear_damping: LINER_DAMPING,
                angular_damping: ANGULAR_DAMPING,
            },
        }
    }
}
