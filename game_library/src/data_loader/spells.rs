use bevy::prelude::*;

use super::{events::LoadedSpellData, storage::GameData};

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
        let spell = event.spell_data.data.clone().with_unique_id(&unique_id);

        game_data.spells.insert(unique_id.clone(), spell);
        tracing::debug!(
            "load_spells: loaded spell {} as {}",
            event.spell_data.data.name(),
            unique_id
        );
    }
}
