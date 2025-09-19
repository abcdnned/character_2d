use bevy::prelude::*;

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

/// Plugin to handle stun mechanics
pub struct StunPlugin;

impl Plugin for StunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_stun_effects);
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

/// Helper functions for applying/removing stun effects
impl Stun {
    /// Apply stun to an entity
    pub fn apply_to_entity(commands: &mut Commands, entity: Entity, duration: f32) {
        debug!("Applying stun to entity {:?} for {:.2}s", entity, duration);
        commands.entity(entity).insert(Stun::new(duration));
    }

    /// Remove stun from an entity if it exists
    pub fn remove_from_entity(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).remove::<Stun>();
    }
}