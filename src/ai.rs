use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use crate::weapon::GearSet;
use crate::{Player, Unit};

#[derive(Component)]
pub struct AIBrain {
    pub gear_set: GearSet,
    pub target: Entity,
    pub alert_range: f32,
    pub dis_alert_range: f32,
}

// System to detect players within alert range and set them as targets
pub fn ai_target_detection_system(
    mut ai_query: Query<(&mut AIBrain, &Transform, Entity)>,
    player_query: Query<(Entity, &Transform), (With<Player>, Without<AIBrain>)>,
) {
    for (mut ai_brain, ai_transform, ai_entity) in ai_query.iter_mut() {
        let mut closest_player: Option<(Entity, f32)> = None;
        
        // Check all players
        for (player_entity, player_transform) in player_query.iter() {
            let distance = ai_transform.translation.distance(player_transform.translation);
            
            // If we have a current target, check if we should disengage
            if ai_brain.target == player_entity && distance > ai_brain.dis_alert_range {
                ai_brain.target = Entity::PLACEHOLDER;
                continue;
            }
            
            // Check if player is within alert range
            if distance <= ai_brain.alert_range {
                match closest_player {
                    None => closest_player = Some((player_entity, distance)),
                    Some((_, closest_distance)) => {
                        if distance < closest_distance {
                            closest_player = Some((player_entity, distance));
                        }
                    }
                }
            }
        }
        
        // Set the closest player as target
        if let Some((player_entity, _)) = closest_player {
            ai_brain.target = player_entity;
        }
    }
}

// System to move AI entities towards their targets
pub fn ai_movement_system(
    mut ai_query: Query<(&AIBrain, &mut Transform, &mut Velocity, &Unit)>,
    target_query: Query<&Transform, (Without<AIBrain>, Without<Velocity>)>,
    time: Res<Time>,
) {
    for (ai_brain, mut ai_transform, mut velocity, unit) in ai_query.iter_mut() {
        // Skip if no valid target
        if ai_brain.target == Entity::PLACEHOLDER {
            // Optionally slow down or stop when no target
            velocity.linvel *= 0.9; // Gradual slowdown
            continue;
        }
        
        // Get target position
        if let Ok(target_transform) = target_query.get(ai_brain.target) {
            // Calculate direction to target
            let direction = (target_transform.translation - ai_transform.translation).normalize_or_zero();
            
            // Apply movement using the unit's speed and delta time
            let movement_delta = direction * unit.speed * time.delta_secs();
            
            // Add delta to the translation
            ai_transform.translation += movement_delta;
        }
    }
}

// Optional: System to clear invalid targets
pub fn ai_cleanup_system(
    mut ai_query: Query<&mut AIBrain>,
    target_query: Query<Entity, With<Player>>,
) {
    // Create a set of valid player entities
    let valid_targets: std::collections::HashSet<Entity> = target_query.iter().collect();
    
    for mut ai_brain in ai_query.iter_mut() {
        if ai_brain.target != Entity::PLACEHOLDER && !valid_targets.contains(&ai_brain.target) {
            ai_brain.target = Entity::PLACEHOLDER;
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
            ai_cleanup_system,
        ).chain()); // Chain ensures they run in order
    }
}