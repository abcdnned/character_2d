//! Sword Trail Plugin
//!
//! This plugin creates particle trails for entities with the SwordTrail component.
//! The trail follows the entity's movement and creates a ribbon-like effect that
//! resembles a sword slash or magical weapon trail.

use bevy::color::palettes::basic::*;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

/// Plugin that handles sword trail effects
pub struct SwordTrailPlugin;

impl Plugin for SwordTrailPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin);
        app.add_systems(Update, (spawn_sword_trails, despawn_sword_trails));
    }
}

/// Component that marks an entity as having a sword trail
#[derive(Component)]
pub struct SwordTrail {
    /// Color of the trail
    pub color: Color,
    /// Width of the trail
    pub width: f32,
    /// How long the trail lasts (in seconds)
    pub lifetime: f32,
    /// How fast particles are spawned (particles per second)
    pub spawn_rate: f32,
}

const PARTICLE_CAPACITY: u32 = 200;

fn spawn_sword_trails(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    query: Query<(Entity, &SwordTrail), (Without<ParticleEffect>, With<Transform>)>,
) {
    for (entity, sword_trail) in query.iter() {
        // Create a new effect customized for this sword trail
        let writer = ExprWriter::new();

        // Initialize particle attributes with custom values
        let init_position_attr = SetAttributeModifier {
            attribute: Attribute::POSITION,
            value: writer.lit(Vec3::ZERO).expr(),
        };

        let init_age_attr = SetAttributeModifier {
            attribute: Attribute::AGE,
            value: writer.lit(0.0).expr(),
        };

        let init_lifetime_attr = SetAttributeModifier {
            attribute: Attribute::LIFETIME,
            value: writer.lit(sword_trail.lifetime).expr(),
        };

        let init_size_attr = SetAttributeModifier {
            attribute: Attribute::SIZE,
            value: writer.lit(sword_trail.width).expr(),
        };

        let init_ribbon_id = SetAttributeModifier {
            attribute: Attribute::RIBBON_ID,
            value: writer.lit(0u32).expr(),
        };

        // Create custom color gradient
        let color_vec = Vec4::new(
            sword_trail.color.to_srgba().red * 2.0,
            sword_trail.color.to_srgba().green * 2.0,
            sword_trail.color.to_srgba().blue * 2.0,
            1.0,
        );
        let transparent_color = Vec4::new(
            sword_trail.color.to_srgba().red * 2.0,
            sword_trail.color.to_srgba().green * 2.0,
            sword_trail.color.to_srgba().blue * 2.0,
            0.0,
        );

        let render_color =
            ColorOverLifetimeModifier::new(Gradient::linear(color_vec, transparent_color));

        let size_over_lifetime = SizeOverLifetimeModifier {
            gradient: Gradient::linear(Vec3::splat(sword_trail.width), Vec3::ZERO),
            ..default()
        };

        let spawner = SpawnerSettings::rate(sword_trail.spawn_rate.into());

        let custom_effect = EffectAsset::new(PARTICLE_CAPACITY, spawner, writer.finish())
            .with_name("sword_trail")
            .with_motion_integration(MotionIntegration::None)
            .with_simulation_space(SimulationSpace::Global)
            .init(init_position_attr)
            .init(init_age_attr)
            .init(init_lifetime_attr)
            .init(init_size_attr)
            .init(init_ribbon_id)
            .render(size_over_lifetime)
            .render(render_color);

        let custom_effect_handle = effects.add(custom_effect);

        // Add the particle effect directly to the sword entity
        commands
            .entity(entity)
            .insert(ParticleEffect::new(custom_effect_handle));
    }
}

fn despawn_sword_trails(
    mut commands: Commands,
    query: Query<Entity, (Without<SwordTrail>, With<ParticleEffect>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).remove::<ParticleEffect>();
        commands.entity(entity).remove::<EffectSpawner>();
        commands.entity(entity).remove::<CompiledParticleEffect>();
        info!("sword trail completely cleaned up");
    }
}

// Helper functions for common sword trail configurations
impl SwordTrail {
    pub fn new() -> Self {
        Self {
            color: Color::from(RED).with_alpha(0.5),
            width: 10.,
            lifetime: 0.3,
            spawn_rate: 300.0,
        }
    }
}
