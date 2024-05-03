//! Defines the `BiomeData` resource.
use bevy::prelude::*;
use bevy::reflect::Reflect;
use rand::{seq::SliceRandom, SeedableRng};
use std::hash::Hash;

use crate::{
    enums::biome::{Altitude, Biome, Humidity, Latitude},
    noise::OBJECT_POOL,
};

/// The biome system is a list of 1 - 10 "biomes" that are then used to determine the actual
/// biome of the world. This is then used to determine the type of terrain and the type of
/// objects that are placed in the world. This is then used to determine the actual biome
/// of the world.
#[derive(Debug, Clone, Resource, Reflect, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub struct BiomeData {
    /// The actual biome of the world.
    pub biome: Biome,
    /// The altitude (and temperature) of the biome.
    pub altitude: Altitude,
    /// The humidity of the biome.
    pub humidity: Humidity,
    /// The latitudinal band of the biome.
    pub latitude: Latitude,
    /// Details about the ground tileset for the biome
    pub ground_tilesets: Vec<TilesetDetail>,
    /// Details about the various "single-tile" objects that can be placed in the biome.
    pub simple_objects: Vec<SimpleObjectDetail>,
}

impl BiomeData {
    /// Creates a new barren biome
    #[must_use]
    pub const fn barren() -> Self {
        Self {
            biome: Biome::Barren,
            altitude: Altitude::Montane,
            humidity: Humidity::Arid,
            latitude: Latitude::WarmTemperate,
            ground_tilesets: Vec::new(),
            simple_objects: Vec::new(),
        }
    }
    /// Return a random tile from the ground tilesets.
    pub fn random_ground_tile(&self) -> Option<(&str, usize)> {
        let mut rng = rand::thread_rng();
        if self.ground_tilesets.is_empty() {
            tracing::error!("random_ground_tile: no ground tilesets!");
            return None;
        }

        let Some(tileset) = self.ground_tilesets.choose(&mut rng) else {
            tracing::error!("random_ground_tile: unable to get random tilset!");
            return None;
        };

        let Ok(tile) = tileset
            .weights
            .choose_weighted(&mut rng, |item| item.weight)
        else {
            tracing::error!("random_ground_tile: unable to get random tile!");
            return None;
        };
        Some((tileset.id.as_str(), tile.tile))
    }

    /// Flat map of objects against the `OBJECT_POOL` for the biome.
    #[must_use]
    pub fn object_pool(&self, seed: u64) -> Vec<Option<&str>> {
        let mut pool = Vec::with_capacity(OBJECT_POOL);

        for object in &self.simple_objects {
            for _ in 0..object.weight {
                pool.push(Some(object.id.as_str()));
            }
        }

        while pool.len() < OBJECT_POOL {
            pool.push(None);
        }

        // Shuffle the pool so that the objects are placed randomly. Seeded.
        let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
        pool.shuffle(&mut rng);

        pool
    }
}

/// Details about the tileset for the realm.
#[derive(Debug, Clone, Resource, Reflect, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TilesetDetail {
    /// The unique identifier for the tileset.
    pub id: String,
    /// Weight details for the individual tiles in the tileset.
    pub weights: Vec<TilesetWeight>,
}

/// The weight details for the individual tiles in the tileset.
#[derive(Debug, Clone, Resource, Reflect, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TilesetWeight {
    /// The tile index
    pub tile: usize,
    /// The weight of the tile
    pub weight: f32,
}

impl Hash for BiomeData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.biome.hash(state);
        self.altitude.hash(state);
        self.humidity.hash(state);
        self.latitude.hash(state);
    }
}

/// Details about the various "simple" objects that can be placed in the biome.
#[derive(Debug, Clone, Resource, Reflect, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleObjectDetail {
    /// The unique identifier for the object.
    pub id: String,
    /// The "objective" weight for the object. This is some value that is weighted against
    /// a total of [`crate::noise::OBJECT_POOL`] to be spawned in the world.
    pub weight: usize,
}
