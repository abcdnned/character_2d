use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::damage::Damage;
use crate::unit::Hp;
use crate::enemy::Enemy;
use crate::physics::*;

pub fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    damage_query: Query<&Damage>,
    mut hp_query: Query<&mut Hp>,
    mut enemy_query: Query<(Entity, &mut Velocity, &Transform), With<Enemy>>,
    weapon_transform_query: Query<&Transform, (With<Damage>, Without<Enemy>)>,
    weapon_knockback_query: Query<&WeaponKnockback>,
    mut commands: Commands,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                // Check if entity1 has damage and entity2 is an enemy with HP
                if let (Ok(damage), Ok(mut hp)) = (
                    damage_query.get(*entity1),
                    hp_query.get_mut(*entity2)
                ) {
                    if let Ok((enemy_entity, mut enemy_velocity, enemy_transform)) = enemy_query.get_mut(*entity2) {
                        let damage_amount = damage.get_amount();
                        let old_hp = hp.hp;
                        
                        hp.hp = (hp.hp - damage_amount).max(0.0);
                        
                        println!("Sword hit! Damage: {:.1} | HP: {:.1} -> {:.1}", 
                            damage_amount, old_hp, hp.hp);
                        
                        // Apply knockback using weapon's knockback settings
                        if let Ok(weapon_transform) = weapon_transform_query.get(*entity1) {
                            if let Ok(weapon_knockback) = weapon_knockback_query.get(*entity1) {
                                apply_knockback_force(
                                    enemy_entity,
                                    &mut enemy_velocity,
                                    enemy_transform,
                                    weapon_transform,
                                    weapon_knockback,
                                    &mut commands,
                                );
                            }
                        }
                    }
                }
                // Check the reverse case (entity2 has damage, entity1 is enemy)
                else if let (Ok(damage), Ok(mut hp)) = (
                    damage_query.get(*entity2),
                    hp_query.get_mut(*entity1)
                ) {
                    if let Ok((enemy_entity, mut enemy_velocity, enemy_transform)) = enemy_query.get_mut(*entity1) {
                        let damage_amount = damage.get_amount();
                        let old_hp = hp.hp;
                        
                        hp.hp = (hp.hp - damage_amount).max(0.0);
                        
                        println!("Sword hit! Damage: {:.1} | HP: {:.1} -> {:.1}", 
                            damage_amount, old_hp, hp.hp);
                        
                        // Apply knockback using weapon's knockback settings
                        if let Ok(weapon_transform) = weapon_transform_query.get(*entity2) {
                            if let Ok(weapon_knockback) = weapon_knockback_query.get(*entity2) {
                                apply_knockback_force(
                                    enemy_entity,
                                    &mut enemy_velocity,
                                    enemy_transform,
                                    weapon_transform,
                                    weapon_knockback,
                                    &mut commands,
                                );
                            }
                        }
                    }
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _) => {}
        }
    }
}