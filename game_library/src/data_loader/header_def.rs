use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::{enums::GameSystem, spells::Spell, InternalId, Tileset};

/// Each data file includes header information about the data in the file.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataFileHeader {
    /// A unique identifier for this data file.
    pub unique_id: String,
    /// The game system does this data file describe/alter/define
    pub system: GameSystem,
    /// If this data file must be before another one, list the data file which must follow this one here, by unique_id
    pub must_precede: Option<String>,
    /// If this data file must be after another one, list the data file which must precede this one here, by unique_id
    pub must_follow: Option<String>,
    /// Author of this data file
    pub author: String,
    /// Short description of the contents of this data file
    pub description: String,
    /// An internal version number, to be able to upgrade automatically
    pub internal_version: u32,
    /// A version string which can be shown to the user
    pub display_version: String,
    /// The target version of the game this is for, to be able to upgrade/replace automatically
    pub valid_game_internal_version: u32,
}

/// A generic data file which only loads the header.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataFileHeaderOnly {
    /// The header information for this data file.
    pub header: DataFileHeader,
}

/// A generic data file which can be loaded into the game.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataFile<D>
where
    D: Hash + InternalId,
{
    /// The header information for this data file.
    pub header: DataFileHeader,
    /// The data in this data file.
    pub data: D,
}

impl<D> InternalId for DataFile<D>
where
    D: Hash + InternalId,
{
    /// Update the data file's internal ID.
    fn update_internal_id(&mut self) {
        self.data.update_internal_id();
    }
    /// Get the data file's internal ID.
    #[must_use]
    fn get_internal_id(&self) -> String {
        self.data.get_internal_id()
    }
}

impl<D: Hash + InternalId + 'static> DataFile<D> {
    /// Get the data file as a tileset. Otherwise, return None.
    #[must_use]
    pub fn as_tileset(&self) -> Option<Tileset> {
        if self.header.system != GameSystem::Tileset {
            return None;
        }

        self.try_into().ok()
    }
    /// Get the data file as a spell. Otherwise, return None.
    #[must_use]
    pub fn as_spell(&self) -> Option<Spell> {
        if self.header.system != GameSystem::Spell {
            return None;
        }

        self.try_into().ok()
    }
}

impl Default for DataFileHeader {
    fn default() -> Self {
        Self {
            unique_id: String::new(),
            system: GameSystem::Spell,
            must_precede: None,
            must_follow: None,
            author: String::new(),
            description: String::new(),
            internal_version: 0,
            display_version: String::new(),
            valid_game_internal_version: 1,
        }
    }
}
