//! Plugin for the save file system.

use bevy::prelude::*;

use super::{data::SaveFile, events::SaveFileEvent, initialize, save_load};

/// Plugin for the save file system.
#[allow(clippy::module_name_repetitions)]
pub struct SaveFilePlugin;

impl Plugin for SaveFilePlugin {
    fn build(&self, app: &mut App) {
        // Initialize the save file system
        app.add_systems(PreStartup, initialize::save_file_directories);
        // Add the save file resource
        app.init_resource::<SaveFile>();
        // Add the events for loading and saving save files
        app.add_event::<SaveFileEvent>();
        // Add the system to save and load save files
        app.add_systems(Update, save_load::handle_save_file_event);
    }
}
