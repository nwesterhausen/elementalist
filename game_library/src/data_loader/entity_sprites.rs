//! Loads sprites from the data files and stores them in the entity sprite store.
use bevy::prelude::*;

use crate::{enums::GameSystem, images::EntitySprite};

use super::{
    events::{DataFileFound, LoadedEntitySpriteData},
    loader::read_data_file,
    storage::GameData,
    DataFile,
};

/// System to load an entity sprite.
pub(super) fn load_entity_sprites(
    mut er_realm_df: EventReader<LoadedEntitySpriteData>,
    mut game_data: ResMut<GameData>,
) {
    for data_file in er_realm_df.read() {
        let unique_id = &data_file.entity_sprite_data.header.unique_id;
        let entity_sprite = &data_file.entity_sprite_data.data;

        game_data
            .entity_sprites
            .insert(String::from(unique_id), entity_sprite.clone());
    }
}

/// System to parse an entity sprite data file.
pub(super) fn parse_entity_sprite_file(
    mut er_df_found: EventReader<DataFileFound>,
    mut ew_entity_sprite_df: EventWriter<LoadedEntitySpriteData>,
) {
    for event in er_df_found.read() {
        if event.header.system == GameSystem::EntitySprite {
            let entity_sprite_data: DataFile<EntitySprite> =
                if let Some(d) = read_data_file(&event.filepath) {
                    d
                } else {
                    warn!(
                        "load_data_file_dir: failed to read entity sprite data from {}",
                        event.header.unique_id
                    );
                    continue;
                };
            ew_entity_sprite_df.send(LoadedEntitySpriteData { entity_sprite_data });
        }
    }
}
