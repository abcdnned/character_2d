use bevy::prelude::*;

use crate::Player;

#[derive(Event)]
pub struct MoveEvent {
    pub direction: Vec2,
}

#[derive(Event)]
pub struct ActionEvent {
    pub entity: Entity,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveEvent>().add_event::<ActionEvent>();
    }
}

pub fn handle_input(
    player: Single<Entity, (With<Player>, With<Transform>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut move_events: EventWriter<MoveEvent>,
    mut action_events: EventWriter<ActionEvent>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        move_events.write(MoveEvent { direction });
    }

    if keyboard_input.just_pressed(KeyCode::KeyJ) {
        action_events.write(ActionEvent { entity: *player});
    }
}
