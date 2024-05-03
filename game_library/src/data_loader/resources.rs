//! Resources include the directories that included data is pulled from.
use bevy::prelude::*;
use std::path::PathBuf;

use crate::save_file::SaveFileDirectories;

/// The (relative) directory where the game data files are stored
pub const DATA_FILE_DIR: &str = "game_data";

/// A 'mods' directory that can be used to store mods for the game. It is located in the game
/// directory itself (by the game executable).
pub const MODS_DIR: &str = "mods";

/// Plugins directory should be inserted into the 'settings' directory.
pub const PLUGIN_DIR_NAME: &str = "plugins";

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

    /// Validate that the directories exist. Optionally create them if they don't.
    ///
    /// If they don't exist and we aren't creating them, remove them from the list.
    ///
    /// ## Parameters
    ///
    /// - `create`: If true, create the directories if they don't exist.
    ///
    /// ## Returns
    ///
    /// The number of directories that were removed from the list.
    #[must_use]
    pub fn validate_directories(&mut self, create: bool) -> usize {
        let mut removed = 0;
        self.directories.retain(|dir| {
            if dir.is_dir() {
                true
            } else if create {
                info!("Creating directory: {}", dir.display());
                std::fs::create_dir_all(dir).is_ok()
            } else {
                removed += 1;
                false
            }
        });
        removed
    }
}

/// Resource to indicate if we should create missing directories.
#[derive(Debug, Default, Resource, Clone, Reflect)]
pub struct CreateMissingDirs(pub bool);

/// System to add any new directories from our directory resource and also validate them.
#[allow(clippy::needless_pass_by_value)]
pub(super) fn validate_data_file_dirs(
    create_missing_dirs: Option<Res<CreateMissingDirs>>,
    directory_paths: Res<SaveFileDirectories>,
    mut data_file_dirs: ResMut<DataFileDirs>,
) {
    // No reason not to create the directories if they don't exist.
    let mut create_dirs = true;
    // Always prefer the value from the resource if it exists.
    if let Some(create) = create_missing_dirs {
        create_dirs = create.0;
    }

    // Add a 'plugins' directory to the list of directories to check for data files.
    data_file_dirs.add_directory(directory_paths.settings.join(PLUGIN_DIR_NAME));

    let removed = data_file_dirs.validate_directories(create_dirs);
    if removed > 0 {
        info!(
            "Removed {} non-existent directories from the list of data file directories",
            removed
        );
    }
}
