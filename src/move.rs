use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
struct Move {
    pub move_metadata: MoveMetadata,
    pub move_time: f32,
    current_phase: MovePhase,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MovePhase {
    Startup,
    Active, // accept next move during Active and Recorvery phase
    Recovery,
}

#[derive(Clone)]
pub enum MoveType { // MoveType determine how weapon moves
    Swing,
    Stub,
    DashStub,
}

#[derive(Clone)]
enum MoveInput {
    Attack,
}

#[derive(Event)]
pub struct ExecuteMoveEvent {
    pub entity: Entity,
    pub move_name: String,
}

#[derive(Clone)]
pub struct MoveMetadata {
    name: String,
    start_pos: Vec2,
    start_rotation: f32,
    radius: f32, // parameter to move weapon
    pub startup_time: f32,
    pub active_time: f32,
    recovery_time: f32,
    pub move_type: MoveType,
    accept_input: MoveInput,
    next_move: Option<Box<MoveMetadata>>,
}

// The main plugin
pub struct MovePlugin;

impl Plugin for MovePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MoveDatabase>()
            .add_event::<ExecuteMoveEvent>()
            .add_systems(Update, (
                handle_move_execution,
                update_moves,
            ));
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
            name: "SwingLeft".to_string(),
            start_pos: Vec2::new(-10.0, 0.0),
            start_rotation: -45.0_f32.to_radians(),
            radius: 80.0,
            startup_time: 0.15,
            active_time: 0.25,
            recovery_time: 0.35,
            move_type: MoveType::Swing,
            accept_input: MoveInput::Attack,
            next_move: None,
        };
        
        moves.insert("SwingLeft".to_string(), swing_left);
        
        Self { moves }
    }
}

fn handle_move_execution(
    mut commands: Commands,
    mut move_events: EventReader<ExecuteMoveEvent>,
    move_db: Res<MoveDatabase>,
    mut query: Query<(Entity, Option<&mut Move>)>,
) {
    for event in move_events.read() {
        if let Ok((entity, current_move)) = query.get_mut(event.entity) {
            if let Some(mut current) = current_move {
                info!("Entity {:?} is busy executing move: {}", entity, current.move_metadata.name);
                continue;
            }
            
            if let Some(move_data) = move_db.moves.get(&event.move_name) {
                let total_time = move_data.startup_time + move_data.active_time + move_data.recovery_time;
                
                // Create or update the current move component
                let new_current_move = Move {
                    move_metadata: move_data.clone(),
                    move_time: 0.0,
                    current_phase: MovePhase::Startup,
                };
                
                commands.entity(entity).insert(new_current_move);
                
                info!("Entity {:?} started executing move: {}", entity, event.move_name);
            } else {
                warn!("Move '{}' not found in database", event.move_name);
            }
        }
    }
}

fn update_moves(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Move)>,
    time: Res<Time>,
) {
    for (entity, mut current_move) in query.iter_mut() {
        let delta_time = time.delta_secs();
        current_move.move_time += delta_time;
        
        let metadata = &current_move.move_metadata;
        let previous_phase = current_move.current_phase;
        
        // Determine the new phase based on move_time
        let new_phase = if current_move.move_time < metadata.startup_time {
            MovePhase::Startup
        } else if current_move.move_time < metadata.startup_time + metadata.active_time {
            MovePhase::Active
        } else if current_move.move_time < metadata.startup_time + metadata.active_time + metadata.recovery_time {
            MovePhase::Recovery
        } else {
            // Move is complete, remove the component
            commands.entity(entity).remove::<Move>();
            info!("Entity {:?} completed move: {}", entity, metadata.name);
            continue;
        };
        
        // Log phase changes
        if previous_phase != new_phase {
            info!("Entity {:?} move '{}' phase changed: {:?} -> {:?} (time: {:.3}s)", 
                  entity, metadata.name, previous_phase, new_phase, current_move.move_time);
        }
        
        current_move.current_phase = new_phase;
    }
}