//! Events that are fired when data is loaded.
//!
//! For every type of data loaded, there is a separate event.
//!
//! Currently, the following events are available:
//!
//! * [`LoadedSpellData`] - Fired when spell data is loaded.

use std::path::PathBuf;

use bevy::ecs::event::Event;

use crate::{
    images::EntitySprite, particle::Particle, realm_data::Realm, simple_object::SimpleObject,
    spells::Spell, Tileset,
};

use super::{DataFile, DataFileHeader};

/// Event that is fired when a valid data file is found.
#[derive(Event)]
pub struct DataFileFound {
    /// The header of the data file
    pub header: DataFileHeader,
    /// The path to the data file
    pub filepath: PathBuf,
}

/// Event that is fired when a spell is loaded.
#[derive(Event)]
pub struct LoadedSpellData {
    /// The spell data that was loaded.
    pub spell_data: DataFile<Spell>,
}

/// Event that is fired when a tileset is loaded.
#[derive(Event)]
pub struct LoadedTilesetData {
    /// The tileset data that was loaded.
    pub tileset_data: DataFile<Tileset>,
}

/// Event that is fired when a particle is loaded.
#[derive(Event)]
pub struct LoadedParticleData {
    /// The particle data that was loaded.
    pub particle_data: DataFile<Particle>,
}

/// Event that is fired when a realm is loaded.
#[derive(Event)]
pub struct LoadedRealmData {
    /// The realm data that was loaded.
    pub realm_data: DataFile<Realm>,
}

/// Event that is fired when a realm is loaded.
#[derive(Event)]
pub struct LoadedSimpleObjectData {
    /// The realm data that was loaded.
    pub object_data: DataFile<SimpleObject>,
}

/// Event that is fired when an entity sprite is loaded.
#[derive(Event)]
pub struct LoadedEntitySpriteData {
    /// The realm data that was loaded.
    pub entity_sprite_data: DataFile<EntitySprite>,
}
