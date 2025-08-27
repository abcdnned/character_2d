use crate::constants::{STOP_CHASING_RANGE, SWING_LEFT, SWING_RIGHT, SWORD_STUB};
use crate::force::Force;
use crate::global_entity_map::GlobalEntityMap;
use crate::unit::UnitType;
use crate::weapon::GearSet;
use crate::{Player, Unit};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

#[derive(Component)]
pub struct TargetDetector {
    pub target: Entity,
    pub alert_range: f32,
    pub dis_alert_range: f32,
}

#[derive(Clone)]
pub struct AIOption {
    pub name: String,
    pub active_range: f32,
}

impl AIOption {
    pub fn new(name: String, active_range: f32) -> Self {
        Self { name, active_range }
    }
}

#[derive(Component)]
pub struct AI {
    pub option_queue: Vec<AIOption>,
    pub current_move_index: usize,
}

impl AI {
    pub fn new(moves: Vec<AIOption>) -> Self {
        Self {
            option_queue: moves,
            current_move_index: 0,
        }
    }

    pub fn get_next_move(&mut self, distance: f32) -> Option<&AIOption> {
        if self.option_queue.is_empty() {
            return None;
        }

        let current_move = &self.option_queue[self.current_move_index];

        // Check if distance matches the current move's active range
        if distance <= current_move.active_range {
            // Update to next move index for next call
            self.current_move_index = (self.current_move_index + 1) % self.option_queue.len();
            Some(current_move)
        } else {
            None
        }
    }
}

impl Default for AI {
    fn default() -> Self {
        Self::new(vec![AIOption::new(
            SWORD_STUB.to_string(),
            STOP_CHASING_RANGE + 20.0,
        )])
    }
}

pub fn ai_target_detection_system(
    mut ai_query: Query<(&mut TargetDetector, &Transform, &Force, Entity)>,
    potential_targets_query: Query<(Entity, &Transform, &Force)>,
) {
    for (mut ai_brain, ai_transform, ai_force, ai_entity) in ai_query.iter_mut() {
        let mut closest_target: Option<(Entity, f32)> = None;

        // Check all potential targets with different force
        for (target_entity, target_transform, target_force) in potential_targets_query.iter() {
            // info!("Checking potential target: {:?}", target_entity);
            // Skip if same force (don't target allies)
            if ai_force.force == target_force.force {
                continue;
            }

            let distance = ai_transform
                .translation
                .distance(target_transform.translation);

            // If we have a current target, check if we should disengage
            if ai_brain.target == target_entity && distance > ai_brain.dis_alert_range {
                info!(
                    "AI entity {:?} disengaging from target {:?} at distance {:.2}",
                    ai_entity, target_entity, distance
                );
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
            if ai_brain.target == Entity::PLACEHOLDER {
                info!(
                    "entity {:?} acquiring new target {:?} at distance {:.2}",
                    ai_entity, target_entity, distance
                );
                ai_brain.target = target_entity;
            }
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
            let distance = ai_transform
                .translation
                .distance(target_transform.translation);
            if (distance >= STOP_CHASING_RANGE) {
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
}

// Attack system using force-based targeting
pub fn ai_attack_system(
    mut ai_query: Query<(&TargetDetector, &Transform, &mut AI, Entity)>,
    target_query: Query<&Transform, (Without<AI>)>,
    mut move_events: EventWriter<crate::custom_move::ExecuteMoveEvent>,
    global_entities: ResMut<GlobalEntityMap>,
) {
    for (ai_brain, ai_transform, mut ai, ai_entity) in ai_query.iter_mut() {
        // Skip if no valid target
        if ai_brain.target == Entity::PLACEHOLDER {
            continue;
        }

        // Get target position
        if let Ok(target_transform) = target_query.get(ai_brain.target) {
            let distance = ai_transform
                .translation
                .distance(target_transform.translation);

            // Get the AI's weapon entity from global_entities
            if let Some(weapon) = global_entities.player_weapon.get(&ai_entity) {
                // Find the best move for the current distance
                if let Some(selected_move) = ai.get_next_move(distance) {
                    // info!("AI entity {:?} attacking target {:?} with move {} (range: {:.1}) at distance {:.2}",
                    //       ai_entity, ai_brain.target, selected_move.name, selected_move.active_range, distance);

                    move_events.write(crate::custom_move::ExecuteMoveEvent {
                        entity: *weapon,
                        move_name: selected_move.name.clone(),
                        move_input: crate::custom_move::MoveInput::Attack,
                    });
                }
            }
        }
    }
}

pub fn initialize_unit_aioptions(mut global_map: ResMut<GlobalEntityMap>) {
    // Define moves for SwordMan
    let moves = vec![
        AIOption::new(SWING_LEFT.to_string(), STOP_CHASING_RANGE + 10.0),
        AIOption::new(SWING_RIGHT.to_string(), STOP_CHASING_RANGE + 10.0),
        AIOption::new(SWORD_STUB.to_string(), STOP_CHASING_RANGE + 20.0),
    ];
    // Insert into global map
    global_map
        .unittype_aioptions
        .insert(UnitType::SwordMan, moves);
}

// Plugin to register the AI systems
pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                ai_target_detection_system,
                ai_movement_system,
                ai_attack_system,
            )
                .chain(),
        )
        .add_systems(Startup, initialize_unit_aioptions); // Chain ensures they run in order
    }
}
