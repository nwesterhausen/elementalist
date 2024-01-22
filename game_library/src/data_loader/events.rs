//! Events that are fired when data is loaded.
//!
//! For every type of data loaded, there is a separate event.
//!
//! Currently, the following events are available:
//!
//! * [`LoadedSpellData`] - Fired when spell data is loaded.

use bevy::ecs::event::Event;

use crate::SpellData;

use super::DataFile;

#[derive(Event)]
/// Event that is fired when a spell is loaded.
pub struct LoadedSpellData {
    /// The spell data that was loaded.
    pub spell_data: DataFile<SpellData>,
}
