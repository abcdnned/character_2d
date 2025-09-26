use bevy::prelude::*;

use crate::level::tiled::{TiledMapBundle, TiledMapHandle};


pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Loading tiled map from: map/level.tmx");
    let map_handle = TiledMapHandle(asset_server.load("map/level.tmx"));

    commands.spawn(TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}