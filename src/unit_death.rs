use bevy::prelude::*;
use crate::unit::HpChangeEvent;

pub struct UnitDeathPlugin;

impl Plugin for UnitDeathPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_unit_death);
    }
}

fn handle_unit_death(
    mut commands: Commands,
    mut hp_events: EventReader<HpChangeEvent>,
) {
    for event in hp_events.read() {
        debug!("Received HpChangeEvent for entity: {:?}, new_hp: {}", event.entity, event.new_hp);
        
        // Check if the unit's HP has reached zero or below
        if event.new_hp <= 0.0 {
            info!("Unit {:?} has died (HP: {}), despawning entity", event.entity, event.new_hp);
            
            // Despawn the entity
            commands.entity(event.entity).despawn_recursive();
        }
    }
}