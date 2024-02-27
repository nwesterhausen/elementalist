//! Loads particles from the data files and stores them in the particle effect store.
use bevy::prelude::*;

use super::{events::LoadedEntitySpriteData, storage::GameData};

/// System to load a particle effect.
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
