use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct GlobalEntityMap {
    /// Maps Player entity to their weapon collider entity
    pub player_to_collider: HashMap<Entity, Entity>,
    pub player_sword_trail: HashMap<Entity, Entity>,
    pub weapon_collider: HashMap<Entity, Entity>,
    pub player_weapon: HashMap<Entity, Entity>,
}

// Plugin to initialize the resource
pub struct GlobalEntityMapPlugin;

impl Plugin for GlobalEntityMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalEntityMap>();
    }
}
