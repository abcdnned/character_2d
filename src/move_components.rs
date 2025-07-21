use crate::global_weapon_collider::WeaponColliderMap;
use crate::r#move::*;
use crate::sword_trail::SwordTrail;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct MoveComponentsPlugin;

impl Plugin for MoveComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartMoveEvent>()
            .add_event::<EndMoveEvent>()
            .add_systems(Update, (handle_start_move, handle_end_move));
    }
}

fn handle_start_move(
    mut commands: Commands,
    mut start_move_events: EventReader<StartMoveEvent>,
    weapon_map: Res<WeaponColliderMap>,
) {
    for event in start_move_events.read() {
        if let Some(collider_entity) = weapon_map.get(event.actor) {
            info!(
                "StartMove: Actor {:?} started move '{}', enabling collider {:?}",
                event.actor, event.move_name, collider_entity
            );

            // Add SwordTrail component to the collider
            commands.entity(collider_entity).insert(SwordTrail::new());

            // Remove ColliderDisabled component to enable collision detection
            commands
                .entity(collider_entity)
                .remove::<ColliderDisabled>();
        } else {
            warn!(
                "StartMove: No weapon collider found for actor {:?}",
                event.actor
            );
        }
    }
}

fn handle_end_move(
    mut commands: Commands,
    mut end_move_events: EventReader<EndMoveEvent>,
    weapon_map: Res<WeaponColliderMap>,
) {
    for event in end_move_events.read() {
        if let Some(collider_entity) = weapon_map.get(event.actor) {
            info!(
                "EndMove: Actor {:?} ended move '{}', disabling collider {:?}",
                event.actor, event.move_name, collider_entity
            );

            // Remove SwordTrail component
            commands.entity(collider_entity).remove::<SwordTrail>();

            // Add ColliderDisabled component to disable collision detection
            commands.entity(collider_entity).insert(ColliderDisabled);
        } else {
            warn!(
                "EndMove: No weapon collider found for actor {:?}",
                event.actor
            );
        }
    }
}
