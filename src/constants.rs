use bevy::prelude::*;

/// Player movement speed factor.
pub const PLAYER_SPEED: f32 = 300.;

/// How quickly should the camera snap to the desired location.
pub const CAMERA_DECAY_RATE: f32 = 5.;
pub const PLAYER_ROTATION_SPEED: f32 = 10.0; // Rotation speed in radians per second
pub const ATTACK_SPEED: f32 = 150.;

// Color
pub const WORLD_COLOR: Color = Color::srgb(0.2, 0.2, 0.3);
pub const PLAYER_COLOR: Color = Color::srgb(6.25, 9.4, 9.1);
pub const ENEMY_COLOR: Color = Color::srgb_u8(109, 119, 129);

pub const MESH_RADIUS: f32 = 25.0;

pub const LINER_DAMPING: f32 = 2.0;
pub const ANGULAR_DAMPING: f32 = 5.0;
