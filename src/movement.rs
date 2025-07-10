
use bevy::prelude::*;
use crate::constants::*;

pub fn move_player(
    mut player: Single<&mut Transform, With<crate::Player>>,
    time: Res<Time>,
    mut move_events: EventReader<crate::input::MoveEvent>
) {
    let mut direction = Vec2::ZERO;
    
    // Accumulate all movement events this frame
    for event in move_events.read() {
        direction += event.direction;
    }
    
    // Only process movement and rotation if there's input
    if direction.length_squared() > 0.0 {
        // Normalize the direction vector to prevent it from exceeding a magnitude of 1 when
        // moving diagonally.
        let normalized_direction = direction.normalize();
        
        // Apply movement
        let move_delta = normalized_direction * PLAYER_SPEED * time.delta_secs();
        player.translation += move_delta.extend(0.);
        
        // Calculate rotation to face movement direction
        let player_forward = (player.rotation * Vec3::Y).xy();
        let forward_dot_movement = player_forward.dot(normalized_direction);
        
        // If not already facing the movement direction
        if (forward_dot_movement - 1.0).abs() > f32::EPSILON {
            let player_right = (player.rotation * Vec3::X).xy();
            let right_dot_movement = player_right.dot(normalized_direction);
            
            // Determine rotation direction (negate for Bevy's coordinate system)
            let rotation_sign = -f32::copysign(1.0, right_dot_movement);
            
            // Calculate maximum rotation angle to prevent overshooting
            let max_angle = std::f32::consts::PI.min(
                forward_dot_movement.clamp(-1.0, 1.0).acos()
            );
            
            // Calculate actual rotation angle with speed limit
            let rotation_angle = rotation_sign * 
                (PLAYER_ROTATION_SPEED * time.delta_secs()).min(max_angle);
            
            // Apply rotation
            player.rotate_z(rotation_angle);
        }
    }
}