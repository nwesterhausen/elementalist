//! Loads particles from the data files and stores them in the particle effect store.
use bevy::prelude::*;

use crate::{enums::GameSystem, Realm};

use super::{
    events::{DataFileFound, LoadedRealmData},
    loader::read_data_file,
    storage::GameData,
    DataFile,
};

/// System to load a realm.
pub(super) fn load_realms(
    mut er_realm_df: EventReader<LoadedRealmData>,
    mut game_data: ResMut<GameData>,
) {
    for data_file in er_realm_df.read() {
        let unique_id = &data_file.realm_data.header.unique_id;
        let realm = &data_file.realm_data.data;
        info!("load_realms: loaded realm {} as {}", realm.name, unique_id);
        game_data
            .realms
            .insert(String::from(unique_id), realm.clone());
    }
}

/// System to parse a realm data file.
pub(super) fn parse_realm_file(
    mut er_df_found: EventReader<DataFileFound>,
    mut ew_realm_df: EventWriter<LoadedRealmData>,
) {
    for event in er_df_found.read() {
        if event.header.system == GameSystem::Realm {
            let realm_data: DataFile<Realm> = if let Some(d) = read_data_file(&event.filepath) {
                d
            } else {
                warn!(
                    "load_data_file_dir: failed to read realm data from {}",
                    event.header.unique_id
                );
                continue;
            };
            ew_realm_df.send(LoadedRealmData { realm_data });
        }
    }
}
