use crate::animation_base::*;
use crate::constants::DURATION_FACTOR;
use crate::global_entity_map::GlobalEntityMap;
use crate::move_database::*;
use crate::physics::WeaponKnockback;
use bevy::prelude::*;

#[derive(Component)]
pub struct Move {
    pub move_metadata: MoveMetadata,
    pub move_time: f32,
    pub current_phase: MovePhase,
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

    pub fn transition_to(&mut self, next_metadata: MoveMetadata, weapon: Entity, command: &mut Commands) {
        trace!(
            "Transitioning to next move: {} from {}",
            next_metadata.name, self.move_metadata.name
        );

        let m = Self::new(next_metadata, self.actor);
        command.entity(weapon).insert(m);
    }

    pub fn can_accept_input(&self, input: &MoveInput) -> bool {
        matches!(self.current_phase, MovePhase::Active | MovePhase::Recovery)
            && self.move_metadata.accept_input == *input
    }

    pub fn total_duration(&self) -> f32 {
        self.move_metadata.startup_time
            + self.move_metadata.active_time
            + self.move_metadata.recovery_time
    }

    pub fn update_phase(&mut self) -> (MovePhase, bool) {
        let previous_phase = self.current_phase;

        let new_phase = if self.move_time < self.move_metadata.startup_time {
            MovePhase::Startup
        } else if self.move_time < self.move_metadata.startup_time + self.move_metadata.active_time
        {
            MovePhase::Active
        } else if self.move_time < self.total_duration() {
            MovePhase::Recovery
        } else {
            return (MovePhase::Recovery, true); // Move completed
        };

        let phase_changed = previous_phase != new_phase;
        if phase_changed {
            trace!(
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
pub struct PlayerMove {
    pub move_metadata: MoveMetadata,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum MovePhase {
    Startup,
    Active,
    Recovery,
}

#[derive(Clone, PartialEq, Debug)]
pub enum MoveType {
    Swing,
    Stub,
    Interrupt,
}

#[derive(Clone, PartialEq, Debug)]
pub enum MoveInput {
    Attack,
    Interrupt,
    None,
}

#[derive(Event)]
pub struct ExecuteMoveEvent {
    pub entity: Entity,
    pub move_name: String,
    pub move_input: MoveInput,
}

#[derive(Clone)]
pub struct MoveMetadata {
    pub name: String,
    pub radius: f32,
    pub startup_time: f32,
    pub active_time: f32,
    pub recovery_time: f32,
    pub move_type: MoveType,
    pub accept_input: MoveInput,
    pub next_move: Option<String>,
    pub kb_force: f32,
    pub critical_rate: f32,
    pub best_range_min: f32,
    pub move_speed: f32,
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
            .add_systems(Update, handle_move_execution)
            .add_systems(FixedUpdate, update_moves);
    }
}

fn handle_move_execution(
    mut commands: Commands,
    mut move_events: EventReader<ExecuteMoveEvent>,
    move_db: Res<MoveDatabase>,
    mut query: Query<(Entity, Option<&mut Move>)>,
    global_entity_map: Res<GlobalEntityMap>,
    mut weapon_knockback_query: Query<&mut WeaponKnockback>,
    mut end_move_events: EventWriter<MoveRecoveryEvent>,
) {
    for event in move_events.read() {
        if let Ok((entity, current_move)) = query.get_mut(event.entity) {
            // Handle interrupt input - immediately start new move regardless of current state
            if event.move_input == MoveInput::Interrupt {
                debug!(
                    "Interrupt input received for entity {:?}, force-starting move: {}",
                    entity, event.move_name
                );

                // Send recovery event for interrupted move if one exists
                if let Some(current) = current_move.as_ref() {
                    end_move_events.write(MoveRecoveryEvent {
                        actor: current.actor,
                        move_name: current.move_metadata.name.clone(),
                    });
                    debug!(
                        "Sent MoveRecoveryEvent for interrupted move: {}",
                        current.move_metadata.name
                    );
                }

                force_start_move(
                    &mut commands,
                    &event,
                    &move_db,
                    entity,
                    &global_entity_map,
                    &mut weapon_knockback_query,
                );
                continue;
            }

            // Handle normal move chaining for non-interrupt inputs
            if let Some(mut current) = current_move {
                handle_move_chaining(&mut current, &event, &move_db, entity);
                continue;
            }

            // Start new move if no current move exists
            start_new_move(
                &mut commands,
                &event,
                &move_db,
                entity,
                &global_entity_map,
                &mut weapon_knockback_query,
            );
        }
    }
}

fn force_start_move(
    commands: &mut Commands,
    event: &ExecuteMoveEvent,
    move_db: &MoveDatabase,
    entity: Entity,
    global_entity_map: &GlobalEntityMap,
    weapon_knockback_query: &mut Query<&mut WeaponKnockback>,
) {
    if let Some(move_data) = move_db.moves.get(&event.move_name) {
        if let Some(actor) = global_entity_map.weapon_player.get(&event.entity) {
            // Force remove any existing move component and replace with new one
            let new_move = Move::new(move_data.clone(), *actor);
            commands.entity(entity).insert(new_move);
            commands.entity(*actor).insert(PlayerMove {
                move_metadata: move_data.clone(),
            });

            debug!(
                "Force-started interrupt move '{}' for entity {:?}, overriding any existing move",
                event.move_name, entity
            );
        }
    } else {
        warn!("Interrupt move '{}' not found in database", event.move_name);
    }
}

fn handle_move_chaining(
    current: &mut Move,
    event: &ExecuteMoveEvent,
    move_db: &MoveDatabase,
    entity: Entity,
) {
    if !current.can_accept_input(&event.move_input) {
        // debug!(
        //     "Entity {:?} is busy executing move: {} (phase: {:?}, cannot accept input: {:?})",
        //     entity, current.move_metadata.name, current.current_phase, event.move_input
        // );
        return;
    }

    if let Some(next_move_name) = current.move_metadata.next_move.clone() {
        if let Some(next_move_data) = move_db.moves.get(&next_move_name) {
            current.next_move = Some(next_move_data.clone());
            // debug!(
            //     "Queued next move '{}' for entity {:?} during {:?} phase",
            //     next_move_name, entity, current.current_phase
            // );
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
    global_entity_map: &GlobalEntityMap,
    mut weapon_knockback_query: &mut Query<&mut WeaponKnockback>,
) {
    if let Some(move_data) = move_db.moves.get(&event.move_name) {
        if let Some(actor) = global_entity_map.weapon_player.get(&event.entity) {
            let new_move = Move::new(move_data.clone(), *actor);
            commands.entity(entity).insert(new_move);
            commands.entity(*actor).insert(PlayerMove {
                move_metadata: move_data.clone(),
            });

            // Update knockback using the extracted method
            update_knockback(entity, move_data, global_entity_map, weapon_knockback_query);

            trace!(
                "Added PlayerMove component to player entity {:?}",
                event.entity
            );
            trace!(
                "Entity {:?} started executing move: {}",
                entity, event.move_name
            );
        }
    } else {
        warn!("Move '{}' not found in database", event.move_name);
    }
}

fn update_knockback(
    entity: Entity,
    move_data: &MoveMetadata,
    global_entity_map: &GlobalEntityMap,
    weapon_knockback_query: &mut Query<&mut WeaponKnockback>,
) {
    // Get the collider entity using the weapon_collider map
    if let Some(&collider_entity) = global_entity_map.weapon_collider.get(&entity) {
        // Update the WeaponKnockback component
        if let Ok(mut weapon_knockback) = weapon_knockback_query.get_mut(collider_entity) {
            weapon_knockback.force = move_data.kb_force;
            weapon_knockback.duration = move_data.kb_force * DURATION_FACTOR;

            trace!(
                "Updated WeaponKnockback for collider {:?}: force={}, duration=2.25",
                collider_entity, move_data.kb_force
            );
        } else {
            warn!(
                "WeaponKnockback component not found on collider entity {:?}",
                collider_entity
            );
        }
    } else {
        warn!("No collider entity found for weapon entity {:?}", entity);
    }
}

fn update_moves(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Move, &mut Transform, &crate::weapon::Weapon)>,
    mut player_query: Query<Entity, With<crate::Player>>,
    mut start_move_events: EventWriter<MoveActiveEvent>,
    mut end_move_events: EventWriter<MoveRecoveryEvent>,
    animation_db: Res<AnimationDatabase>,
    time: Res<Time>,
    global_entity_map: Res<GlobalEntityMap>,
    mut weapon_knockback_query: Query<&mut WeaponKnockback>,
) {
    for (entity, mut current_move, mut transform, sword) in query.iter_mut() {
        current_move.move_time += time.delta_secs();

        let previous_phase = current_move.current_phase;
        let (new_phase, move_completed) = current_move.update_phase();

        // Handle phase transition events
        handle_phase_events(
            &mut start_move_events,
            &mut end_move_events,
            &current_move,
            previous_phase,
            new_phase,
        );

        // Handle move completion or chaining
        if move_completed {
            complete_move(
                &mut commands,
                entity,
                &mut transform,
                sword,
                &current_move,
                &mut player_query,
            );
            continue;
        }

    // In the update_moves function, replace the early transition section with this:

    // Handle early transition during recovery
    if new_phase == MovePhase::Recovery && current_move.next_move.is_some() {
        if let Some(next_move_data) = current_move.next_move.take() {
            trace!(
                "Early transition to next move: {} from {} (skipping recovery)",
                next_move_data.name, current_move.move_metadata.name
            );
            
            // Clone the data before moving it into transition_to
            let next_move_data_clone = next_move_data.clone();
            current_move.transition_to(next_move_data, entity, &mut commands);
            
            // Update knockback for the early transition move
            update_knockback(
                entity,
                &next_move_data_clone,
                &global_entity_map,
                &mut weapon_knockback_query,
            );
            
            // FIX: Update PlayerMove component on the player entity
            if let Some(&player_entity) = global_entity_map.weapon_player.get(&entity) {
                commands.entity(player_entity).insert(PlayerMove {
                    move_metadata: next_move_data_clone,
                });
            }
            
            continue;
        }
    }

        // Update position during active phase
        if current_move.current_phase == MovePhase::Active {
            update_move_animation(&mut transform, &current_move, &animation_db);
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
                start_events.write(MoveActiveEvent {
                    actor: current_move.actor,
                    move_name: current_move.move_metadata.name.clone(),
                });
            }
            MovePhase::Recovery => {
                end_events.write(MoveRecoveryEvent {
                    actor: current_move.actor,
                    move_name: current_move.move_metadata.name.clone(),
                });
            }
            _ => {}
        }
    }
}

fn complete_move(
    commands: &mut Commands,
    entity: Entity,
    transform: &mut Transform,
    sword: &crate::weapon::Weapon,
    current_move: &Move,
    player_query: &mut Query<Entity, With<crate::Player>>,
) {
    // Reset position and rotation
    transform.translation = sword.offset;
    transform.rotation = Quat::IDENTITY;

    commands.entity(entity).remove::<Move>();
    trace!(
        "Entity {:?} completed move: {} - position reset to offset",
        entity, current_move.move_metadata.name
    );

    // Remove PlayerMove component
    if let Ok(player_entity) = player_query.single() {
        commands.entity(player_entity).remove::<PlayerMove>();
    }
}

fn update_move_animation(
    transform: &mut Transform,
    current_move: &Move,
    animation_db: &AnimationDatabase,
) {
    let active_progress = current_move.get_active_progress();

    // Query the animation database for the current move's animation function
    if let Some(animation_func) = animation_db
        .animations
        .get(&current_move.move_metadata.name)
    {
        let (swing_offset, swing_rotation) =
            animation_func(active_progress, current_move.move_metadata.radius);

        transform.translation.x = swing_offset.x;
        transform.translation.y = swing_offset.y;
        transform.rotation = Quat::from_rotation_z(swing_rotation);
    } else {
        // Fallback or warning if animation not found
        warn!(
            "No animation found for move: {}",
            current_move.move_metadata.name
        );
    }
}
