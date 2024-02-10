//! Loads particles from the data files and stores them in the particle effect store.
use bevy::prelude::*;

use super::{events::LoadedRealmData, storage::GameData};

/// System to load a particle effect.
pub(super) fn load_realms(
    mut er_realm_df: EventReader<LoadedRealmData>,
    mut game_data: ResMut<GameData>,
) {
    for data_file in er_realm_df.read() {
        let unique_id = &data_file.realm_data.header.unique_id;
        let realm = &data_file.realm_data.data;

        game_data
            .realms
            .insert(String::from(unique_id), realm.clone());
    }
}
