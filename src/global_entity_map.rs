use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct GlobalEntityMap {
    /// Maps Player entity to their weapon collider entity
    pub player_to_collider: HashMap<Entity, Entity>,
    pub player_sword_trail: HashMap<Entity, Entity>,
    pub weapon_collider: HashMap<Entity, Entity>,
    pub player_weapon: HashMap<Entity, Entity>,
    pub entity_transfrom: HashMap<Entity, Transform>,
    pub weapon_player: HashMap<Entity, Entity>,
}

// Plugin to initialize the resource
pub struct GlobalEntityMapPlugin;

impl Plugin for GlobalEntityMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalEntityMap>()
            .add_systems(Update, auto_register_transform);
    }
}

/// System to automatically register all entity transforms in the global map
pub fn auto_register_transform(
    mut global_map: ResMut<GlobalEntityMap>,
    query: Query<(Entity, &Transform)>,
) {
    for (entity, transform) in query.iter() {
        global_map.entity_transfrom.insert(entity, *transform);
    }
}
