use bevy::prelude::*;

/// Player movement speed factor.
pub const PLAYER_SPEED: f32 = 300.;

/// How quickly should the camera snap to the desired location.
pub const CAMERA_DECAY_RATE: f32 = 5.;
pub const PLAYER_ROTATION_SPEED: f32 = 10.0; // Rotation speed in radians per second

pub const SWING_RADIUS: f32 = 200.0;