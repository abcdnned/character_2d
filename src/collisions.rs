use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::Player;
use crate::constants::*;

pub fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                println!("Collision started between {:?} and {:?}", entity1, entity2);
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {}
        }
    }
}