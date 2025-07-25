use bevy::prelude::*;
use std::collections::HashMap;
use crate::r#move::*;

use crate::constants::*;

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
            next_move: Some(SWING_RIGHT.to_string()),
        };

        let swing_right = MoveMetadata {
            name: SWING_RIGHT.to_string(),
            radius: 130.0,
            startup_time: 0.15,
            active_time: 0.15,
            recovery_time: 0.35,
            move_type: MoveType::Swing,
            accept_input: MoveInput::None,
            next_move: None,
        };

        moves.insert(SWING_LEFT.to_string(), swing_left);
        moves.insert(SWING_RIGHT.to_string(), swing_right);
        Self { moves }
    }
}