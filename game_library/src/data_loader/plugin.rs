use bevy::prelude::*;

use super::{
    events::{LoadedParticleData, LoadedSpellData, LoadedTilesetData},
    load_data_file_dir,
    particles::load_particle_effects,
    spells::load_spells,
    storage::GameData,
    tilesets::load_tilesets,
};

/// The plugin for the data loader.
///
/// This takes care of adding the required events and the system to load the data.
#[allow(clippy::module_name_repetitions)]
pub struct DataLoaderPlugin;

impl Plugin for DataLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadedSpellData>()
            .add_event::<LoadedTilesetData>()
            .add_event::<LoadedParticleData>();

        // Set up the resources used and the systems to store the data
        app.init_resource::<GameData>()
            .add_systems(Update, (load_tilesets, load_particle_effects, load_spells));

        // Add the system to load the data
        app.add_systems(Startup, load_data_file_dir);
    }
}
