use crate::{ai::TargetDetector, constants::*};
use bevy::prelude::*;

pub fn move_player(
    mut player: Single<(Entity, &mut Transform, &TargetDetector), With<crate::Player>>,
    time: Res<Time>,
    mut move_events: EventReader<crate::input::MoveEvent>,
    move_query: Query<&crate::r#move::PlayerMove, With<crate::Player>>,
) {
    let mut direction = Vec2::ZERO;

    // Accumulate all movement events this frame
    for event in move_events.read() {
        direction += event.direction;
    }

    // Only process movement if there's input
    if direction.length_squared() > 0.0 {
        // Normalize the direction vector to prevent it from exceeding a magnitude of 1 when
        // moving diagonally.
        let normalized_direction = direction.normalize();

        // Check if player is currently performing a move (has Move component)
        let is_attacking = move_query.get(player.0).is_ok();

        // Choose speed based on whether player is attacking
        let current_speed = if is_attacking {
            ATTACK_SPEED
        } else {
            PLAYER_SPEED
        };

        // Apply movement
        let move_delta = normalized_direction * current_speed * time.delta_secs();
        player.1.translation += move_delta.extend(0.);

        let has_target: bool = player.2.target != Entity::PLACEHOLDER;

        // Only apply rotation if NOT attacking
        if !is_attacking && !has_target{
            // Calculate rotation to face movement direction
            let player_forward = (player.1.rotation * Vec3::Y).xy();
            let forward_dot_movement = player_forward.dot(normalized_direction);

            // If not already facing the movement direction
            if (forward_dot_movement - 1.0).abs() > f32::EPSILON {
                let player_right = (player.1.rotation * Vec3::X).xy();
                let right_dot_movement = player_right.dot(normalized_direction);

                // Determine rotation direction (negate for Bevy's coordinate system)
                let rotation_sign = -f32::copysign(1.0, right_dot_movement);

                // Calculate maximum rotation angle to prevent overshooting
                let max_angle =
                    std::f32::consts::PI.min(forward_dot_movement.clamp(-1.0, 1.0).acos());

                // Calculate actual rotation angle with speed limit
                let rotation_angle =
                    rotation_sign * (PLAYER_ROTATION_SPEED * time.delta_secs()).min(max_angle);

                // Apply rotation
                player.1.rotate_z(rotation_angle);
            }
        }
    }
}
