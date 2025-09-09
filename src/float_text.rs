use bevy::prelude::*;
use bevy_tweening::{lens::TransformPositionLens, *};
use std::time::Duration;

/// Component to mark floating text entities for cleanup
#[derive(Component)]
pub struct FloatingText {
    pub lifetime: Duration,
    pub timer: Timer,
}

impl FloatingText {
    pub fn new(lifetime: Duration) -> Self {
        Self {
            lifetime,
            timer: Timer::new(lifetime, TimerMode::Once),
        }
    }
}

/// Configuration for floating text
pub struct FloatingTextConfig {
    pub text: String,
    pub color: Color,
    pub position: Vec3,
    pub lifetime: Duration,
    pub font_size: f32,
    pub float_distance: f32,
}

impl Default for FloatingTextConfig {
    fn default() -> Self {
        Self {
            text: "Default Text".to_string(),
            color: Color::WHITE,
            position: Vec3::ZERO,
            lifetime: Duration::from_secs(2),
            font_size: 24.0,
            float_distance: 100.0,
        }
    }
}

/// Utility function to spawn floating text
pub fn spawn_floating_text(
    commands: &mut Commands,
    config: FloatingTextConfig,
) -> Entity {
    // Create the position tween animation
    let position_tween = Tween::new(
        EaseFunction::QuadraticOut,
        config.lifetime,
        TransformPositionLens {
            start: config.position,
            end: config.position + Vec3::new(0.0, config.float_distance, 0.0),
        },
    );

    // Spawn the floating text entity using the new Bevy text API
    commands.spawn((
        Text2d::new(config.text),
        TextFont {
            font_size: config.font_size,
            ..default()
        },
        TextColor(config.color),
        Transform::from_translation(config.position),
        FloatingText::new(config.lifetime),
        Animator::new(position_tween),
    )).id()
}

/// Convenience function for critical hit text
pub fn spawn_critical_hit_text(
    commands: &mut Commands,
    position: Vec3,
) -> Entity {
    spawn_floating_text(commands, FloatingTextConfig {
        text: "CRITICAL HIT!!".to_string(),
        color: Color::srgb(1.0, 0.2, 0.2), // Red color
        position,
        lifetime: Duration::from_millis(1500),
        font_size: 36.0,
        float_distance: 120.0,
    })
}


/// System to cleanup floating text after their lifetime expires
pub fn cleanup_floating_text_system(
    mut commands: Commands,
    time: Res<Time>,
    mut floating_texts: Query<(Entity, &mut FloatingText)>,
) {
    for (entity, mut floating_text) in floating_texts.iter_mut() {
        floating_text.timer.tick(time.delta());
        if floating_text.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Plugin to add floating text functionality to your Bevy app
pub struct FloatingTextPlugin;

impl Plugin for FloatingTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cleanup_floating_text_system);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floating_text_config_default() {
        let config = FloatingTextConfig::default();
        assert_eq!(config.text, "Default Text");
        assert_eq!(config.color, Color::WHITE);
        assert_eq!(config.position, Vec3::ZERO);
        assert_eq!(config.lifetime, Duration::from_secs(2));
        assert_eq!(config.font_size, 24.0);
        assert_eq!(config.float_distance, 100.0);
    }

    #[test]
    fn test_floating_text_component() {
        let lifetime = Duration::from_secs(3);
        let floating_text = FloatingText::new(lifetime);
        assert_eq!(floating_text.lifetime, lifetime);
        assert!(!floating_text.timer.finished());
    }
}