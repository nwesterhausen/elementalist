//! The storage module contains the resources that store the data that has been loaded into the game.
use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_hanabi::EffectAsset;

use crate::SpellData;

/// The tile atlas store is a resource that holds all the tilesets that have been loaded into the game.
///
/// They are stored by their `unique_id`, which is supplied in the header.
#[derive(Resource, Default, Debug, Clone, PartialEq, Eq, Reflect)]
pub struct TileAtlasStore {
    /// The tilesets that have been loaded into the game.
    ///
    /// The key is the unique id of the tileset.
    tilesets: HashMap<String, Handle<TextureAtlas>>,
}

impl TileAtlasStore {
    /// Returns the handle to the texture atlas that has the given `unique_id`.
    ///
    /// If the texture atlas does not exist, then `None` is returned.
    #[must_use]
    pub fn get(&self, unique_id: &str) -> Option<Handle<TextureAtlas>> {
        self.tilesets.get(unique_id).cloned()
    }
    /// Adds a new texture atlas to the store.
    ///
    /// If the texture atlas already exists, then it is overwritten.
    pub fn add(&mut self, unique_id: String, texture_atlas: Handle<TextureAtlas>) {
        self.tilesets.insert(unique_id, texture_atlas);
    }
    /// Insert is an alias for `add`.
    pub fn insert(&mut self, unique_id: String, texture_atlas: Handle<TextureAtlas>) {
        self.add(unique_id, texture_atlas);
    }
    /// Removes the texture atlas with the given `unique_id` from the store.
    ///
    /// If the texture atlas does not exist, then nothing happens.
    pub fn remove(&mut self, unique_id: &str) {
        self.tilesets.remove(unique_id);
    }
    /// Returns an iterator over the unique ids of the tilesets that have been loaded into the game.
    #[must_use]
    pub fn iter_ids(&self) -> impl Iterator<Item = &String> {
        self.tilesets.keys()
    }

    /// Returns an iterator over the texture atlases of the tilesets that have been loaded into the game.
    #[must_use]
    pub fn iter_tilesets(&self) -> impl Iterator<Item = &Handle<TextureAtlas>> {
        self.tilesets.values()
    }
}

/// The existing spells resource holds all of the spells that have been loaded into the game.
///
/// They are stored by their `unique_id`, which is supplied in the header.
#[derive(Resource, Default, Debug, Clone, Reflect)]
pub struct SpellVault {
    /// The data of the spells that have been loaded into the game.
    ///
    /// The key is the unique id of the spell.
    spells: HashMap<String, SpellData>,
}

impl SpellVault {
    /// Returns the spell data that has the given `unique_id`.
    ///
    /// If the spell does not exist, then `None` is returned.
    #[must_use]
    pub fn get(&self, unique_id: &str) -> Option<&SpellData> {
        self.spells.get(unique_id)
    }
    /// Adds a new spell to the vault.
    ///
    /// If the spell already exists, then it is overwritten.
    pub fn add(&mut self, unique_id: String, spell: SpellData) {
        self.spells.insert(unique_id, spell);
    }
    /// Insert is an alias for `add`.
    pub fn insert(&mut self, unique_id: String, spell: SpellData) {
        self.add(unique_id, spell);
    }
    /// Removes the spell with the given `unique_id` from the vault.
    ///
    /// If the spell does not exist, then nothing happens.
    pub fn remove(&mut self, unique_id: &str) {
        self.spells.remove(unique_id);
    }
    /// Returns an iterator over the unique ids of the spells that have been loaded into the game.
    #[must_use]
    pub fn iter_ids(&self) -> impl Iterator<Item = &String> {
        self.spells.keys()
    }
    /// Returns an iterator over the spell data of the spells that have been loaded into the game.
    #[must_use]
    pub fn iter_spells(&self) -> impl Iterator<Item = &SpellData> {
        self.spells.values()
    }
}

/// The particle asset store is a resource that holds all the particles that have been loaded into the game.
///
/// They are stored by their `unique_id`, which is supplied in the header.
#[derive(Resource, Default, Debug, Clone, PartialEq, Eq, Reflect)]
pub struct ParticleEffectStore {
    /// The particles that have been loaded into the game.
    ///
    /// The key is the unique id of the particle.
    particles: HashMap<String, Handle<EffectAsset>>,
}

impl ParticleEffectStore {
    /// Returns the handle to the particle that has the given `unique_id`.
    ///
    /// If the particle does not exist, then `None` is returned.
    #[must_use]
    pub fn get(&self, unique_id: &str) -> Option<Handle<EffectAsset>> {
        self.particles.get(unique_id).cloned()
    }
    /// Adds a new particle to the store.
    ///
    /// If the particle already exists, then it is overwritten.
    pub fn add(&mut self, unique_id: String, particle: Handle<EffectAsset>) {
        self.particles.insert(unique_id, particle);
    }
    /// Insert is an alias for `add`.
    pub fn insert(&mut self, unique_id: String, particle: Handle<EffectAsset>) {
        self.add(unique_id, particle);
    }
    /// Removes the particle with the given `unique_id` from the store.
    ///
    /// If the particle does not exist, then nothing happens.
    pub fn remove(&mut self, unique_id: &str) {
        self.particles.remove(unique_id);
    }
    /// Returns an iterator over the unique ids of the particles that have been loaded into the game.
    #[must_use]
    pub fn iter_ids(&self) -> impl Iterator<Item = &String> {
        self.particles.keys()
    }

    /// Returns an iterator over the particles of the particles that have been loaded into the game.
    #[must_use]
    pub fn iter_particles(&self) -> impl Iterator<Item = &Handle<EffectAsset>> {
        self.particles.values()
    }
}

/// The loaded game data resource holds all of the data that has been loaded into the game.
///
/// This includes the spells, tile atlases, and particles.
///
/// This resource helps to simplify grabbing resources in systems.
#[derive(Resource, Default, Debug, Clone, Reflect)]
pub struct GameData {
    /// The spells that have been loaded into the game.
    pub spells: SpellVault,
    /// The tile atlases that have been loaded into the game.
    pub tile_atlas: TileAtlasStore,
    /// The particles that have been loaded into the game.
    pub particles: ParticleEffectStore,
}
