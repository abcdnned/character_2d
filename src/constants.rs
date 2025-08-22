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

pub const SWING_LEFT: &str = "SwingLeft";
pub const SWING_RIGHT: &str = "SwingRight";
pub const SWORD_STUB: &str = "SwordStub";
pub const REFLECT: &str = "Reflect";

pub const KNOCK_BACK_LITE: f32 = 300.0;
pub const DURATION_FACTOR: f32 = 2.25 / 800.0;

pub const DEFAULT_SPEED: f32 = 180.0;
pub const DEFAULT_MAX_HP: f32 = 100.0;

pub const ACTION_HENG: u32 = 1;
pub const ACTION_ZHAN: u32 = 2;

pub const FORCE_PLAYER: u32 = 0;
pub const FORCE_ENEMY: u32 = 1;

pub const ALERT_RANGE: f32 = 1000.0;
pub const DIS_ALERT_RANGE: f32 = 2000.0;
pub const STOP_CHASING_RANGE: f32 = 250.0;
