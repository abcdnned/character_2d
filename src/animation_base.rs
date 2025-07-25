use crate::constants::*;
use bevy::prelude::*;
use std::collections::HashMap;
use crate::lerp_animation::calculate_vertical_swing_cubic;

pub type AnimationFunction = fn(f32, f32) -> (Vec2, f32);

#[derive(Resource)]
pub struct AnimationDatabase {
    pub animations: HashMap<String, AnimationFunction>,
}

impl Default for AnimationDatabase {
    fn default() -> Self {
        let mut animations = HashMap::new();
        animations.insert(SWING_LEFT.to_string(), calculate_vertical_swing_cubic as AnimationFunction);
        Self { animations }
    }
}

pub struct AnimationDatabasePlugin;

impl Plugin for AnimationDatabasePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AnimationDatabase>();
    }
}
