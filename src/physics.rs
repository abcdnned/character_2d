use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct KnockbackTimer {
    pub timer: Timer,
}

pub fn update_knockback_timers(
    mut knockback_query: Query<(Entity, &mut KnockbackTimer, &mut Velocity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut knockback_timer, mut velocity) in knockback_query.iter_mut() {
        knockback_timer.timer.tick(time.delta());
        
        if knockback_timer.timer.finished() {
            // Gradually reduce velocity or reset it
            velocity.linvel *= 0.8; // Damping effect
            commands.entity(entity).remove::<KnockbackTimer>();
        }
    }
}

#[derive(Component)]
pub struct WeaponKnockback {
    pub force: f32,
    pub duration: f32,
}

impl WeaponKnockback {
    pub fn new(force: f32, duration: f32) -> Self {
        Self { force, duration }
    }
}

pub fn apply_knockback_force(
    enemy_entity: Entity,
    enemy_velocity: &mut Velocity,
    enemy_transform: &Transform,
    source_transform: &Transform,
    weapon_knockback: &WeaponKnockback,
    commands: &mut Commands,
) {
    // Use enemy's knockback settings if available, otherwise use weapon's settings
    let (force, duration) = (weapon_knockback.force, weapon_knockback.duration);
    
    // Calculate knockback direction (from weapon to enemy)
    let direction = (enemy_transform.translation - source_transform.translation).normalize();
    let knockback_velocity = Vec2::new(direction.x, direction.y) * force;
    
    // Apply knockback velocity
    enemy_velocity.linvel += knockback_velocity;
    
    // Add knockback timer to limit duration
    commands.entity(enemy_entity).insert(KnockbackTimer {
        timer: Timer::from_seconds(duration, TimerMode::Once),
    });
    
    println!("Knockback applied! Force: {:.1}, Direction: ({:.2}, {:.2})", 
        force, direction.x, direction.y);
}