use bevy::prelude::*;
use game_library::LoadedSpellData;

use super::{
    components::{despawn_expired_spells, CastSpell},
    firebolt,
};

/// Spells are fired using the `CastSpell` event.
///
/// The individual spells are implemented in their own modules,
/// and will launch from the player's position when the `CastSpell`
/// event is fired.
pub struct SpellsPlugin;

impl Plugin for SpellsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CastSpell>()
            .add_event::<LoadedSpellData>()
            // Load spell data (from events)
            .add_systems(Update, load_spells)
            // Spell systems
            .add_systems(Update, despawn_expired_spells)
            // Individual Spells
            .add_systems(Update, firebolt::spawn_firebolt);
    }
}

fn load_spells(mut events: EventReader<LoadedSpellData>) {
    if events.is_empty() {
        return;
    }

    tracing::info!("Load spells event with {} spells", events.len());
    for event in events.read() {
        tracing::info!("load_spells: Loaded spell {}", event.spell_data.data.name);
    }
}
