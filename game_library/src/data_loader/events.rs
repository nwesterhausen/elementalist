use bevy::ecs::event::Event;

use crate::SpellData;

use super::DataFile;

#[derive(Event)]
pub struct LoadedSpellData {
    pub spell_data: DataFile<SpellData>,
}
