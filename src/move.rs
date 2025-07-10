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
            radius: 130.0,
            startup_time: 0.15,
            active_time: 0.15,
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
    mut query: Query<(Entity, &mut Move, &mut Transform, &crate::sword::Sword)>,
    time: Res<Time>,
) {
    for (entity, mut current_move, mut transform, sword) in query.iter_mut() {
        let delta_time = time.delta_secs();
        current_move.move_time += delta_time;
        
        let previous_phase = current_move.current_phase;
        
        // Determine the new phase based on move_time
        let new_phase = if current_move.move_time < current_move.move_metadata.startup_time {
            MovePhase::Startup
        } else if current_move.move_time < current_move.move_metadata.startup_time + current_move.move_metadata.active_time {
            MovePhase::Active
        } else if current_move.move_time < current_move.move_metadata.startup_time + current_move.move_metadata.active_time + current_move.move_metadata.recovery_time {
            MovePhase::Recovery
        } else {
            // Move is complete, reset position to sword offset and remove the component
            transform.translation.x = sword.offset.x;
            transform.translation.y = sword.offset.y;
            transform.translation.z = sword.offset.z;
            transform.rotation = Quat::IDENTITY; // Reset rotation to default
            
            commands.entity(entity).remove::<Move>();
            info!("Entity {:?} completed move: {} - position reset to offset", entity, current_move.move_metadata.name);
            continue;
        };
        
        // Log phase changes
        if previous_phase != new_phase {
            info!("Entity {:?} move '{}' phase changed: {:?} -> {:?} (time: {:.3}s)", 
                  entity, current_move.move_metadata.name, previous_phase, new_phase, current_move.move_time);
        }
        
        current_move.current_phase = new_phase;

        // Update entity position during active phase
        if current_move.current_phase == MovePhase::Active {
            // Calculate normalized time within the active phase (0.0 to 1.0)
            let active_start_time = current_move.move_metadata.startup_time;
            let active_elapsed = current_move.move_time - active_start_time;
            let active_progress = (active_elapsed / current_move.move_metadata.active_time).clamp(0.0, 1.0);
            
            // Get position and rotation from cubic swing calculation
            let (swing_offset, swing_rotation) = crate::lerp_animation::calculate_vertical_swing_cubic(active_progress, current_move.move_metadata.radius);
            
            // Apply the swing offset to the start position
            let new_position = swing_offset;
            
            // Update transform
            transform.translation.x = new_position.x;
            transform.translation.y = new_position.y;
            transform.rotation = Quat::from_rotation_z(swing_rotation);
        }
    }
}