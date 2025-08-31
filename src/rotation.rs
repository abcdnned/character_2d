use crate::{ai::TargetDetector, global_entity_map::GlobalEntityMap, custom_move::PlayerMove};
use bevy::prelude::*;
use ordered_float::Float;

pub struct RotationPlugin;

impl Plugin for RotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, facing_target);
    }
}

pub fn facing_target(
    mut query: Query<(Entity, &mut Transform, &TargetDetector)>,
    player_move_query: Query<&PlayerMove>,
    globals: ResMut<GlobalEntityMap>,
) {
    // Define the specific degree threshold (you can adjust this value)
    let degree_threshold = 45.0; // degrees
    let angle_threshold = degree_threshold.to_radians();

    for (entity, mut transform, detector) in query.iter_mut() {
        if detector.target != Entity::PLACEHOLDER {
            if let Some(target_transform) = globals.entity_transfrom.get(&detector.target) {
                let direction = target_transform.translation - transform.translation;
                let angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
                let target_rotation = Quat::from_rotation_z(angle);

                match detector.lock_type {
                    crate::ai::LockType::Lock => {
                        // Original behavior for Lock type
                        transform.rotation = target_rotation;
                    }
                    crate::ai::LockType::Free => {
                        // Check if entity has PlayerMove component
                        if let Ok(_player_move) = player_move_query.get(entity) {
                            // Calculate the angle between current facing and target direction
                            let current_angle = transform.rotation.to_euler(EulerRot::ZYX).0;
                            let angle_difference = (angle - current_angle).abs();
                            
                            // Normalize angle difference to be between 0 and PI
                            let normalized_angle_diff = if angle_difference > std::f32::consts::PI {
                                2.0 * std::f32::consts::PI - angle_difference
                            } else {
                                angle_difference
                            };
                            
                            // If angle difference is less than threshold, face the target
                            if normalized_angle_diff < angle_threshold {
                                info!(
                                    "Free lock success: Entity {:?} facing target {:?} (angle diff: {:.2}° < {:.2}°)",
                                    entity,
                                    detector.target,
                                    normalized_angle_diff.to_degrees(),
                                    degree_threshold
                                );
                                transform.rotation = target_rotation;
                            }
                        }
                    }
                }
            }
        }
    }
}