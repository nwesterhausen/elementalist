//! Plugin for the save file system.

use bevy::prelude::*;

/// Plugin for the save file system.
#[allow(clippy::module_name_repetitions)]
pub struct SaveFilePlugin;

impl Plugin for SaveFilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, super::initialize::save_file_system);
    }
}
