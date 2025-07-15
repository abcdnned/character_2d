use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::damage::Damage;
use crate::unit::Hp;
use crate::enemy::Enemy;

pub fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    damage_query: Query<&Damage>,
    mut hp_query: Query<&mut Hp>,
    enemy_query: Query<&Enemy>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                // Check if entity1 has damage and entity2 is an enemy with HP
                if let (Ok(damage), Ok(mut hp)) = (
                    damage_query.get(*entity1),
                    hp_query.get_mut(*entity2)
                ) {
                    if enemy_query.get(*entity2).is_ok() {
                        let damage_amount = damage.get_amount();
                        let old_hp = hp.hp;
                        
                        hp.hp = (hp.hp - damage_amount).max(0.0);
                        
                        println!("Sword hit! Damage: {:.1} | HP: {:.1} -> {:.1}", 
                            damage_amount, old_hp, hp.hp);
                    }
                }
                // Check the reverse case (entity2 has damage, entity1 is enemy)
                else if let (Ok(damage), Ok(mut hp)) = (
                    damage_query.get(*entity2),
                    hp_query.get_mut(*entity1)
                ) {
                    if enemy_query.get(*entity1).is_ok() {
                        let damage_amount = damage.get_amount();
                        let old_hp = hp.hp;
                        
                        hp.hp = (hp.hp - damage_amount).max(0.0);
                        
                        println!("Sword hit! Damage: {:.1} | HP: {:.1} -> {:.1}", 
                            damage_amount, old_hp, hp.hp);
                    }
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {}
        }
    }
}