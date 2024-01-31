use bevy::prelude::*;

use crate::{LoadedSpellData, LoadedTilesetData};

use super::load_data_file_dir;

/// The plugin for the data loader.
///
/// This takes care of adding the required events and the system to load the data.
#[allow(clippy::module_name_repetitions)]
pub struct DataLoaderPlugin;

impl Plugin for DataLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadedSpellData>()
            .add_event::<LoadedTilesetData>();

        // Add the system to load the data
        app.add_systems(Startup, load_data_file_dir);
    }
}
