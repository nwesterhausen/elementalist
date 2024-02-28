//! System to initialize the save file system.

use super::paths::{save_directory, settings_directory, SaveFileDirectories};
use bevy::prelude::*;

/// System to initialize the save file system.
pub fn save_file_system(mut commands: Commands) {
    let save_directory = save_directory();
    if !save_directory.exists() {
        std::fs::create_dir_all(&save_directory).expect("Unable to create save directory.");
    }
    let settings_directory = settings_directory();
    if !settings_directory.exists() {
        std::fs::create_dir_all(&settings_directory).expect("Unable to create settings directory.");
    }

    commands.insert_resource(SaveFileDirectories {
        settings: settings_directory,
        save: save_directory,
    });
}
