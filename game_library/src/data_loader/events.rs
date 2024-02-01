//! Events that are fired when data is loaded.
//!
//! For every type of data loaded, there is a separate event.
//!
//! Currently, the following events are available:
//!
//! * [`LoadedSpellData`] - Fired when spell data is loaded.

use bevy::ecs::event::Event;

use crate::{particle::Particle, SpellData, Tileset};

use super::DataFile;

#[derive(Event)]
/// Event that is fired when a spell is loaded.
pub struct LoadedSpellData {
    /// The spell data that was loaded.
    pub spell_data: DataFile<SpellData>,
}

#[derive(Event)]
/// Event that is fired when a tileset is loaded.
pub struct LoadedTilesetData {
    /// The tileset data that was loaded.
    pub tileset_data: DataFile<Tileset>,
}

#[derive(Event)]
/// Event that is fired when a particle is loaded.
pub struct LoadedParticleData {
    /// The particle data that was loaded.
    pub particle_data: DataFile<Particle>,
}
