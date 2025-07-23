use bevy::prelude::*;
use std::collections::HashMap;

use crate::constants::SWING_LEFT;

#[derive(Component)]
pub struct Move {
    pub move_metadata: MoveMetadata,
    pub move_time: f32,
    current_phase: MovePhase,
    pub actor: Entity,
    pub next_move: Option<MoveMetadata>,
}

impl Move {
    pub fn new(metadata: MoveMetadata, actor: Entity) -> Self {
        Self {
            move_metadata: metadata,
            move_time: 0.0,
            current_phase: MovePhase::Startup,
            actor,
            next_move: None,
        }
    }

    pub fn transition_to(&mut self, next_metadata: MoveMetadata) {
        info!(
            "Transitioning to next move: {} from {}",
            next_metadata.name, self.move_metadata.name
        );
        
        *self = Self::new(next_metadata, self.actor);
    }

    pub fn can_accept_input(&self, input: &MoveInput) -> bool {
        matches!(self.current_phase, MovePhase::Active | MovePhase::Recovery)
            && self.move_metadata.accept_input == *input
    }

    pub fn total_duration(&self) -> f32 {
        self.move_metadata.startup_time + self.move_metadata.active_time + self.move_metadata.recovery_time
    }

    pub fn update_phase(&mut self) -> (MovePhase, bool) {
        let previous_phase = self.current_phase;
        
        let new_phase = if self.move_time < self.move_metadata.startup_time {
            MovePhase::Startup
        } else if self.move_time < self.move_metadata.startup_time + self.move_metadata.active_time {
            MovePhase::Active
        } else if self.move_time < self.total_duration() {
            MovePhase::Recovery
        } else {
            return (MovePhase::Recovery, true); // Move completed
        };

        let phase_changed = previous_phase != new_phase;
        if phase_changed {
            info!(
                "Move '{}' phase changed: {:?} -> {:?} (time: {:.3}s)",
                self.move_metadata.name, previous_phase, new_phase, self.move_time
            );
        }

        self.current_phase = new_phase;
        (new_phase, false)
    }

    pub fn get_active_progress(&self) -> f32 {
        if self.current_phase != MovePhase::Active {
            return 0.0;
        }
        
        let active_start_time = self.move_metadata.startup_time;
        let active_elapsed = self.move_time - active_start_time;
        (active_elapsed / self.move_metadata.active_time).clamp(0.0, 1.0)
    }
}

#[derive(Component)]
pub struct PlayerMove {}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MovePhase {
    Startup,
    Active,
    Recovery,
}

#[derive(Clone, PartialEq)]
pub enum MoveType {
    Swing,
    Stub,
    DashStub,
}

#[derive(Clone, PartialEq, Debug)]
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
    radius: f32,
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

pub struct MovePlugin;

impl Plugin for MovePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MoveDatabase>()
            .add_event::<ExecuteMoveEvent>()
            .add_event::<MoveActiveEvent>()
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
                handle_move_chaining(&mut current, &event, &move_db, entity);
                continue;
            }

            start_new_move(&mut commands, &event, &move_db, entity, &mut player_query);
        }
    }
}

fn handle_move_chaining(
    current: &mut Move,
    event: &ExecuteMoveEvent,
    move_db: &MoveDatabase,
    entity: Entity,
) {
    if !current.can_accept_input(&event.move_input) {
        info!(
            "Entity {:?} is busy executing move: {} (phase: {:?}, cannot accept input: {:?})",
            entity, current.move_metadata.name, current.current_phase, event.move_input
        );
        return;
    }

    if let Some(next_move_name) = current.move_metadata.next_move.clone() {
        if let Some(next_move_data) = move_db.moves.get(&next_move_name) {
            current.next_move = Some(next_move_data.clone());
            info!(
                "Queued next move '{}' for entity {:?} during {:?} phase",
                next_move_name, entity, current.current_phase
            );
        } else {
            warn!("Next move '{}' not found in database", next_move_name);
        }
    }
}

fn start_new_move(
    commands: &mut Commands,
    event: &ExecuteMoveEvent,
    move_db: &MoveDatabase,
    entity: Entity,
    player_query: &mut Query<Entity, With<crate::Player>>,
) {
    if let Some(move_data) = move_db.moves.get(&event.move_name) {
        if let Ok(player_entity) = player_query.single() {
            let new_move = Move::new(move_data.clone(), player_entity);
            commands.entity(entity).insert(new_move);
            commands.entity(player_entity).insert(PlayerMove {});

            info!("Added PlayerMove component to player entity {:?}", player_entity);
            info!("Entity {:?} started executing move: {}", entity, event.move_name);
        }
    } else {
        warn!("Move '{}' not found in database", event.move_name);
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
        current_move.move_time += time.delta_secs();
        
        let previous_phase = current_move.current_phase;
        let (new_phase, move_completed) = current_move.update_phase();

        // Handle phase transition events
        handle_phase_events(&mut start_move_events, &mut end_move_events, &current_move, previous_phase, new_phase);

        // Handle move completion or chaining
        if move_completed {
            if handle_move_completion_or_chaining(&mut current_move) {
                continue; // Move was chained, continue with new move
            } else {
                complete_move(&mut commands, entity, &mut transform, sword, &current_move, &mut player_query);
                continue;
            }
        }

        // Handle early transition during recovery
        if new_phase == MovePhase::Recovery && current_move.next_move.is_some() {
            if let Some(next_move_data) = current_move.next_move.take() {
                info!(
                    "Early transition to next move: {} from {} (skipping recovery)",
                    next_move_data.name, current_move.move_metadata.name
                );
                current_move.transition_to(next_move_data);
                continue;
            }
        }

        // Update position during active phase
        if current_move.current_phase == MovePhase::Active {
            update_move_animation(&mut transform, &current_move);
        }
    }
}

fn handle_phase_events(
    start_events: &mut EventWriter<MoveActiveEvent>,
    end_events: &mut EventWriter<MoveRecoveryEvent>,
    current_move: &Move,
    previous_phase: MovePhase,
    new_phase: MovePhase,
) {
    if previous_phase != new_phase {
        match new_phase {
            MovePhase::Active => {
                start_events.send(MoveActiveEvent {
                    actor: current_move.actor,
                    move_name: current_move.move_metadata.name.clone(),
                });
            }
            MovePhase::Recovery => {
                end_events.send(MoveRecoveryEvent {
                    actor: current_move.actor,
                    move_name: current_move.move_metadata.name.clone(),
                });
            }
            _ => {}
        }
    }
}

fn handle_move_completion_or_chaining(current_move: &mut Move) -> bool {
    if let Some(next_move_data) = current_move.next_move.take() {
        current_move.transition_to(next_move_data);
        true // Move was chained
    } else {
        false // No chaining, move should complete
    }
}

fn complete_move(
    commands: &mut Commands,
    entity: Entity,
    transform: &mut Transform,
    sword: &crate::sword::Sword,
    current_move: &Move,
    player_query: &mut Query<Entity, With<crate::Player>>,
) {
    // Reset position and rotation
    transform.translation = sword.offset;
    transform.rotation = Quat::IDENTITY;

    commands.entity(entity).remove::<Move>();
    info!(
        "Entity {:?} completed move: {} - position reset to offset",
        entity, current_move.move_metadata.name
    );

    // Remove PlayerMove component
    if let Ok(player_entity) = player_query.single() {
        commands.entity(player_entity).remove::<PlayerMove>();
    }
}

fn update_move_animation(transform: &mut Transform, current_move: &Move) {
    let active_progress = current_move.get_active_progress();
    
    let (swing_offset, swing_rotation) = crate::lerp_animation::calculate_vertical_swing_cubic(
        active_progress,
        current_move.move_metadata.radius,
    );

    transform.translation.x = swing_offset.x;
    transform.translation.y = swing_offset.y;
    transform.rotation = Quat::from_rotation_z(swing_rotation);
}