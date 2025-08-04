use crate::damage::Damage;
use crate::enemy::Enemy;
use crate::physics::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    damage_query: Query<&Damage>,
    transform_query: Query<&Transform>,
    mut unit_query: Query<&mut crate::unit::Unit>,
    mut enemy_query: Query<(Entity, &mut Velocity, &Transform), With<Enemy>>,
    weapon_knockback_query: Query<&WeaponKnockback>,
    mut commands: Commands,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                process_hit(
                    *entity1,
                    *entity2,
                    &damage_query,
                    &transform_query,
                    &mut unit_query,
                    &mut enemy_query,
                    &weapon_knockback_query,
                    &mut commands,
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
                );
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
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
) {
    if let (Ok(damage), Ok(mut hp)) = (damage_query.get(attacker), unit_query.get_mut(target)) {
        if let Ok((enemy_entity, mut enemy_velocity, enemy_transform)) = enemy_query.get_mut(target)
        {
            let damage_amount = damage.get_amount();
            let old_hp = hp.hp;

            hp.hp = (hp.hp - damage_amount).max(0.0);

            println!(
                "Sword hit! Damage: {:.1} | HP: {:.1} -> {:.1}",
                damage_amount, old_hp, hp.hp
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
