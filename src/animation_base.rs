use crate::constants::*;
use crate::lerp_animation::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub type AnimationFunction = fn(f32, f32) -> (Vec2, f32);

#[derive(Resource)]
pub struct AnimationDatabase {
    pub animations: HashMap<String, AnimationFunction>,
}

impl Default for AnimationDatabase {
    fn default() -> Self {
        let mut animations = HashMap::new();
        animations.insert(
            SWING_LEFT.to_string(),
            calculate_left_swing_cubic as AnimationFunction,
        );
        animations.insert(
            SWING_RIGHT.to_string(),
            calculate_right_swing_cubic as AnimationFunction,
        );
        animations.insert(
            SWORD_STUB.to_string(),
            calculate_stub_cubic as AnimationFunction,
        );
        Self { animations }
    }
}

pub struct AnimationDatabasePlugin;

impl Plugin for AnimationDatabasePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AnimationDatabase>();
    }
}
