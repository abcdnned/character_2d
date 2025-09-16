use crate::{
    ai::{LockType, TargetDetector}, berserker::Berserker, constants::*, float_text::spawn_sprint_ready_text, physics::apply_impulse
};
use bevy::{ecs::component, prelude::*};
use bevy_rapier2d::prelude::Velocity;
use crate::custom_move::Move;

#[derive(Component)]
pub struct SprintCD(pub f64);

#[derive(Component)]
pub struct SprintReadyLogged(pub bool);


pub fn move_player(
    mut player: Single<
        (Entity, &mut Transform, &TargetDetector, &mut Velocity, &mut SprintCD),
        With<crate::Player>,
    >,
    time: Res<Time>,
    mut move_events: EventReader<crate::input::MoveEvent>,
    move_query: Query<&crate::custom_move::PlayerMove, With<crate::Player>>,
    berserker_query: Query<&Berserker, With<crate::Player>>,
    global_entity_map: Res<crate::global_entity_map::GlobalEntityMap>,
    weapon_move_query: Query<&Move>,
) {
    let mut walk_direction = Vec2::ZERO;
    let mut sprint_direction = Vec2::ZERO;
    
    // Separate movement events by type
    for event in move_events.read() {
        trace!(
            "Received move event: direction={:?}, type={:?}",
            event.direction, event.movement_type
        );
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
        let current_time = time.elapsed_secs_f64();
        let time_since_last_sprint = current_time - player.4.0;
        
        // Check if sprint is on cooldown
        if time_since_last_sprint < SPRINT_CD {
            debug!(
                "Sprint on cooldown for entity {:?}. Time remaining: {:.2}s",
                player.0,
                SPRINT_CD - time_since_last_sprint
            );
            // Do nothing if still on cooldown
        } else {
            let normalized_sprint_direction = sprint_direction.normalize();
            
            // Use the physics module's apply_impulse method
            apply_impulse(
                player.0,
                normalized_sprint_direction,
                SPRINT_IMPULSE_FORCE,
                &mut player.3,
            );
            
            // Record the current time as the last sprint time
            player.4.0 = current_time;
            
            debug!(
                "Sprint impulse applied to entity {:?} at time {:.3}",
                player.0, player.4.0
            );
        }
    }
    
    // Handle walk movement (original logic)
    if walk_direction.length_squared() > 0.0 {
        // Normalize the direction vector to prevent it from exceeding a magnitude of 1 when
        // moving diagonally.
        let normalized_direction = walk_direction.normalize();
        
        // Check if player is currently performing a move (has Move component)
        let is_attacking = move_query.get(player.0).is_ok();
        
        // Query weapon entity from player entity using global entity map
        let base_speed = if let Some(weapon_entity) = global_entity_map.player_weapon.get(&player.0) {
            trace!("Found weapon entity {:?} for player {:?}", weapon_entity, player.0);
            
            // Query Move component from weapon entity
            if let Ok(weapon_move) = weapon_move_query.get(*weapon_entity) {
                let move_speed = weapon_move.move_metadata.move_speed;
                trace!(
                    "Using weapon move speed {} for player {:?} (weapon: {:?})",
                    move_speed, player.0, weapon_entity
                );
                move_speed
            } else {
                trace!(
                    "No Move component found on weapon entity {:?} for player {:?}, using default PLAYER_SPEED",
                    weapon_entity, player.0
                );
                PLAYER_SPEED
            }
        } else {
            trace!(
                "No weapon entity found for player {:?} in global entity map, using default PLAYER_SPEED",
                player.0
            );
            PLAYER_SPEED
        };
        
        // Check for Berserker component and modify speed if level 1
        let mut current_speed = if let Ok(berserker) = berserker_query.get(player.0) {
            if berserker.level == 1 {
                base_speed + BERSERKER_MOVE_SPEED
            } else {
                base_speed
            }
        } else {
            base_speed
        };

        if is_attacking {
            current_speed = current_speed * ATTACK_SPEED_FACTOR;
        }
        
        trace!(
            "Walking: direction={:?}, is_attacking={}, base_speed={}, current_speed={}",
            normalized_direction, is_attacking, base_speed, current_speed
        );
        
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

#[derive(Resource)]
pub struct SprintCheckTimer {
    timer: Timer,
}

impl Default for SprintCheckTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

pub struct SprintReadyPlugin;

impl Plugin for SprintReadyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SprintCheckTimer::default())
            .add_systems(Update, check_sprint_ready);
    }
}

pub fn check_sprint_ready(
    mut timer: ResMut<SprintCheckTimer>,
    time: Res<Time>,
    mut players: Query<(Entity, &SprintCD, &mut SprintReadyLogged), With<crate::Player>>,
    mut commands: Commands,
    transform_query: Query<&Transform>,
) {
    timer.timer.tick(time.delta());
    
    if timer.timer.just_finished() {
        let current_time = time.elapsed_secs_f64();
        
        for (entity, sprint_cd, mut ready_logged) in players.iter_mut() {
            let time_since_last_sprint = current_time - sprint_cd.0;
            
            if time_since_last_sprint >= SPRINT_CD {
                // Only log if we haven't already logged this ready state
                if !ready_logged.0 {
                    debug!(
                        "Sprint ready for player entity {:?} (last sprint: {:.2}s ago)",
                        entity, time_since_last_sprint
                    );
                    if let Ok(transform) = transform_query.get(entity) {
                        spawn_sprint_ready_text(&mut commands, transform.translation);
                    } else {
                        warn!("no transform found when sprint ready {:?}", entity);
                    }
                    ready_logged.0 = true;
                }
            } else {
                // Reset the logged flag when sprint goes on cooldown
                ready_logged.0 = false;
            }
        }
    }
}