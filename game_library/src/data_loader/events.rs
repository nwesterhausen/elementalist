use bevy::ecs::event::Event;

use crate::SpellData;

use super::DataFile;

#[derive(Event)]
/// Event that is fired when a spell is loaded.
pub struct LoadedSpellData {
    /// The spell data that was loaded.
    pub spell_data: DataFile<SpellData>,
}
