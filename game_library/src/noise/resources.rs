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
    /// The width of the map. Each tile is 16x16 px.
    width: usize,
    /// The height of the map. Each tile is 16x16 px.
    height: usize,
}

impl GeneratedMaps {
    /// The default size of the map.
    pub const DEFAULT_SIZE: (usize, usize) = (100, 100);

    /// Create a new Empty `GeneratedMaps` with the given size.
    #[must_use]
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
    #[must_use]
    pub fn get_biome(&self, pos: Vec2) -> GenericBiome {
        let (x, y) = self.world_to_map(Vec3::new(pos.x, pos.y, 0.0));

        self.biome_map[x][y]
    }

    /// Function to transform map coordinates to world coordinates.
    ///
    /// The world origin is in the exact center of the map. This is represented by a 0,0
    /// coordinate being at the middle index of the map.
    ///
    /// The world grid is 16x16 pixels per tile.
    ///
    /// ## Note
    ///
    /// This function has possible truncation and precision loss. This is because the
    /// world coordinates are floats and the map coordinates are usize.
    ///
    /// # Parameters
    ///
    /// * `pos`: The map position to convert to world coordinates. It sets `pos.z` to 0.0.
    ///
    /// # Returns
    ///
    /// The world position for the given map coordinates.
    #[must_use]
    pub fn map_to_world(&self, pos: (usize, usize)) -> Vec3 {
        #[allow(clippy::cast_precision_loss)]
        Vec3::new(
            (pos.0 as f32).mul_add(16.0, self.width as f32 * -8.0),
            (pos.1 as f32).mul_add(16.0, self.height as f32 * -8.0),
            0.0,
        )
    }

    /// Function to transform world coordinates to map coordinates.
    ///
    /// The world origin is in the exact center of the map. This is represented by a 0,0
    /// world coordinate being at the middle index of the map (width/2, height/2).
    ///
    /// The world grid is 16x16 pixels per tile.
    ///
    /// ## Note
    ///
    /// This function has possible truncation and precision loss. This is because the
    /// world coordinates are floats and the map coordinates are usize.
    ///
    /// # Parameters
    ///
    /// * `pos`: The world position to convert to map coordinates. `pos.z` is ignored.
    ///
    /// # Returns
    ///
    /// A tuple of the x and y coordinates of the map for the given world position.
    ///
    /// If the world position is out of bounds, then `(0,0)` is returned.
    #[must_use]
    pub fn world_to_map(&self, pos: Vec3) -> (usize, usize) {
        // Check if the position is out of bounds.
        #[allow(clippy::cast_precision_loss)]
        if pos.x < -(self.width as f32 / 2.0) || pos.x > (self.width as f32 / 2.0) {
            tracing::error!(
                "world_to_map: X position out of bounds: ({}, {})",
                pos.x,
                pos.y
            );
            return (0, 0);
        }
        #[allow(clippy::cast_precision_loss)]
        if pos.y < -(self.height as f32 / 2.0) || pos.y > (self.height as f32 / 2.0) {
            tracing::error!(
                "world_to_map: Y position out of bounds: ({}, {})",
                pos.x,
                pos.y
            );
            return (0, 0);
        }

        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss
        )]
        (
            pos.x.mul_add(1. / 16.0, self.width as f32 * -8.0) as usize,
            pos.y.mul_add(1. / 16.0, self.height as f32 * -8.0) as usize,
        )
    }

    /// Get the dimensions of the map.
    ///
    /// Returns a tuple of the width and height of the map.
    #[must_use]
    pub const fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Reset the maps to be empty.
    ///
    /// This will clear the biome and object maps and reset them to be empty. It will also
    /// add the correct amount of empty vectors to the maps (just like during a `new` call)
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
        Self::new(Self::DEFAULT_SIZE)
    }
}
