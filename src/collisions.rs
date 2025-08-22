use crate::damage::Damage;
use crate::enemy::Enemy;
use crate::global_entity_map::GlobalEntityMap;
use crate::particle::ParticleMaterialAsset;
use crate::physics::*;
use crate::r#move::{ExecuteMoveEvent, MoveInput, MoveType, PlayerMove};
use crate::constants::REFLECT; // Assuming REFLECT is defined in constants
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_enoki::prelude::*;
use std::collections::HashSet;

pub fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    damage_query: Query<&Damage>,
    transform_query: Query<&Transform>,
    mut unit_query: Query<&mut crate::unit::Unit>,
    mut enemy_query: Query<(Entity, &mut Velocity, &Transform), With<Enemy>>,
    weapon_knockback_query: Query<&WeaponKnockback>,
    move_query: Query<&PlayerMove>,
    mut move_events: EventWriter<ExecuteMoveEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    material: Res<ParticleMaterialAsset>,
    global_entities: Res<GlobalEntityMap>,
) {
    let mut processed_damage_pairs: HashSet<(Entity, Entity)> = HashSet::new();

    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                // Check if both entities have Damage components
                let entity1_has_damage = damage_query.get(*entity1).is_ok();
                let entity2_has_damage = damage_query.get(*entity2).is_ok();

                if entity1_has_damage && entity2_has_damage {
                    // Both have damage - ensure we only process once
                    let pair = if entity1 < entity2 {
                        (*entity1, *entity2)
                    } else {
                        (*entity2, *entity1)
                    };
                    
                    if !processed_damage_pairs.contains(&pair) {
                        processed_damage_pairs.insert(pair);
                        println!("Collision between two damage entities: {:?} and {:?}", entity1, entity2);
                        
                        // Handle move interaction logic
                        handle_move_interaction(
                            *entity1,
                            *entity2,
                            &damage_query,
                            &move_query,
                            &mut move_events,
                            &global_entities,
                        );
                    }
                } else {
                    // Normal processing for non-damage-damage collisions
                    process_hit(
                        *entity1,
                        *entity2,
                        &damage_query,
                        &transform_query,
                        &mut unit_query,
                        &mut enemy_query,
                        &weapon_knockback_query,
                        &mut commands,
                        &asset_server,
                        &material,
                    );
                    process_hit(
                        *entity2,
                        *entity1,
                        &damage_query,
                        &transform_query,
                        &mut unit_query,
                        &mut enemy_query,
                        &weapon_knockback_query,
                        &mut commands,
                        &asset_server,
                        &material,
                    );
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

fn handle_move_interaction(
    entity1: Entity,
    entity2: Entity,
    damage_query: &Query<&Damage>,
    move_query: &Query<&PlayerMove>,
    move_events: &mut EventWriter<ExecuteMoveEvent>,
    global_entities: &Res<GlobalEntityMap>,
) {
    // Get damage components to access source entities
    let damage1 = damage_query.get(entity1);
    let damage2 = damage_query.get(entity2);
    
    if let (Ok(dmg1), Ok(dmg2)) = (damage1, damage2) {
        // Get move components from the source entities
        let move1 = move_query.get(dmg1.source);
        let move2 = move_query.get(dmg2.source);
        
        if let (Ok(player_move1), Ok(player_move2)) = (move1, move2) {
            let move_type1 = &player_move1.move_metadata.move_type;
            let move_type2 = &player_move2.move_metadata.move_type;
            
            // Check for Swing vs Stub interaction
            match (move_type1, move_type2) {
                (MoveType::Swing, MoveType::Stub) => {
                    // Stub counters Swing - find weapon entity and trigger REFLECT move
                    if let Some(&weapon_entity) = global_entities.player_weapon.get(&dmg2.source) {
                        println!("Move interaction: Swing vs Stub - Stub performer triggers REFLECT");
                        move_events.write(ExecuteMoveEvent {
                            entity: weapon_entity,
                            move_name: REFLECT.to_string(),
                            move_input: MoveInput::Interrupt,
                        });
                    } else {
                        println!("Could not find weapon entity for player: {:?}", dmg2.source);
                    }
                }
                (MoveType::Stub, MoveType::Swing) => {
                    // Stub counters Swing - find weapon entity and trigger REFLECT move
                    if let Some(&weapon_entity) = global_entities.player_weapon.get(&dmg1.source) {
                        println!("Move interaction: Stub vs Swing - Stub performer triggers REFLECT");
                        move_events.write(ExecuteMoveEvent {
                            entity: weapon_entity,
                            move_name: REFLECT.to_string(),
                            move_input: MoveInput::Interrupt,
                        });
                    } else {
                        println!("Could not find weapon entity for player: {:?}", dmg1.source);
                    }
                }
                _ => {
                    // Other combinations - just log
                    println!("Move interaction: {:?} vs {:?} - No special interaction", move_type1, move_type2);
                }
            }
        } else {
            println!("Could not retrieve move components from source entities");
        }
    } else {
        println!("Could not retrieve damage components for move interaction");
    }
}

fn process_hit(
    attacker: Entity,
    target: Entity,
    damage_query: &Query<&Damage>,
    transform_query: &Query<&Transform>,
    unit_query: &mut Query<&mut crate::unit::Unit>,
    enemy_query: &mut Query<(Entity, &mut Velocity, &Transform), With<Enemy>>,
    weapon_knockback_query: &Query<&WeaponKnockback>,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    material: &Res<ParticleMaterialAsset>,
) {
    if let (Ok(damage), Ok(mut tu)) = (damage_query.get(attacker), unit_query.get_mut(target)) {
        if let Ok((enemy_entity, mut enemy_velocity, enemy_transform)) = enemy_query.get_mut(target)
        {
            let damage_amount = damage.get_amount();
            let old_hp = tu.hp;
            tu.hp = (tu.hp - damage_amount).max(0.0);
            
            // Spawn hit particle effect at enemy position
            commands.spawn((
                ParticleEffectHandle(asset_server.load("hitten.ron")),
                Transform::from_translation(enemy_transform.translation),
                Name::new("HitEffect"),
                ParticleSpawner(material.0.clone()),
                OneShot::Despawn,
            ));
            
            println!(
                "Sword hit! Damage: {:.1} | HP: {:.1} -> {:.1}",
                damage_amount, old_hp, tu.hp
            );
            
            if let (Ok(weapon_knockback), Ok(source_transform)) = (
                weapon_knockback_query.get(attacker),
                transform_query.get(damage.source),
            ) {
                apply_knockback_force(
                    enemy_entity,
                    &mut enemy_velocity,
                    enemy_transform,
                    source_transform,
                    weapon_knockback,
                    commands,
                );
            }
        }
    }
}