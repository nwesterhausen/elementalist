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
    /// The biome map.
    pub biome_map: Vec<Vec<GenericBiome>>,
    /// The object map.
    pub object_map: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl GeneratedMaps {
    /// The default size of the map.
    pub const DEFAULT_SIZE: (usize, usize) = (100, 100);

    /// Create a new Empty GeneratedMaps with the given size.
    pub fn new(size: (usize, usize)) -> Self {
        let mut empty_map = Self {
            biome_map: Vec::with_capacity(size.0),
            object_map: Vec::with_capacity(size.0),
            width: size.0,
            height: size.1,
        };
        for _ in 0..size.0 {
            empty_map.biome_map.push(Vec::with_capacity(size.1));
            empty_map.object_map.push(Vec::with_capacity(size.1));
        }

        empty_map
    }

    /// Ask for the biome at the given position.
    ///
    /// Each tile is a 16x16 px but is accessed via the x and y coordinates of the tile.
    ///
    /// The world origin is in the exact center of the map. This is represented by a 0,0
    /// coordinate being at the middle index of the map.
    pub fn get_biome(&self, pos: Vec2) -> Option<GenericBiome> {
        // Check if the position is out of bounds.
        if pos.x < -(self.width as f32 / 2.0) || pos.x > (self.width as f32 / 2.0) {
            tracing::warn!(
                "get_biome: X position out of bounds: ({}, {})",
                pos.x,
                pos.y
            );
            return None;
        }
        if pos.y < -(self.height as f32 / 2.0) || pos.y > (self.height as f32 / 2.0) {
            tracing::warn!(
                "get_biome: Y position out of bounds: ({}, {})",
                pos.x,
                pos.y
            );
            return None;
        }

        // Convert the position to the map's coordinates.
        let x = (pos.x + (self.width as f32 / 2.0)) as usize;
        let y = (pos.y + (self.height as f32 / 2.0)) as usize;

        // Sanity check before returning the biome.
        if x < self.biome_map.len() && y < self.biome_map[0].len() {
            Some(self.biome_map[x][y])
        } else {
            tracing::warn!("get_biome: Position out of bounds: ({}, {})", x, y);
            None
        }
    }

    /// Function to transform map coordinates to world coordinates.
    ///
    /// The world origin is in the exact center of the map. This is represented by a 0,0
    /// coordinate being at the middle index of the map.
    ///
    /// The world grid is 16x16 pixels per tile.
    pub fn map_to_world(&self, pos: Vec3) -> Vec3 {
        Vec3::new(
            pos.x * 16.0 - (self.width as f32 * 8.0),
            pos.y * 16.0 - (self.height as f32 * 8.0),
            pos.z,
        )
    }

    /// Get the dimensions of the map.
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Reset the maps to be empty.
    pub fn reset(&mut self) {
        self.biome_map.clear();
        self.object_map.clear();

        for _ in 0..self.width {
            self.biome_map.push(Vec::with_capacity(self.height));
            self.object_map.push(Vec::with_capacity(self.height));
        }
    }
}

impl Default for GeneratedMaps {
    fn default() -> Self {
        Self::new(GeneratedMaps::DEFAULT_SIZE)
    }
}
