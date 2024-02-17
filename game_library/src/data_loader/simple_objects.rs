//! Loads particles from the data files and stores them in the particle effect store.
use bevy::prelude::*;

use super::{events::LoadedSimpleObjectData, storage::GameData};

/// System to load a particle effect.
pub(super) fn load_simple_objects(
    mut er_realm_df: EventReader<LoadedSimpleObjectData>,
    mut game_data: ResMut<GameData>,
) {
    for data_file in er_realm_df.read() {
        let unique_id = &data_file.object_data.header.unique_id;
        let simple_obj = &data_file.object_data.data;

        // Sanity check the object is valid.
        if !simple_obj.is_tile() && !simple_obj.is_sprite() {
            error!(
                "load_simple_objects: Simple object with unique_id '{}' is missing tile or sprite data!",
                unique_id
            );
            info!("{simple_obj:?}");
            continue;
        }

        game_data
            .simple_objects
            .insert(String::from(unique_id), simple_obj.clone());
    }
}
