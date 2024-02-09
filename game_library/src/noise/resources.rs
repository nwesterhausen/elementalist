use bevy::prelude::*;

use crate::enums::GenericBiome;

/// A resource that stores the seed for the generation of the primal realm.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Resource)]
pub struct GenerationSeed(pub u32);

/// Stores in the generated biome map and generated object map.
///
/// This is used to store the results of the noise generation, and then
/// used to generate the actual realm. The default map size is 1000x1000.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Resource)]
pub struct GeneratedMaps {
    pub biome_map: Vec<Vec<GenericBiome>>,
    pub object_map: Vec<Vec<usize>>,
}

impl GeneratedMaps {
    pub const DEFAULT_SIZE: (usize, usize) = (1000, 1000);

    /// Create a new Empty GeneratedMaps with the given size.
    pub fn new(size: (usize, usize)) -> Self {
        let mut empty_map = Self {
            biome_map: Vec::with_capacity(size.0),
            object_map: Vec::with_capacity(size.0),
        };
        for _ in 0..size.0 {
            empty_map.biome_map.push(Vec::with_capacity(size.1));
            empty_map.object_map.push(Vec::with_capacity(size.1));
        }

        empty_map
    }
}

impl Default for GeneratedMaps {
    fn default() -> Self {
        Self::new(GeneratedMaps::DEFAULT_SIZE)
    }
}
