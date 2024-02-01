use bevy::prelude::*;

use crate::{LoadedParticleData, LoadedSpellData, LoadedTilesetData};

use super::{
    load_particle_effects,
    loader::{load_data_file_dir, load_tilesets, ParticleEffectStore, TileAtlasStore},
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
        app.init_resource::<TileAtlasStore>()
            .init_resource::<ParticleEffectStore>()
            .add_systems(Update, (load_tilesets, load_particle_effects));

        // Add the system to load the data
        app.add_systems(Startup, load_data_file_dir);
    }
}
