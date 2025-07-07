use bevy::prelude::*;

pub fn input_map_to_move(
    player: Single<(Entity, &mut Transform), With<crate::sword::Sword>>,
    mut action_events: EventReader<crate::input::ActionEvent>,
    mut move_events: EventWriter<crate::r#move::ExecuteMoveEvent>,
) {
    for _action_event in action_events.read() {
        move_events.write(crate::r#move::ExecuteMoveEvent {
            entity: player.0,
            move_name: "SwingLeft".to_string(),
        });
    }
}