use bevy::{
    core_pipeline::bloom::Bloom, 
    diagnostic::DiagnosticsStore, 
    image::ImageSamplerDescriptor,
    prelude::*,
};
use bevy_enoki::{prelude::*, EnokiPlugin};
use std::time::Duration;

/// Plugin for handling particle systems
pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EnokiPlugin)
            .add_systems(Startup, setup_particle_material);
    }
}

/// Resource that holds the particle material handle
#[derive(Deref, Resource, DerefMut)]
pub struct ParticleMaterialAsset(pub Handle<SpriteParticle2dMaterial>);

/// Setup system that initializes the particle material resource
fn setup_particle_material(
    mut cmd: Commands,
    mut materials: ResMut<Assets<SpriteParticle2dMaterial>>,
    server: Res<AssetServer>,
) {
    cmd.insert_resource(ParticleMaterialAsset(materials.add(
        SpriteParticle2dMaterial::new(server.load("particle.png"), 6, 1),
    )));
}