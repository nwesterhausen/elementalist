//! Data about objects that can appear on the map.
use bevy::prelude::*;

use crate::InternalId;

/// The details about a simple (one sprite or tile) object that can be placed on the map.
#[derive(Debug, Hash, Clone, Resource, Reflect, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleObject {
    /// The internal ID of the object
    pub internal_id: Option<String>,
    /// The path to the sprite to load for the object (relative to the game's asset directory).
    ///
    /// If this is loaded from a tileset, this should be left empty.
    pub sprite_path: Option<String>,
    /// The tileset to load the sprite from. This is the unique identifier for the tileset.
    ///
    /// If this is loaded from a sprite, this should be left empty.
    pub tileset: Option<String>,
    /// The index of the tile in the tileset to use for the sprite.
    ///
    /// If this is loaded from a sprite, this should be left empty.
    pub tile_index: Option<usize>,
}

impl SimpleObject {
    /// Create a new simple object.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            internal_id: None,
            sprite_path: None,
            tileset: None,
            tile_index: None,
        }
    }

    /// Returns true if the object is loaded from a sprite.
    #[must_use]
    pub const fn is_sprite(&self) -> bool {
        self.sprite_path.is_some()
    }

    /// Returns true if the object is loaded from a tileset.
    #[must_use]
    pub const fn is_tile(&self) -> bool {
        self.tileset.is_some() && self.tile_index.is_some()
    }
}

impl InternalId for SimpleObject {
    /// Update the object's internal ID.
    fn update_internal_id(&mut self) {
        self.internal_id = Some(self.get_internal_id());
    }
    /// Get the object's internal ID.
    #[must_use]
    fn get_internal_id(&self) -> String {
        if self.internal_id.is_some() {
            let id = self.internal_id.clone().unwrap_or_default();
            if !id.is_empty() {
                return id;
            }
        }

        if self.is_sprite() {
            return format!(
                "smpObjSprite{}",
                self.sprite_path
                    .clone()
                    .unwrap_or_else(|| "no-sprite-path".to_string())
            );
        }
        if self.is_tile() {
            return format!(
                "smpObjTile{}{}",
                self.tileset
                    .clone()
                    .unwrap_or_else(|| "no-tileset".to_string()),
                self.tile_index.unwrap_or(0)
            );
        }
        tracing::error!("simple object without any valid sprite data");
        "smpObjUnknown-Unknown".to_string()
    }
}
