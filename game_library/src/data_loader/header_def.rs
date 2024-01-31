use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::{enums::GameSystem, InternalId};

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
pub struct DataFile<T>
where
    T: Hash + InternalId,
{
    /// The header information for this data file.
    pub header: DataFileHeader,
    /// The data in this data file.
    pub data: T,
}
