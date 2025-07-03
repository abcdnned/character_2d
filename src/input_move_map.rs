use bevy::prelude::*;

fn input_map_to_move(
    mut player: Single<&mut Transform, With<crate::Player>>,
    mut action_events: EventReader<crate::input::ActionEvent>,
    mut move_events: EventWriter<crate::r#move::ExecuteMoveEvent>,
) {
    // fire an ExecuteMoveEvent
}