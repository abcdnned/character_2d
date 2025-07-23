use bevy::prelude::*;

pub fn input_map_to_move(
    sword: Single<(Entity, &mut Transform), With<crate::sword::Sword>>,
    mut action_events: EventReader<crate::input::ActionEvent>,
    mut move_events: EventWriter<crate::r#move::ExecuteMoveEvent>,
) {
    for _action_event in action_events.read() {
        move_events.write(crate::r#move::ExecuteMoveEvent {
            entity: sword.0,
            move_name: "SwingLeft".to_string(),
            move_input: crate::r#move::MoveInput::Attack,
        });
    }
}
