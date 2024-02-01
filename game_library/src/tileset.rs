//! Tileset data. Instructions for what tilesets to load.
use std::{any::Any, hash::Hash};

use crate::{data_loader::DataFile, enums::GameSystem, InternalId};

mod tileset_defaults {
    pub(super) const fn tile_dimension() -> f32 {
        32.0
    }
    pub(super) const fn tileset_dimension() -> usize {
        5
    }
}

/// Details about a tileset.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
            tile_width: 32.0,
            tile_height: 32.0,
            tileset_width: 5,
            tileset_height: 5,
        }
    }
}
