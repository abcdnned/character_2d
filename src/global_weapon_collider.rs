use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct WeaponColliderMap {
    /// Maps Player entity to their weapon collider entity
    player_to_collider: HashMap<Entity, Entity>,
}

impl WeaponColliderMap {
    /// Create a new empty weapon collider map
    pub fn new() -> Self {
        Self {
            player_to_collider: HashMap::new(),
        }
    }

    /// Insert a mapping from player entity to weapon collider entity
    pub fn insert(&mut self, player_entity: Entity, collider_entity: Entity) {
        self.player_to_collider
            .insert(player_entity, collider_entity);
    }

    /// Get the weapon collider entity for a given player entity
    pub fn get(&self, player_entity: Entity) -> Option<Entity> {
        self.player_to_collider.get(&player_entity).copied()
    }

    /// Remove a mapping for a player entity
    pub fn remove(&mut self, player_entity: Entity) -> Option<Entity> {
        self.player_to_collider.remove(&player_entity)
    }

    /// Check if a player has a weapon collider mapped
    pub fn contains(&self, player_entity: Entity) -> bool {
        self.player_to_collider.contains_key(&player_entity)
    }

    /// Get all player entities that have weapon colliders
    pub fn players(&self) -> impl Iterator<Item = &Entity> {
        self.player_to_collider.keys()
    }

    /// Get all weapon collider entities
    pub fn colliders(&self) -> impl Iterator<Item = &Entity> {
        self.player_to_collider.values()
    }

    /// Get iterator over all (player, collider) pairs
    pub fn iter(&self) -> impl Iterator<Item = (&Entity, &Entity)> {
        self.player_to_collider.iter()
    }

    /// Clear all mappings
    pub fn clear(&mut self) {
        self.player_to_collider.clear();
    }

    /// Get the number of mapped players
    pub fn len(&self) -> usize {
        self.player_to_collider.len()
    }

    /// Check if the map is empty
    pub fn is_empty(&self) -> bool {
        self.player_to_collider.is_empty()
    }
}

// Plugin to initialize the resource
pub struct WeaponColliderMapPlugin;

impl Plugin for WeaponColliderMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeaponColliderMap>();
    }
}
