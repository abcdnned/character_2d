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

        if detector.target != Entity::PLACEHOLDER {

            if let Some(target_transform) =  globals.entity_transfrom.get(&detector.target) {

                let direction = target_transform.translation - transform.translation;

                let angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;

                let target_rotation = Quat::from_rotation_z(angle);

                transform.rotation = target_rotation;
            }
            
        }
    }
}