use bevy::prelude::*;

use super::{
    entity_sprites::{load_entity_sprites, parse_entity_sprite_file},
    events::{
        DataFileFound, LoadedEntitySpriteData, LoadedParticleData, LoadedRealmData,
        LoadedSimpleObjectData, LoadedSpellData, LoadedTilesetData,
    },
    loader::read_data_file_dirs,
    particles::{load_particle_effects, parse_particle_file},
    realms::{load_realms, parse_realm_file},
    simple_objects::{load_simple_objects, parse_simple_object_file},
    spells::{load_spells, parse_spell_file},
    storage::GameData,
    tilesets::{load_tilesets, parse_tileset_file},
    DataFileDirs,
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
            .add_event::<LoadedParticleData>()
            .add_event::<LoadedRealmData>()
            .add_event::<LoadedSimpleObjectData>()
            .add_event::<LoadedEntitySpriteData>()
            .add_event::<DataFileFound>();

        // Set up the resources used and the systems to store the data
        app.init_resource::<DataFileDirs>()
            .init_resource::<GameData>()
            .add_systems(
                Update,
                (
                    parse_tileset_file,
                    parse_entity_sprite_file,
                    parse_simple_object_file,
                    parse_particle_file,
                    parse_realm_file,
                    parse_spell_file,
                    load_tilesets,
                    load_entity_sprites,
                    load_simple_objects,
                    load_particle_effects,
                    load_realms,
                    load_spells,
                ),
            );

        // Add the system to load the data
        app.add_systems(Startup, (read_data_file_dirs,));
    }
}
