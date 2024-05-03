//! The storage module contains the resources that store the data that has been loaded into the game.
use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_hanabi::EffectAsset;

use crate::{
    images::{EntitySprite, StoredTextureAtlas},
    realm_data::Realm,
    spells::Spell,
    SimpleObject,
};

/// The vault resource is a generic resource that holds data that is stored by a unique id.
///
/// This is useful for storing data that is loaded from files, such as spells, tile atlases, and particles.
#[derive(Resource, Default, Debug, Clone, PartialEq, Eq)]
pub struct Vault<T> {
    /// The data that has been loaded into the game.
    data: HashMap<String, T>,
}

impl<T> Vault<T> {
    /// Returns the data that has the given `unique_id`.
    ///
    /// If the data does not exist, then `None` is returned.
    #[must_use]
    pub fn get(&self, unique_id: &str) -> Option<&T> {
        self.data.get(unique_id)
    }
    /// Adds a new piece of data to the vault.
    ///
    /// If the data already exists, then it is overwritten.
    pub fn add(&mut self, unique_id: String, data: T) {
        self.data.insert(unique_id, data);
    }
    /// Insert is an alias for `add`.
    pub fn insert(&mut self, unique_id: String, data: T) {
        self.add(unique_id, data);
    }
    /// Removes the data with the given `unique_id` from the vault.
    ///
    /// If the data does not exist, then nothing happens.
    pub fn remove(&mut self, unique_id: &str) {
        self.data.remove(unique_id);
    }
    /// Returns an iterator over the unique ids of the data that have been loaded into the game.
    pub fn iter_ids(&self) -> impl Iterator<Item = &String> {
        self.data.keys()
    }
    /// Returns an iterator over the data that have been loaded into the game.
    pub fn iter_data(&self) -> impl Iterator<Item = &T> {
        self.data.values()
    }
}

/// The loaded game data resource holds all of the data that has been loaded into the game.
///
/// This includes the spells, tile atlases, and particles.
///
/// This resource helps to simplify grabbing resources in systems.
#[derive(Resource, Default, Debug, Clone)]
pub struct GameData {
    /// The spells that have been loaded into the game.
    pub spells: Vault<Spell>,
    /// The tile atlases that have been loaded into the game.
    pub tile_atlas: Vault<StoredTextureAtlas>,
    /// The particles that have been loaded into the game.
    pub particles: Vault<Handle<EffectAsset>>,
    /// Loaded realm definitions
    pub realms: Vault<Realm>,
    /// Loaded simple objects
    pub simple_objects: Vault<SimpleObject>,
    /// Loaded entity sprites
    pub entity_sprites: Vault<EntitySprite>,
}
