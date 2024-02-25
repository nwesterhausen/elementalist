//! This module is responsible for loading the tilesets into the game and storing them in the `TileAtlasStore` resource.

use bevy::prelude::*;

use super::{
    events::LoadedTilesetData,
    storage::{GameData, StoredTextureAtlas},
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
