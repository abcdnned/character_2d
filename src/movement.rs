use crate::{ai::{LockType, TargetDetector}, constants::*, physics::apply_impulse};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

pub fn move_player(
    mut player: Single<(Entity, &mut Transform, &TargetDetector, &mut Velocity), With<crate::Player>>,
    time: Res<Time>,
    mut move_events: EventReader<crate::input::MoveEvent>,
    move_query: Query<&crate::custom_move::PlayerMove, With<crate::Player>>,
) {
    let mut walk_direction = Vec2::ZERO;
    let mut sprint_direction = Vec2::ZERO;

    // Separate movement events by type
    for event in move_events.read() {
        trace!("Received move event: direction={:?}, type={:?}", event.direction, event.movement_type);
        match event.movement_type {
            crate::input::MovementType::Walk => {
                walk_direction += event.direction;
            }
            crate::input::MovementType::Sprint => {
                sprint_direction += event.direction;
            }
        }
    }

    // Handle sprint movement (velocity impulse)
    if sprint_direction.length_squared() > 0.0 {
        let normalized_sprint_direction = sprint_direction.normalize();
        
        // Use the physics module's apply_impulse method
        apply_impulse(player.0, normalized_sprint_direction, SPRINT_IMPULSE_FORCE, &mut player.3);
        debug!("Sprint impulse applied to entity {:?}", player.0);
    }

    // Handle walk movement (original logic)
    if walk_direction.length_squared() > 0.0 {
        // Normalize the direction vector to prevent it from exceeding a magnitude of 1 when
        // moving diagonally.
        let normalized_direction = walk_direction.normalize();

        // Check if player is currently performing a move (has Move component)
        let is_attacking = move_query.get(player.0).is_ok();

        // Choose speed based on whether player is attacking
        let current_speed = if is_attacking {
            ATTACK_SPEED
        } else {
            PLAYER_SPEED
        };

        trace!("Walking: direction={:?}, is_attacking={}, speed={}", normalized_direction, is_attacking, current_speed);

        // Apply movement
        let move_delta = normalized_direction * current_speed * time.delta_secs();
        player.1.translation += move_delta.extend(0.);

        let has_target: bool = player.2.target != Entity::PLACEHOLDER;

        // Only apply rotation if NOT attacking
        if !is_attacking && (!has_target || player.2.lock_type == LockType::Free) {
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

                trace!("Applying rotation: angle={:.2} rad", rotation_angle);

                // Apply rotation
                player.1.rotate_z(rotation_angle);
            }
        }
    }
}