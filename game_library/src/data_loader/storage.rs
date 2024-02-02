//! The storage module contains the resources that store the data that has been loaded into the game.
use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_hanabi::EffectAsset;

use crate::SpellData;

/// The tile atlas store is a resource that holds all the tilesets that have been loaded into the game.
///
/// They are stored by their `unique_id`, which is supplied in the header.
#[derive(Resource, Default, Debug, Clone, PartialEq, Eq)]
pub struct TileAtlasStore {
    /// The tilesets that have been loaded into the game.
    pub tilesets: HashMap<String, Handle<TextureAtlas>>,
}

/// The existing spells resource holds all of the spells that have been loaded into the game.
///
/// They are stored by their `unique_id`, which is supplied in the header.
#[derive(Resource, Default, Debug, Clone)]
pub struct ExistingSpells {
    /// The data of the spells that have been loaded into the game.\
    pub data: Vec<SpellData>,
    /// The unique ids of the spells that have been loaded into the game.
    pub ids: Vec<String>,
}

/// The particle asset store is a resource that holds all the particles that have been loaded into the game.
///
/// They are stored by their `unique_id`, which is supplied in the header.
#[derive(Resource, Default, Debug, Clone, PartialEq, Eq)]
pub struct ParticleEffectStore {
    /// The particles that have been loaded into the game.
    pub particles: HashMap<String, Handle<EffectAsset>>,
}
