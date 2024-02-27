//! Events that are fired when data is loaded.
//!
//! For every type of data loaded, there is a separate event.
//!
//! Currently, the following events are available:
//!
//! * [`LoadedSpellData`] - Fired when spell data is loaded.

use bevy::ecs::event::Event;

use crate::{
    images::EntitySprite, particle::Particle, realm_data::Realm, simple_object::SimpleObject,
    spells::Spell, Tileset,
};

use super::DataFile;

#[derive(Event)]
/// Event that is fired when a spell is loaded.
pub struct LoadedSpellData {
    /// The spell data that was loaded.
    pub spell_data: DataFile<Spell>,
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

#[derive(Event)]
/// Event that is fired when a realm is loaded.
pub struct LoadedRealmData {
    /// The realm data that was loaded.
    pub realm_data: DataFile<Realm>,
}

#[derive(Event)]
/// Event that is fired when a realm is loaded.
pub struct LoadedSimpleObjectData {
    /// The realm data that was loaded.
    pub object_data: DataFile<SimpleObject>,
}

#[derive(Event)]
/// Event that is fired when an entity sprite is loaded.
pub struct LoadedEntitySpriteData {
    /// The realm data that was loaded.
    pub entity_sprite_data: DataFile<EntitySprite>,
}
