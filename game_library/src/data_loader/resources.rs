//! Resources include the directories that included data is pulled from.
use bevy::prelude::*;
use std::path::PathBuf;

/// The (relative) directory where the game data files are stored
pub const DATA_FILE_DIR: &str = "game_data";

/// A 'mods' directory that can be used to store mods for the game.
pub const MODS_DIR: &str = "mods";

/// Resource for the directories to check for data recursively.
///
/// This resource makes adding new directories (like mods or plugins?) easier.
#[derive(Debug, Resource, Clone, Reflect)]
pub struct DataFileDirs {
    directories: Vec<PathBuf>,
}

impl Default for DataFileDirs {
    fn default() -> Self {
        Self {
            directories: vec![PathBuf::from(DATA_FILE_DIR), PathBuf::from(MODS_DIR)],
        }
    }
}

impl DataFileDirs {
    /// Add a directory to the list of directories to check for data files.
    pub fn add_directory(&mut self, directory: PathBuf) {
        self.directories.push(directory);
    }

    /// Get the list of directories to check for data files.
    #[must_use]
    pub fn directories(&self) -> &[PathBuf] {
        self.directories.as_slice()
    }
}
