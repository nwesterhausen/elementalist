//! Resources include the directories that included data is pulled from.
use bevy::prelude::*;
use std::path::PathBuf;

/// The (relative) directory where the game data files are stored
pub const DATA_FILE_DIR: &str = "game_data";

/// Resource for the directories to check for data recursively.
///
/// This resource makes adding new directories (like mods or plugins?) easier.
#[derive(Debug, Resource, Reflect)]
pub struct DataFileDirs {
    directories: Vec<PathBuf>,
}

impl Default for DataFileDirs {
    fn default() -> Self {
        Self {
            directories: vec![PathBuf::from(DATA_FILE_DIR)],
        }
    }
}
