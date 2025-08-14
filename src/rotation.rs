use bevy::prelude::*;
use crate::{ai::TargetDetector, global_entity_map::GlobalEntityMap};

pub struct RotationPlugin;

impl Plugin for RotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, facing_target);
    }
}

pub fn facing_target(
    mut query: Query<(&mut Transform, &TargetDetector)>,
    globals: ResMut<GlobalEntityMap>,
) {
    for (mut transform, detector) in query.iter_mut() {
        // Check if target is valid (not a placeholder)
        if detector.target != Entity::PLACEHOLDER {
            // Get the target's position
            if let Some(target_transform) =  globals.entity_transfrom.get(&detector.target) {
                // Calculate direction vector from current position to target
                let direction = target_transform.translation - transform.translation;
                
                // Calculate angle for 2D rotation (around Z-axis)
                let angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
                let target_rotation = Quat::from_rotation_z(angle);
                
                // Set the rotation to face the target
                transform.rotation = target_rotation;
            }
            
        }
    }
}