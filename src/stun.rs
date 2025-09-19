use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
};

/// Component that represents a stun effect preventing movement and attacks
#[derive(Component, Debug)]
pub struct Stun {
    /// Duration remaining in seconds
    pub remaining_duration: f32,
    /// Total duration when stun was applied (for reference)
    pub total_duration: f32,
}

impl Stun {
    /// Create a new stun effect with specified duration
    pub fn new(duration: f32) -> Self {
        Self {
            remaining_duration: duration,
            total_duration: duration,
        }
    }

    /// Check if the stun effect is still active
    pub fn is_active(&self) -> bool {
        self.remaining_duration > 0.0
    }

    /// Get the percentage of stun duration remaining (for UI/feedback)
    pub fn progress_percentage(&self) -> f32 {
        if self.total_duration == 0.0 {
            0.0
        } else {
            (self.remaining_duration / self.total_duration).clamp(0.0, 1.0)
        }
    }
}

/// This struct defines the data that will be passed to the stun shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct StunMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode2d,
}

impl Default for StunMaterial {
    fn default() -> Self {
        Self {
            color: LinearRgba::RED,  // Red tint for stunned entities
            color_texture: None,
            alpha_mode: AlphaMode2d::Blend,
        }
    }
}

impl Material2d for StunMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/stun_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self.alpha_mode
    }
}

/// Plugin to handle stun mechanics
pub struct StunPlugin;

impl Plugin for StunPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<StunMaterial>::default())
            .add_systems(Update, (update_stun_effects, apply_stun_shader));
    }
}

/// System that updates all stun effects, decrementing duration and removing expired stuns
fn update_stun_effects(
    mut commands: Commands,
    mut stun_query: Query<(Entity, &mut Stun)>,
    time: Res<Time>,
) {
    for (entity, mut stun) in stun_query.iter_mut() {
        stun.remaining_duration -= time.delta_secs();

        // Remove stun effect when duration expires
        if !stun.is_active() {
            debug!("Stun effect expired on entity {:?}", entity);
            commands.entity(entity).remove::<Stun>();
        }
    }
}

/// System that applies red shader to newly stunned entities and removes it when stun expires
fn apply_stun_shader(
    mut commands: Commands,
    mut materials: ResMut<Assets<StunMaterial>>,
    stunned_query: Query<(Entity, &MeshMaterial2d<ColorMaterial>), (With<Stun>, Without<MeshMaterial2d<StunMaterial>>)>,
    unstunned_query: Query<Entity, (Without<Stun>, With<MeshMaterial2d<StunMaterial>>)>,
    mut removed: RemovedComponents<Stun>,
) {
    // Apply stun shader to newly stunned entities
    for (entity, original_material) in stunned_query.iter() {
        debug!("Applying stun shader to entity {:?}", entity);
        // Store the original material as a component so we can restore it later
        commands.entity(entity)
            .insert(OriginalMaterial(original_material.clone()))
            .insert(MeshMaterial2d(materials.add(StunMaterial::default())));
    }

    // Remove stun shader from entities that are no longer stunned
    for entity in unstunned_query.iter() {
        debug!("Removing stun shader from entity {:?}", entity);
        commands.entity(entity).remove::<MeshMaterial2d<StunMaterial>>();
    }

    // Handle entities where Stun component was removed
    for entity in removed.read() {
        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            // Try to restore original material if it exists
            entity_commands.remove::<MeshMaterial2d<StunMaterial>>();
        }
    }
}

/// Component to store the original material of an entity before applying stun shader
#[derive(Component, Clone)]
pub struct OriginalMaterial(pub MeshMaterial2d<ColorMaterial>);

/// Helper functions for applying/removing stun effects
impl Stun {
    /// Apply stun to an entity
    pub fn apply_to_entity(commands: &mut Commands, entity: Entity, duration: f32) {
        debug!("Applying stun to entity {:?} for {:.2}s", entity, duration);
        // Check if entity exists before trying to insert component
        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.insert(Stun::new(duration));
        } else {
            debug!("Cannot apply stun to entity {:?} - entity does not exist", entity);
        }
    }

    /// Remove stun from an entity if it exists
    pub fn remove_from_entity(commands: &mut Commands, entity: Entity) {
        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.remove::<Stun>();
        }
    }
}