use crate::global_entity_map::GlobalEntityMap;
use crate::custom_move::*;
use crate::sword_trail::SwordTrail;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct MoveComponentsPlugin;

impl Plugin for MoveComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveActiveEvent>()
            .add_event::<MoveRecoveryEvent>()
            .add_systems(Update, (handle_start_move, handle_end_move));
    }
}

fn handle_start_move(
    mut commands: Commands,
    mut start_move_events: EventReader<MoveActiveEvent>,
    global_entity: Res<GlobalEntityMap>,
) {
    for event in start_move_events.read() {
        if let Some(collider_entity) = global_entity.player_to_collider.get(&event.actor) {
            // Remove ColliderDisabled component to enable collision detection
            commands
                .entity(*collider_entity)
                .remove::<ColliderDisabled>();
        }
        if let Some(tip) = global_entity.player_sword_trail.get(&event.actor) {
            // Add SwordTrail component to the collider
            commands.entity(*tip).insert(SwordTrail::new());
        }
    }
}

fn handle_end_move(
    mut commands: Commands,
    mut end_move_events: EventReader<MoveRecoveryEvent>,
    global_entities: Res<GlobalEntityMap>,
) {
    for event in end_move_events.read() {
        if let Some(collider_entity) = global_entities.player_to_collider.get(&event.actor) {
            commands.entity(*collider_entity).insert(ColliderDisabled);
        }
        if let Some(tip) = global_entities.player_sword_trail.get(&event.actor) {
            commands.entity(*tip).remove::<SwordTrail>();
        }
    }
}
