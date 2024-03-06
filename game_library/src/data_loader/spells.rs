use bevy::prelude::*;

use crate::{enums::GameSystem, spells::Spell};

use super::{
    events::{DataFileFound, LoadedSpellData},
    loader::read_data_file,
    storage::GameData,
    DataFile,
};

/// System to load spell data into the spells vault for usage in the game.
///
/// This responds to the `LoadedSpellData` event and loads the spell data into the `GameData` resource.
pub(super) fn load_spells(
    mut events: EventReader<LoadedSpellData>,
    mut game_data: ResMut<GameData>,
) {
    if events.is_empty() {
        return;
    }

    debug!("Load spells event with {} spells", events.len());
    for event in events.read() {
        let unique_id = &event.spell_data.header.unique_id;
        let spell = event.spell_data.data.clone().with_unique_id(unique_id);

        game_data.spells.insert(unique_id.clone(), spell);
        info!("Loaded {}", event.spell_data.data);
    }
}

/// System to parse the spell data files and load them into the game.
///
/// This responds to the `DataFileFound` event and reads the spell data from the file, and upon success, sends a `LoadedSpellData` event.
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
                    "parse_spell_file: failed to read spell data from {}; {}",
                    event.header.unique_id,
                    event.filepath.display()
                );
                continue;
            };
            ew_spell_df.send(LoadedSpellData { spell_data });
        }
    }
}
