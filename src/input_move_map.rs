use bevy::prelude::*;

use crate::{constants::*, global_entity_map::GlobalEntityMap, weapon::Weapon};

pub fn input_map_to_move(
    mut action_events: EventReader<crate::input::ActionEvent>,
    mut move_events: EventWriter<crate::r#move::ExecuteMoveEvent>,
    global_entities: ResMut<GlobalEntityMap>,
) {
    for action_event in action_events.read() {
        if let Some(weapon) = global_entities.player_weapon.get(&action_event.entity) {
            let (move_name, move_input) = match action_event.action_type {
                ACTION_HENG => (SWING_LEFT.to_string(), crate::r#move::MoveInput::Attack),
                ACTION_ZHAN => (SWORD_STUB.to_string(), crate::r#move::MoveInput::Attack),
                _ => continue, // Skip unknown action types
            };

            move_events.write(crate::r#move::ExecuteMoveEvent {
                entity: *weapon,
                move_name,
                move_input,
            });
        }
    }
}