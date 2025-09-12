use crate::{particle::ParticleMaterialAsset, unit::HpChangeEvent};
use bevy::prelude::*;
use bevy_enoki::{prelude::OneShot, ParticleEffectHandle, ParticleSpawner};

#[derive(Component)]
pub struct Berserker {
    pub level: i32,
}

pub struct BerserkerPlugin;

impl Plugin for BerserkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BerserkerHealEvent>()
            .add_event::<BerserkerActiveEvent>()
            .add_systems(Update, (berserker_lifesteal, berserker_active_handler));
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
        if berserker_check_query.contains(event.source) {
            debug!("Damage source is a berserker, applying lifesteal");

            // Calculate the damage dealt (old_hp - new_hp)
            let damage_dealt = event.old_hp - event.new_hp;

            if damage_dealt > 0.0 {
                debug!(
                    "Berserker dealt {} damage, healing for the same amount",
                    damage_dealt
                );

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
                            debug!(
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