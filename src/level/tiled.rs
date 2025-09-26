use std::io::{Cursor, ErrorKind};
use std::path::Path;
use std::sync::Arc;

use bevy::ecs::resource::Resource;
use bevy::ecs::system::{ResMut, SystemId};
use bevy::log::{info, error};
use bevy::{
    asset::{AssetLoader, io::Reader},
    platform::collections::HashMap,
    prelude::{
        Asset, AssetApp, Assets, Bundle, Commands, Component, Entity, GlobalTransform, Handle,
        Image, Plugin, Query, Res, Transform, Update,
    },
    reflect::TypePath,
};
use bevy_ecs_tilemap::prelude::*;
use tiled::ObjectData;

#[derive(Default)]
pub struct TiledMapPlugin;

impl Plugin for TiledMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<TiledMap>()
            .init_resource::<LevelData>()
            .init_resource::<ObjectLayers>()
            .register_asset_loader(TiledLoader)
            .add_systems(Update, process_loaded_maps);
    }
}
#[derive(Resource, Default)]
pub struct LevelData {
    pub map: Option<tiled::Map>,
}

#[derive(TypePath, Asset)]
pub struct TiledMap {
    pub map: tiled::Map,
    pub tilemap_textures: HashMap<usize, TilemapTexture>,
}

#[derive(Resource, Default)]
pub struct ObjectLayers {
    pub layer_data: HashMap<String, Vec<ObjectData>>,
    pub loader_systems: HashMap<String, SystemId>,
}

#[derive(Component, Default)]
pub struct TiledLayersStorage {
    pub storage: HashMap<u32, Entity>,
}

#[derive(Component, Default)]
pub struct TiledMapHandle(pub Handle<TiledMap>);

#[derive(Component, Default)]
pub struct TiledMapLoadState {
    pub load_flag: bool,
}
#[derive(Default, Bundle)]
pub struct TiledMapBundle {
    pub tiled_map: TiledMapHandle,
    pub storage: TiledLayersStorage,
    pub load_state: TiledMapLoadState,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub render_settings: TilemapRenderSettings,
}

struct BytesResourceReader {
    bytes: Arc<[u8]>,
}

impl BytesResourceReader {
    fn new(bytes: &[u8]) -> Self {
        Self {
            bytes: Arc::from(bytes),
        }
    }
}

impl tiled::ResourceReader for BytesResourceReader {
    type Resource = Cursor<Arc<[u8]>>;
    type Error = std::io::Error;

    fn read_from(&mut self, _path: &Path) -> std::result::Result<Self::Resource, Self::Error> {
        Ok(Cursor::new(self.bytes.clone()))
    }
}

pub struct TiledLoader;

#[derive(Debug)]
pub enum TiledAssetLoaderError {
    Io(std::io::Error),
}

impl From<std::io::Error> for TiledAssetLoaderError {
    fn from(err: std::io::Error) -> Self {
        TiledAssetLoaderError::Io(err)
    }
}

impl std::fmt::Display for TiledAssetLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TiledAssetLoaderError::Io(err) => write!(f, "Could not load Tiled file: {}", err),
        }
    }
}

impl std::error::Error for TiledAssetLoaderError {}

impl AssetLoader for TiledLoader {
    type Asset = TiledMap;
    type Settings = ();
    type Error = TiledAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let mut loader = tiled::Loader::with_cache_and_reader(
            tiled::DefaultResourceCache::new(),
            BytesResourceReader::new(&bytes),
        );
        let map = loader.load_tmx_map(load_context.path()).map_err(|e| {
            error!("Could not load TMX map: {}", e);
            std::io::Error::new(ErrorKind::Other, format!("Could not load TMX map: {e}"))
        })?;

        let mut tilemap_textures = HashMap::default();

        for (tileset_index, tileset) in map.tilesets().iter().enumerate() {
            let tilemap_texture = match &tileset.image {
                None => {
                    info!("Skipping image collection tileset '{}'", tileset.name);
                    continue;
                }
                Some(img) => {
                    let texture_path = img.source.to_str().unwrap();
                    let texture: Handle<Image> = load_context.load(texture_path);

                    TilemapTexture::Single(texture.clone())
                }
            };

            tilemap_textures.insert(tileset_index, tilemap_texture);
        }

        let asset_map = TiledMap {
            map,
            tilemap_textures,
        };

        info!("Loaded map: {}", load_context.path().display());
        Ok(asset_map)
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["tmx"];
        EXTENSIONS
    }
}

pub fn process_loaded_maps(
    mut commands: Commands,
    maps: Res<Assets<TiledMap>>,
    tile_storage_query: Query<(Entity, &TileStorage)>,
    mut map_query: Query<(
        &TiledMapHandle,
        &mut TiledMapLoadState,
        &mut TiledLayersStorage,
        &TilemapRenderSettings,
    )>,
    mut level_data: ResMut<LevelData>,
    mut object_layers: ResMut<ObjectLayers>,
) {
    if let Ok((map_handle, mut load_state, mut layer_storage, render_settings)) =
        map_query.single_mut()
    {
        if load_state.load_flag {
            return;
        }
        if let Some(tiled_map) = maps.get(&map_handle.0) {
            level_data.map = Some(tiled_map.map.clone());
            load_state.load_flag = true;

            for layer_entity in layer_storage.storage.values() {
                if let Ok((_, layer_tile_storage)) = tile_storage_query.get(*layer_entity) {
                    for tile in layer_tile_storage.iter().flatten() {
                        commands.entity(*tile).despawn()
                    }
                }
                commands.entity(*layer_entity).despawn();
            }
            
            for (tileset_index, tileset) in tiled_map.map.tilesets().iter().enumerate() {
                let tilemap_texture = tiled_map.tilemap_textures.get(&tileset_index).unwrap();

                let tile_size = TilemapTileSize {
                    x: tileset.tile_width as f32,
                    y: tileset.tile_height as f32,
                };

                let tile_spacing = TilemapSpacing {
                    x: tileset.spacing as f32,
                    y: tileset.spacing as f32,
                };

                for (layer_index, layer) in tiled_map.map.layers().enumerate() {
                    let offset_x = layer.offset_x;
                    let offset_y = layer.offset_y;

                    match layer.layer_type() {
                        tiled::LayerType::Tiles(tile_layer) => {
                            
                            if let tiled::TileLayer::Finite(layer_data) = tile_layer {
                                let map_size = TilemapSize {
                                    x: tiled_map.map.width,
                                    y: tiled_map.map.height,
                                };

                                let grid_size = TilemapGridSize {
                                    x: tiled_map.map.tile_width as f32,
                                    y: tiled_map.map.tile_height as f32,
                                };

                                let map_type = match tiled_map.map.orientation {
                                    tiled::Orientation::Hexagonal => {
                                        TilemapType::Hexagon(HexCoordSystem::Row)
                                    }
                                    tiled::Orientation::Isometric => {
                                        TilemapType::Isometric(IsoCoordSystem::Diamond)
                                    }
                                    tiled::Orientation::Staggered => {
                                        TilemapType::Isometric(IsoCoordSystem::Staggered)
                                    }
                                    tiled::Orientation::Orthogonal => TilemapType::Square,
                                };

                                let mut tile_storage = TileStorage::empty(map_size);
                                let layer_entity = commands.spawn_empty().id();

                                for x in 0..map_size.x {
                                    for y in 0..map_size.y {
                                        let mapped_y = (tiled_map.map.height - 1 - y) as i32;

                                        let mapped_x = x as i32;


                                        let layer_tile =
                                            match layer_data.get_tile(mapped_x, mapped_y) {
                                                Some(t) => t,
                                                None => {
                                                    continue;
                                                }
                                            };
                                        if tileset_index != layer_tile.tileset_index() {
                                            continue;
                                        }
                                        let layer_tile_data =
                                            match layer_data.get_tile_data(mapped_x, mapped_y) {
                                                Some(d) => d,
                                                None => {
                                                    continue;
                                                }
                                            };
                                        
                                        let texture_index = match tilemap_texture {
                                            TilemapTexture::Single(_) => layer_tile.id(),
                                        };

                                        let tile_pos = TilePos { x, y };
                                        let tile_entity = commands
                                            .spawn(TileBundle {
                                                position: tile_pos,
                                                tilemap_id: TilemapId(layer_entity),
                                                texture_index: TileTextureIndex(texture_index),
                                                flip: TileFlip {
                                                    x: layer_tile_data.flip_h,
                                                    y: layer_tile_data.flip_v,
                                                    d: layer_tile_data.flip_d,
                                                },
                                                ..Default::default()
                                            })
                                            .id();
                                        tile_storage.set(&tile_pos, tile_entity);
                                    }
                                }

                                commands.entity(layer_entity).insert(TilemapBundle {
                                    grid_size,
                                    size: map_size,
                                    storage: tile_storage,
                                    texture: tilemap_texture.clone(),
                                    tile_size,
                                    spacing: tile_spacing,
                                    anchor: TilemapAnchor::BottomLeft,
                                    transform: Transform::from_xyz(
                                        offset_x,
                                        -offset_y,
                                        layer_index as f32,
                                    ),
                                    map_type,
                                    render_settings: *render_settings,
                                    ..Default::default()
                                });

                                layer_storage
                                    .storage
                                    .insert(layer_index as u32, layer_entity);
                            }
                        }
                        tiled::LayerType::Objects(object_layer) => {
                            let data: Vec<ObjectData> = object_layer.object_data().iter().cloned().collect();
                            object_layers.layer_data.insert(layer.name.clone(), data);
                            info!("Loaded object layer '{}' with {} objects", layer.name, object_layer.object_data().len());

                            // Run system if one is registered for this layer
                            if let Some(system) = object_layers.loader_systems.get(&layer.name) {
                                commands.run_system(*system);
                            }
                        }
                        _ => {
                            info!("Unsupported Layer {}", layer.id());
                        }
                    }
                }
            }
        }
    }
}