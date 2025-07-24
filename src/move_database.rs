use bevy::prelude::*;
use std::collections::HashMap;
use crate::r#move::*;

use crate::constants::SWING_LEFT;

#[derive(Resource)]
pub struct MoveDatabase {
    pub moves: HashMap<String, MoveMetadata>,
}

impl Default for MoveDatabase {
    fn default() -> Self {
        let mut moves = HashMap::new();

        let swing_left = MoveMetadata {
            name: SWING_LEFT.to_string(),
            radius: 130.0,
            startup_time: 0.15,
            active_time: 0.15,
            recovery_time: 0.35,
            move_type: MoveType::Swing,
            accept_input: MoveInput::Attack,
            next_move: Some(SWING_LEFT.to_string()),
        };

        moves.insert(SWING_LEFT.to_string(), swing_left);
        Self { moves }
    }
}