use bevy::prelude::*;

use crate::weapon::Weapon;

pub fn input_map_to_move(
    weapons: Query<Entity, With<crate::weapon::Weapon>>,
    mut action_events: EventReader<crate::input::ActionEvent>,
    mut move_events: EventWriter<crate::r#move::ExecuteMoveEvent>,
) {
    for action_event in action_events.read() {
        move_events.write(crate::r#move::ExecuteMoveEvent {
            entity: action_event.entity,
            move_name: "SwingLeft".to_string(),
            move_input: crate::r#move::MoveInput::Attack,
        });
    }
}
