//! This module is responsible for loading the tilesets into the game and storing them in the `TileAtlasStore` resource.

use bevy::prelude::*;

use crate::{enums::GameSystem, images::StoredTextureAtlas, Tileset};

use super::{
    events::{DataFileFound, LoadedTilesetData},
    loader::read_data_file,
    storage::GameData,
    DataFile,
};

/// Load the tilesets into the game and store a handle under the `unique_id`.
#[allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]
pub fn load_tilesets(
    mut er_tileset_df: EventReader<LoadedTilesetData>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut game_data: ResMut<GameData>,
    asset_server: Res<AssetServer>,
) {
    for tileset in er_tileset_df.read() {
        let unique_id = &tileset.tileset_data.header.unique_id;
        let tileset = &tileset.tileset_data.data;

        let texture_handle = asset_server.load(&tileset.path);
        let texture_atlas = TextureAtlasLayout::from_grid(
            Vec2::new(tileset.tile_width, tileset.tile_height),
            tileset.tileset_width,
            tileset.tileset_height,
            tileset.get_padding(),
            tileset.get_offset(),
        );

        let atlas_handle = texture_atlases.add(texture_atlas);

        game_data.tile_atlas.insert(
            String::from(unique_id),
            StoredTextureAtlas {
                atlas_handle,
                texture_handle,
            },
        );
    }
}

/// System to parse a tileset data file.
pub(super) fn parse_tileset_file(
    mut er_df_found: EventReader<DataFileFound>,
    mut ew_tileset_df: EventWriter<LoadedTilesetData>,
) {
    for event in er_df_found.read() {
        if event.header.system == GameSystem::Tileset {
            let tileset_data: DataFile<Tileset> = if let Some(d) = read_data_file(&event.filepath) {
                d
            } else {
                warn!(
                    "load_data_file_dir: failed to read tileset data from {}",
                    event.header.unique_id
                );
                continue;
            };
            ew_tileset_df.send(LoadedTilesetData { tileset_data });
        }
    }
}
