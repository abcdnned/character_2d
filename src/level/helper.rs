use bevy::prelude::*;
pub fn tiled_to_world_position(tiled_position: Vec2, tiled_map: &tiled::Map) -> Vec2 {
    return Vec2::new(
        tiled_position.x,
        (tiled_map.height * tiled_map.tile_height) as f32 - tiled_position.y,
    );
}