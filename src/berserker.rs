use crate::{particle::ParticleMaterialAsset, unit::HpChangeEvent};
use bevy::prelude::*;
use bevy_enoki::{prelude::OneShot, ParticleEffectHandle, ParticleSpawner};

#[derive(Component)]
pub struct Berserker {
    pub level: i32,
}

#[derive(Component)]
pub struct SacrificeTimer {
    pub timer: Timer,
}

pub struct BerserkerPlugin;

impl Plugin for BerserkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BerserkerHealEvent>()
            .add_event::<BerserkerActiveEvent>()
            .add_systems(Update, (berserker_lifesteal, berserker_active_handler, sacrificed_hp));
    }
}

#[derive(Event)]
pub struct BerserkerHealEvent {
    pub entity: Entity,
    pub source: Entity,
    pub old_hp: f32,
    pub new_hp: f32,
    pub max_hp: f32,
}

#[derive(Event)]
pub struct BerserkerActiveEvent {
    pub entity: Entity,
}

/// System that decreases HP every 0.1 seconds when berserker is at level 1
pub fn sacrificed_hp(
    mut commands: Commands,
    time: Res<Time>,
    mut berserker_query: Query<(Entity, &Berserker, &mut crate::unit::Unit, Option<&mut SacrificeTimer>)>,
    mut hp_event_writer: EventWriter<HpChangeEvent>,
) {
    for (entity, berserker, mut unit, timer_option) in berserker_query.iter_mut() {
        if berserker.level == 1 {
            // Add timer component if it doesn't exist
            if timer_option.is_none() {
                commands.entity(entity).insert(SacrificeTimer {
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                });
                continue;
            }
            
            if let Some(mut sacrifice_timer) = timer_option {
                sacrifice_timer.timer.tick(time.delta());
                
                if sacrifice_timer.timer.just_finished() {
                    let old_hp = unit.hp;
                    if unit.hp > 0.5 {
                        unit.hp -= 0.5;
                        
                        hp_event_writer.write(HpChangeEvent {
                            entity,
                            source: entity, // The berserker is sacrificing its own HP
                            old_hp,
                            new_hp: unit.hp,
                            max_hp: unit.max_hp,
                            change_type: crate::unit::HpChangeType::Damage,
                        });
                        
                        debug!(
                            "Berserker {:?} sacrificed HP: {} -> {} (-0.5)",
                            entity, old_hp, unit.hp
                        );
                    } else {
                        // If HP would go below 0.5, set it to 0.1 to keep the unit barely alive
                        unit.hp = 0.0;
                        
                        hp_event_writer.write(HpChangeEvent {
                            entity,
                            source: entity,
                            old_hp,
                            new_hp: unit.hp,
                            max_hp: unit.max_hp,
                            change_type: crate::unit::HpChangeType::SetValue,
                        });
                        
                        debug!(
                            "Berserker {:?} sacrificed HP (minimal): {} -> 0.1",
                            entity, old_hp
                        );
                    }
                }
            }
        } else {
            // Remove timer component if berserker is not at level 1
            if timer_option.is_some() {
                commands.entity(entity).remove::<SacrificeTimer>();
                debug!("Removed SacrificeTimer from berserker {:?} (level {})", entity, berserker.level);
            }
        }
    }
}

/// System that heals berserkers when they deal damage to enemies
pub fn berserker_lifesteal(
    mut hp_events: EventReader<HpChangeEvent>,
    mut berserker_uni_query: Query<&mut crate::unit::Unit, With<Berserker>>,
    berserker_query: Query<&Berserker>,
    berserker_check_query: Query<Entity, With<Berserker>>,
    mut event_writer: EventWriter<BerserkerHealEvent>,
) {
    for event in hp_events.read() {
        debug!(
            "Checking HpChangeEvent for berserker lifesteal: source={:?}",
            event.source
        );

        // Check if the damage source is a berserker
        if berserker_check_query.contains(event.source) && event.source != event.entity {
            debug!("Damage source is a berserker, applying lifesteal");

            // Calculate the damage dealt (old_hp - new_hp)
            let damage_dealt = event.old_hp - event.new_hp;

            if damage_dealt > 0.0 {

                // Heal the berserker for the amount of damage dealt
                if let Ok(mut berserker_unit) = berserker_uni_query.get_mut(event.source) {
                    if let Ok(berserker) = berserker_query.get(event.source) {
                        if (berserker.level == 0) {
                            let old_hp = berserker_unit.hp;
                            let entity = event.source; // The berserker entity being healed
                            let source = event.source; // The berserker is also the source of the healing

                            berserker_unit.berserker_heal(damage_dealt, entity, source, &mut event_writer);

                            info!(
                                "Berserker healed from {} to {} HP (gained {})",
                                old_hp,
                                berserker_unit.hp,
                                berserker_unit.hp - old_hp
                            );
                        } else {
                            trace!(
                                "Berserker active for berserker entity: {:?}, no lifesteal",
                                event.source
                            );
                        }
                    } else {
                        warn!(
                            "failed to get berserker component for berserker entity: {:?}",
                            event.source
                        );
                    }
                } else {
                    warn!(
                        "Failed to get Unit component for berserker entity: {:?}",
                        event.source
                    );
                }
            } else {
                debug!("No damage dealt, no healing applied");
            }
        }
    }
}

pub fn berserker_active_handler(
    mut berserker_active_events: EventReader<BerserkerActiveEvent>,
    mut berserker_query: Query<&mut Berserker>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    material: Res<ParticleMaterialAsset>,
    transform_query: Query<&Transform>,
) {
    for event in berserker_active_events.read() {
        if let Ok(mut berserker) = berserker_query.get_mut(event.entity) {
            let old_level = berserker.level;
            berserker.level = if berserker.level == 0 { 1 } else { 0 };
            info!(
                "Berserker active state changed for entity {:?}: level {} -> {}",
                event.entity, old_level, berserker.level
            );
            
            if let Ok(transform) = transform_query.get(event.entity) {
                if berserker.level == 1 {
                    // Spawn the particle effect as a child of the berserker entity
                    let particle_entity = commands.spawn((
                        ParticleEffectHandle(asset_server.load("berserker_active.ron")),
                        Transform::from_translation(Vec3::ZERO), // Use local position relative to parent
                        ParticleSpawner(material.0.clone()),
                        OneShot::Despawn,
                    )).id();
                    
                    // Make the particle effect a child of the berserker entity
                    commands.entity(event.entity).add_child(particle_entity);
                    
                    info!("Created Berserker effect as child of entity {:?}", event.entity);
                }
            } else {
                warn!(
                    "Transform received failed for entity {:?}",
                    event.entity
                );
            }
        } else {
            warn!(
                "BerserkerActiveEvent received for entity {:?} without Berserker component",
                event.entity
            );
        }
    }
}