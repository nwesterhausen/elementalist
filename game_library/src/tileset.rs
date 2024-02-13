//! Tileset data. Instructions for what tilesets to load.
use bevy::math::Vec2;
use bevy::reflect::Reflect;
use std::{any::Any, hash::Hash};

use crate::{data_loader::DataFile, enums::GameSystem, InternalId};

/// Default tile dimensions (32x32 pixels)
pub const DEFAULT_TILE_DIMENSION: f32 = 32.0;
/// Default tileset dimensions (5x5 tiles)
pub const DEFAULT_TILESET_DIMENSION: usize = 5;
/// Default tileset padding (0x0 pixels)
pub const DEFAULT_TILESET_PADDING: f32 = 1.0;
/// Default tileset offset (0x0 pixels)
pub const DEFAULT_TILESET_OFFSET: f32 = 1.0;

mod tileset_defaults {
    use super::{
        DEFAULT_TILESET_DIMENSION, DEFAULT_TILESET_OFFSET, DEFAULT_TILESET_PADDING,
        DEFAULT_TILE_DIMENSION,
    };

    pub(super) const fn tile_dimension() -> f32 {
        DEFAULT_TILE_DIMENSION
    }
    pub(super) const fn tileset_dimension() -> usize {
        DEFAULT_TILESET_DIMENSION
    }
    pub(super) const fn tileset_padding() -> f32 {
        DEFAULT_TILESET_PADDING
    }
    pub(super) const fn tileset_offset() -> f32 {
        DEFAULT_TILESET_OFFSET
    }
}

/// Details about a tileset.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct Tileset {
    /// The internal ID of the tileset.
    pub internal_id: Option<String>,
    /// The path to the tileset (relative to the game's asset directory).
    pub path: String,
    /// The width of the tiles in the tileset.
    #[serde(default = "tileset_defaults::tile_dimension")]
    pub tile_width: f32,
    /// The height of the tiles in the tileset.
    #[serde(default = "tileset_defaults::tile_dimension")]
    pub tile_height: f32,
    /// The number of tiles in the tileset.
    #[serde(default = "tileset_defaults::tileset_dimension")]
    pub tileset_width: usize,
    /// The number of tiles in the tileset.
    #[serde(default = "tileset_defaults::tileset_dimension")]
    pub tileset_height: usize,
    /// The horizontal padding between tiles.
    #[serde(default = "tileset_defaults::tileset_padding")]
    pub horizontal_padding: f32,
    /// The vertical padding between tiles.
    #[serde(default = "tileset_defaults::tileset_padding")]
    pub vertical_padding: f32,
    /// The horizontal offset for the tileset.
    #[serde(default = "tileset_defaults::tileset_offset")]
    pub horizontal_offset: f32,
    /// The vertical offset for the tileset.
    #[serde(default = "tileset_defaults::tileset_offset")]
    pub vertical_offset: f32,
}

impl Hash for Tileset {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.tileset_width.hash(state);
        self.tileset_height.hash(state);
    }
}

impl InternalId for Tileset {
    /// Update the spell's internal ID.
    fn update_internal_id(&mut self) {
        self.internal_id = Some(self.get_internal_id());
    }
    /// Get the spell's internal ID.
    #[must_use]
    fn get_internal_id(&self) -> String {
        if self.internal_id.is_some() {
            let id = self.internal_id.clone().unwrap_or_default();
            if !id.is_empty() {
                return id;
            }
        }

        format!(
            "{}{}{}",
            self.path.replace(' ', ""),
            self.tileset_width,
            self.tileset_height
        )
    }
}

impl<D: Hash + InternalId + 'static> TryInto<Tileset> for DataFile<D> {
    type Error = ();

    fn try_into(self) -> Result<Tileset, Self::Error> {
        if self.header.system != GameSystem::Tileset {
            return Err(());
        }

        (&self.data as &dyn Any)
            .downcast_ref::<Tileset>()
            .cloned()
            .ok_or(())
    }
}

impl<D: Hash + InternalId + 'static> TryFrom<&DataFile<D>> for Tileset {
    type Error = ();

    fn try_from(data_file: &DataFile<D>) -> Result<Self, Self::Error> {
        if data_file.header.system != GameSystem::Tileset {
            return Err(());
        }

        (&data_file.data as &dyn Any)
            .downcast_ref::<Self>()
            .cloned()
            .ok_or(())
    }
}

impl Default for Tileset {
    fn default() -> Self {
        Self {
            internal_id: None,
            path: "path/to/tileset.png".to_string(),
            tile_width: DEFAULT_TILE_DIMENSION,
            tile_height: DEFAULT_TILE_DIMENSION,
            tileset_width: DEFAULT_TILESET_DIMENSION,
            tileset_height: DEFAULT_TILESET_DIMENSION,
            horizontal_padding: DEFAULT_TILESET_PADDING,
            vertical_padding: DEFAULT_TILESET_PADDING,
            horizontal_offset: DEFAULT_TILESET_OFFSET,
            vertical_offset: DEFAULT_TILESET_OFFSET,
        }
    }
}

impl Tileset {
    /// Get the Vec2 padding for the tileset.
    ///
    /// This will return None if both paddings are 0.0.
    #[must_use]
    pub fn get_padding(&self) -> Option<Vec2> {
        if self.horizontal_padding == 0.0 && self.vertical_padding == 0.0 {
            return None;
        }

        Some(Vec2::new(self.horizontal_padding, self.vertical_padding))
    }

    /// Get the Vec2 offset for the tileset.
    ///
    /// This will return None if both offsets are 0.0.
    #[must_use]
    pub fn get_offset(&self) -> Option<Vec2> {
        if self.horizontal_offset == 0.0 && self.vertical_offset == 0.0 {
            return None;
        }

        Some(Vec2::new(self.horizontal_offset, self.vertical_offset))
    }
}
