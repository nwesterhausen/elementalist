use bevy::prelude::*;

use crate::{enums::GameSystem, spells::Spell};

use super::{
    events::{DataFileFound, LoadedSpellData},
    loader::read_data_file,
    storage::GameData,
    DataFile,
};

pub(super) fn load_spells(
    mut events: EventReader<LoadedSpellData>,
    mut game_data: ResMut<GameData>,
) {
    if events.is_empty() {
        return;
    }

    tracing::info!("Load spells event with {} spells", events.len());
    for event in events.read() {
        let unique_id = &event.spell_data.header.unique_id;
        let spell = event.spell_data.data.clone().with_unique_id(unique_id);

        game_data.spells.insert(unique_id.clone(), spell);
        tracing::debug!(
            "load_spells: loaded spell {} as {}",
            event.spell_data.data.name(),
            unique_id
        );
    }
}

pub(super) fn parse_spell_file(
    mut er_df_found: EventReader<DataFileFound>,
    mut ew_spell_df: EventWriter<LoadedSpellData>,
) {
    for event in er_df_found.read() {
        if event.header.system == GameSystem::Spell {
            let spell_data: DataFile<Spell> = if let Some(d) = read_data_file(&event.filepath) {
                d
            } else {
                warn!(
                    "load_data_file_dir: failed to read spell data from {}",
                    event.header.unique_id
                );
                continue;
            };
            ew_spell_df.send(LoadedSpellData { spell_data });
        }
    }
}
