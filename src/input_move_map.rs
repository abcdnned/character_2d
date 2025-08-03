use bevy::prelude::*;

use crate::{global_entity_map::GlobalEntityMap, weapon::Weapon};

pub fn input_map_to_move(
    mut action_events: EventReader<crate::input::ActionEvent>,
    mut move_events: EventWriter<crate::r#move::ExecuteMoveEvent>,
    global_entities: ResMut<GlobalEntityMap>,
) {
    for action_event in action_events.read() {
        if let Some(weapon) = global_entities.player_weapon.get(&action_event.entity) {
            move_events.write(crate::r#move::ExecuteMoveEvent {
                entity: *weapon,
                move_name: "SwingLeft".to_string(),
                move_input: crate::r#move::MoveInput::Attack,
            });
        }
    }
}
