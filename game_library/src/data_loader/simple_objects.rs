//! Loads particles from the data files and stores them in the particle effect store.
use bevy::prelude::*;

use crate::{enums::GameSystem, SimpleObject};

use super::{
    events::{DataFileFound, LoadedSimpleObjectData},
    loader::read_data_file,
    storage::GameData,
    DataFile,
};

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

/// System to parse a simple object data file.
pub(super) fn parse_simple_object_file(
    mut er_df_found: EventReader<DataFileFound>,
    mut ew_simple_object_df: EventWriter<LoadedSimpleObjectData>,
) {
    for event in er_df_found.read() {
        if event.header.system == GameSystem::SimpleObject {
            let object_data: DataFile<SimpleObject> =
                if let Some(d) = read_data_file(&event.filepath) {
                    d
                } else {
                    debug!(
                        "load_data_file_dir: failed to read simple object data from {}",
                        event.header.unique_id
                    );
                    continue;
                };
            ew_simple_object_df.send(LoadedSimpleObjectData { object_data });
        }
    }
}
