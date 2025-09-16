use bevy::prelude::*;

use crate::{berserker::Berserker, constants::*, global_entity_map::GlobalEntityMap, weapon::Weapon};

pub fn input_map_to_move(
    mut action_events: EventReader<crate::input::ActionEvent>,
    mut move_events: EventWriter<crate::custom_move::ExecuteMoveEvent>,
    global_entities: ResMut<GlobalEntityMap>,
    berserker_query: Query<&Berserker>,
) {
    for action_event in action_events.read() {
        if let Some(weapon) = global_entities.player_weapon.get(&action_event.entity) {
            let (move_name, move_input) = match action_event.action_type {
                ACTION_HENG => (
                    SWING_LEFT.to_string(),
                    crate::custom_move::MoveInput::Attack,
                ),
                ACTION_ZHAN => (
                    SWORD_STUB.to_string(),
                    crate::custom_move::MoveInput::Attack,
                ),
                ACTION_SPECIAL => {
                    // Check if the entity has Berserker component and is level 1
                    if let Ok(berserker) = berserker_query.get(action_event.entity) {
                        if berserker.level == 1 {
                            (TUNADO.to_string(), crate::custom_move::MoveInput::Attack)
                        } else {
                            (SPIN_LEFT.to_string(), crate::custom_move::MoveInput::Attack)
                        }
                    } else {
                        (SPIN_LEFT.to_string(), crate::custom_move::MoveInput::Attack)
                    }
                },
                _ => continue, // Skip unknown action types
            };

            move_events.write(crate::custom_move::ExecuteMoveEvent {
                entity: *weapon,
                move_name,
                move_input,
            });
        }
    }
}
