use bevy::prelude::*;

use crate::{InternalId, LoadedSpellData};

use super::storage::ExistingSpells;

pub(super) fn load_spells(
    mut events: EventReader<LoadedSpellData>,
    mut spells: ResMut<ExistingSpells>,
) {
    if events.is_empty() {
        return;
    }

    tracing::info!("Load spells event with {} spells", events.len());
    for event in events.read() {
        let mut spell = event.spell_data.data.clone();
        spell.update_internal_id();
        spells.ids.push(spell.get_internal_id());
        spells.data.push(spell);
        tracing::debug!(
            "load_spells: Loaded spell {} as {}",
            event.spell_data.data.name,
            event.spell_data.data.get_internal_id()
        );
    }
}
