use crate::custom_move::*;
use bevy::prelude::*;
use std::collections::HashMap;

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
            active_time: ATTACK_ANIMATION_SPEED,
            recovery_time: 0.35,
            move_type: MoveType::Swing,
            accept_input: MoveInput::Attack,
            next_move: Some(SWING_RIGHT.to_string()),
            kb_force: KNOCK_BACK_LITE,
            critical_rate: BASE_CRITICAL_RATE,
            best_range_min: 150.0,
            move_speed: PLAYER_SPEED,
        };

        let swing_right = MoveMetadata {
            name: SWING_RIGHT.to_string(),
            radius: 130.0,
            startup_time: 0.15,
            active_time: ATTACK_ANIMATION_SPEED,
            recovery_time: 0.35,
            move_type: MoveType::Swing,
            accept_input: MoveInput::None,
            next_move: None,
            kb_force: KNOCK_BACK_LITE,
            critical_rate: BASE_CRITICAL_RATE + CRIT_A,
            best_range_min: 150.0,
            move_speed: PLAYER_SPEED,
        };

        let sword_stub = MoveMetadata {
            name: SWORD_STUB.to_string(),
            radius: 130.0,
            startup_time: 0.30,
            active_time: ATTACK_ANIMATION_SPEED * 1.3,
            recovery_time: 0.35,
            move_type: MoveType::Stub,
            accept_input: MoveInput::None,
            next_move: None,
            kb_force: KNOCK_BACK_LITE,
            critical_rate: BASE_CRITICAL_RATE + CRIT_B,
            best_range_min: 170.0,
            move_speed: PLAYER_SPEED,
        };

        let reflect = MoveMetadata {
            name: REFLECT.to_string(),
            radius: 130.0,
            startup_time: 0.00,
            active_time: 1.00,
            recovery_time: 0.00,
            move_type: MoveType::Interrupt,
            accept_input: MoveInput::Interrupt,
            next_move: None,
            kb_force: KNOCK_BACK_NONE,
            critical_rate: 0.0,
            best_range_min: 150.0,
            move_speed: PLAYER_SPEED,
        };

        let spin_left = MoveMetadata {
            name: SPIN_LEFT.to_string(),
            radius: 130.0,
            startup_time: 0.0,
            active_time: ATTACK_ANIMATION_SPEED * 2.5,
            recovery_time: 1.2,
            move_type: MoveType::Swing,
            accept_input: MoveInput::None,
            next_move: None,
            kb_force: KNOCK_BACK_HEAVY,
            critical_rate: BASE_CRITICAL_RATE * 2.0,
            best_range_min: 150.0,
            move_speed: PLAYER_SPEED,
        };

        let tunado = MoveMetadata {
            name: TUNADO.to_string(),
            radius: 130.0,
            startup_time: 0.0,
            active_time: ATTACK_ANIMATION_SPEED * 1.0,
            recovery_time: 2.0,
            move_type: MoveType::Swing,
            accept_input: MoveInput::Attack,
            next_move: Some(TUNADO.to_string()),
            kb_force: KNOCK_BACK_LITE,
            critical_rate: BASE_CRITICAL_RATE * 2.0,
            best_range_min: 150.0,
            move_speed: (PLAYER_SPEED + BERSERKER_MOVE_SPEED) * 1.1,
        };

        moves.insert(SWING_LEFT.to_string(), swing_left);
        moves.insert(SWING_RIGHT.to_string(), swing_right);
        moves.insert(SWORD_STUB.to_string(), sword_stub);
        moves.insert(REFLECT.to_string(), reflect);
        moves.insert(SPIN_LEFT.to_string(), spin_left);
        moves.insert(TUNADO.to_string(), tunado);
        Self { moves }
    }
}
