use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use crate::constants::SWORD_STUB;
use crate::weapon::GearSet;
use crate::{Player, Unit};
use crate::global_entity_map::GlobalEntityMap;
use crate::force::Force;

#[derive(Component)]
pub struct TargetDetector {
    pub target: Entity,
    pub alert_range: f32,
    pub dis_alert_range: f32,
}

#[derive(Component)]
pub struct AI {
}

pub fn ai_target_detection_system(
    mut ai_query: Query<(&mut TargetDetector, &Transform, &Force, Entity)>,
    potential_targets_query: Query<(Entity, &Transform, &Force)>,
) {
    for (mut ai_brain, ai_transform, ai_force, ai_entity) in ai_query.iter_mut() {
        let mut closest_target: Option<(Entity, f32)> = None;
        
        // Check all potential targets with different force
        for (target_entity, target_transform, target_force) in potential_targets_query.iter() {
            info!("Checking potential target: {:?}", target_entity);
            // Skip if same force (don't target allies)
            if ai_force.force == target_force.force {
                continue;
            }
            
            let distance = ai_transform.translation.distance(target_transform.translation);
            
            // If we have a current target, check if we should disengage
            if ai_brain.target == target_entity && distance > ai_brain.dis_alert_range {
                info!("AI entity {:?} disengaging from target {:?} at distance {:.2}", ai_entity, target_entity, distance);
                ai_brain.target = Entity::PLACEHOLDER;
                continue;
            }
            
            // Check if target is within alert range
            if distance <= ai_brain.alert_range {
                match closest_target {
                    None => closest_target = Some((target_entity, distance)),
                    Some((_, closest_distance)) => {
                        if distance < closest_distance {
                            closest_target = Some((target_entity, distance));
                        }
                    }
                }
            }
        }
        
        // Set the closest enemy target
        if let Some((target_entity, distance)) = closest_target {
            if ai_brain.target != target_entity {
                info!("AI entity {:?} acquiring new target {:?} at distance {:.2}", ai_entity, target_entity, distance);
            }
            ai_brain.target = target_entity;
        }
    }
}

// System to move AI entities towards their targets
pub fn ai_movement_system(
    mut ai_query: Query<(&TargetDetector, &mut Transform, &mut Velocity, &Unit), With<AI>>,
    target_query: Query<&Transform, Without<AI>>, // Query for target transforms
    time: Res<Time>,
) {
    for (ai_brain, mut ai_transform, mut velocity, unit) in ai_query.iter_mut() {
        // Skip if no valid target
        if ai_brain.target == Entity::PLACEHOLDER {
            // Optionally slow down or stop when no target
            velocity.linvel *= 0.9; // Gradual slowdown
            continue;
        }
        
        if let Ok(target_transform) = target_query.get(ai_brain.target) {
            // Calculate direction to target
            let direction_vector = target_transform.translation - ai_transform.translation;
            let direction = direction_vector.normalize_or_zero();
            
            // Apply movement using the unit's speed and delta time
            let movement_delta = direction * unit.speed * time.delta_secs();
            
            // Add delta to the translation
            ai_transform.translation += movement_delta;
        }
    }
}

// System to clear invalid targets
pub fn ai_cleanup_system(
    mut ai_query: Query<(&mut TargetDetector, &Force)>,
    target_query: Query<(Entity, &Force), Without<TargetDetector>>,
) {
    // Create a set of valid target entities (different force)
    for (mut ai_brain, ai_force) in ai_query.iter_mut() {
        if ai_brain.target != Entity::PLACEHOLDER {
            // Check if the target still exists and has a different force
            let target_valid = target_query.iter()
                .any(|(entity, target_force)| {
                    entity == ai_brain.target && ai_force.force != target_force.force
                });
            
            if !target_valid {
                ai_brain.target = Entity::PLACEHOLDER;
            }
        }
    }
}

// Attack system using force-based targeting
pub fn ai_attack_system(
    ai_query: Query<(&TargetDetector, &Transform, Entity), With<AI>>,
    target_query: Query<&Transform, (Without<AI>)>,
    mut move_events: EventWriter<crate::r#move::ExecuteMoveEvent>,
    global_entities: ResMut<GlobalEntityMap>,
) {
    for (ai_brain, ai_transform, ai_entity) in ai_query.iter() {
        // Skip if no valid target
        if ai_brain.target == Entity::PLACEHOLDER {
            continue;
        }
        
        // Get target position
        if let Ok(target_transform) = target_query.get(ai_brain.target) {
            let distance = ai_transform.translation.distance(target_transform.translation);
            
            // Check if within attack range
            if distance <= 100.0 {
                // Get the AI's weapon entity from global_entities
                if let Some(weapon) = global_entities.player_weapon.get(&ai_entity) {
                    info!("AI entity {:?} attacking target {:?} at distance {:.2}", ai_entity, ai_brain.target, distance);
                    move_events.write(crate::r#move::ExecuteMoveEvent {
                        entity: *weapon,
                        move_name: SWORD_STUB.to_string(),
                        move_input: crate::r#move::MoveInput::Attack,
                    });
                }
            }
        }
    }
}

// Plugin to register the AI systems
pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            ai_target_detection_system,
            ai_movement_system,
            ai_attack_system,
            ai_cleanup_system,
        ).chain()); // Chain ensures they run in order
    }
}