use bevy::prelude::*;
use std::collections::HashMap;

use crate::constants::SWING_LEFT;

#[derive(Component)]
pub struct Move {
    pub move_metadata: MoveMetadata,
    pub move_time: f32,
    current_phase: MovePhase,
    pub actor: Entity,
    pub next_move: Option<MoveMetadata>, // Add this field to store the next move
}

#[derive(Component)]
pub struct PlayerMove {}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MovePhase {
    Startup,
    Active, // accept next move during Active and Recovery phase
    Recovery,
}

#[derive(Clone, PartialEq)] // Add PartialEq for comparison
pub enum MoveType {
    // MoveType determine how weapon moves
    Swing,
    Stub,
    DashStub,
}

#[derive(Clone, PartialEq, Debug)] // Add PartialEq for comparison
pub enum MoveInput {
    Attack,
}

#[derive(Event)]
pub struct ExecuteMoveEvent {
    pub entity: Entity,
    pub move_name: String,
    pub move_input: MoveInput,
}

#[derive(Clone)]
pub struct MoveMetadata {
    name: String,
    radius: f32, // parameter to move weapon
    pub startup_time: f32,
    pub active_time: f32,
    recovery_time: f32,
    pub move_type: MoveType,
    accept_input: MoveInput,
    next_move: Option<String>,
}

#[derive(Event)]
pub struct MoveActiveEvent {
    pub actor: Entity,
    pub move_name: String,
}

#[derive(Event)]
pub struct MoveRecoveryEvent {
    pub actor: Entity,
    pub move_name: String,
}

// The main plugin
pub struct MovePlugin;

impl Plugin for MovePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MoveDatabase>()
            .add_event::<ExecuteMoveEvent>()
            .add_event::<MoveActiveEvent>() // Add this
            .add_event::<MoveRecoveryEvent>()
            .add_systems(Update, (handle_move_execution, update_moves));
    }
}

#[derive(Resource)]
pub struct MoveDatabase {
    pub moves: HashMap<String, MoveMetadata>,
}

impl Default for MoveDatabase {
    fn default() -> Self {
        let mut moves = HashMap::new();

        let swing_left = MoveMetadata {
            name: SWING_LEFT.to_string(),
            radius: 130.0,
            startup_time: 0.15,
            active_time: 0.15,
            recovery_time: 0.35,
            move_type: MoveType::Swing,
            accept_input: MoveInput::Attack,
            next_move: Some(SWING_LEFT.to_string()),
        };

        moves.insert(SWING_LEFT.to_string(), swing_left);

        Self { moves }
    }
}

fn handle_move_execution(
    mut commands: Commands,
    mut move_events: EventReader<ExecuteMoveEvent>,
    move_db: Res<MoveDatabase>,
    mut query: Query<(Entity, Option<&mut Move>)>,
    mut player_query: Query<Entity, With<crate::Player>>,
) {
    for event in move_events.read() {
        if let Ok((entity, current_move)) = query.get_mut(event.entity) {
            if let Some(mut current) = current_move {
                // Check if we can chain moves during Active or Recovery phase
                if (current.current_phase == MovePhase::Active || current.current_phase == MovePhase::Recovery) 
                    && current.move_metadata.accept_input == event.move_input {
                    // Get the move to chain from the database
                    if let Some(next_move_name) = current.move_metadata.next_move.clone()
                        && let Some(next_move_data) = move_db.moves.get(&next_move_name) {
                        current.next_move = Some(next_move_data.clone());
                        info!(
                            "Queued next move '{}' for entity {:?} during {:?} phase",
                            next_move_name, entity, current.current_phase
                        );
                    } else if let Some(next_move_name) = &current.move_metadata.next_move {
                        warn!("Next move '{}' not found in database", next_move_name);
                    }
                } else {
                    info!(
                        "Entity {:?} is busy executing move: {} (phase: {:?}, cannot accept input: {:?})",
                        entity, current.move_metadata.name, current.current_phase, event.move_input
                    );
                }
                continue;
            }

            // No current move, start a new one
            if let Some(move_data) = move_db.moves.get(&event.move_name) {
                if let Ok(player_entity) = player_query.single() {
                    let new_current_move = Move {
                        move_metadata: move_data.clone(),
                        move_time: 0.0,
                        current_phase: MovePhase::Startup,
                        actor: player_entity,
                        next_move: None, // Initialize with no next move
                    };
                    commands.entity(entity).insert(new_current_move);
                    commands.entity(player_entity).insert(PlayerMove {});

                    info!(
                        "Added PlayerMove component to player entity {:?}",
                        player_entity
                    );
                    info!(
                        "Entity {:?} started executing move: {}",
                        entity, event.move_name
                    );
                }
            } else {
                warn!("Move '{}' not found in database", event.move_name);
            }
        }
    }
}

fn update_moves(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Move, &mut Transform, &crate::sword::Sword)>,
    mut player_query: Query<Entity, With<crate::Player>>,
    mut start_move_events: EventWriter<MoveActiveEvent>,
    mut end_move_events: EventWriter<MoveRecoveryEvent>,
    time: Res<Time>,
) {
    for (entity, mut current_move, mut transform, sword) in query.iter_mut() {
        let delta_time = time.delta_secs();
        current_move.move_time += delta_time;

        let previous_phase = current_move.current_phase;

        // Determine the new phase based on move_time
        let new_phase = if current_move.move_time < current_move.move_metadata.startup_time {
            MovePhase::Startup
        } else if current_move.move_time
            < current_move.move_metadata.startup_time + current_move.move_metadata.active_time
        {
            // Fire StartMoveEvent when move begins
            if previous_phase != MovePhase::Active {
                start_move_events.write(MoveActiveEvent {
                    actor: current_move.actor,
                    move_name: current_move.move_metadata.name.clone(),
                });
            }
            MovePhase::Active
        } else if current_move.move_time
            < current_move.move_metadata.startup_time
                + current_move.move_metadata.active_time
                + current_move.move_metadata.recovery_time
        {
            // Fire EndMoveEvent when entering recovery phase
            if previous_phase != MovePhase::Recovery {
                end_move_events.write(MoveRecoveryEvent {
                    actor: current_move.actor,
                    move_name: current_move.move_metadata.name.clone(),
                });
            }
            MovePhase::Recovery
        } else {
            // Move would normally complete here
            // But check if we have a next move to chain
            if let Some(next_move_data) = current_move.next_move.take() {
                info!(
                    "Chaining to next move: {} from {}",
                    next_move_data.name, current_move.move_metadata.name
                );
                
                // Start the next move immediately
                *current_move = Move {
                    move_metadata: next_move_data,
                    move_time: 0.0,
                    current_phase: MovePhase::Startup,
                    actor: current_move.actor,
                    next_move: None,
                };
                continue;
            } else {
                // No next move, complete the current move
                transform.translation.x = sword.offset.x;
                transform.translation.y = sword.offset.y;
                transform.translation.z = sword.offset.z;
                transform.rotation = Quat::IDENTITY;

                commands.entity(entity).remove::<Move>();
                info!(
                    "Entity {:?} completed move: {} - position reset to offset",
                    entity, current_move.move_metadata.name
                );
                
                if let Ok(player_entity) = player_query.single() {
                    commands.entity(player_entity).remove::<PlayerMove>();
                }
                continue;
            }
        };

        // Check for early transition to next move during recovery phase
        if new_phase == MovePhase::Recovery && current_move.next_move.is_some() {
            if let Some(next_move_data) = current_move.next_move.take() {
                info!(
                    "Early transition to next move: {} from {} (skipping recovery)",
                    next_move_data.name, current_move.move_metadata.name
                );
                
                // Start the next move immediately, skipping the rest of recovery
                *current_move = Move {
                    move_metadata: next_move_data,
                    move_time: 0.0,
                    current_phase: MovePhase::Startup,
                    actor: current_move.actor,
                    next_move: None,
                };
                continue;
            }
        }

        // Log phase changes
        if previous_phase != new_phase {
            info!(
                "Entity {:?} move '{}' phase changed: {:?} -> {:?} (time: {:.3}s)",
                entity,
                current_move.move_metadata.name,
                previous_phase,
                new_phase,
                current_move.move_time
            );
        }

        current_move.current_phase = new_phase;

        // Update entity position during active phase
        if current_move.current_phase == MovePhase::Active {
            // Calculate normalized time within the active phase (0.0 to 1.0)
            let active_start_time = current_move.move_metadata.startup_time;
            let active_elapsed = current_move.move_time - active_start_time;
            let active_progress =
                (active_elapsed / current_move.move_metadata.active_time).clamp(0.0, 1.0);

            // Get position and rotation from cubic swing calculation
            let (swing_offset, swing_rotation) =
                crate::lerp_animation::calculate_vertical_swing_cubic(
                    active_progress,
                    current_move.move_metadata.radius,
                );

            // Apply the swing offset to the start position
            let new_position = swing_offset;

            // Update transform
            transform.translation.x = new_position.x;
            transform.translation.y = new_position.y;
            transform.rotation = Quat::from_rotation_z(swing_rotation);
        }
    }
}