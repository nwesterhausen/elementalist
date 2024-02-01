use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_hanabi::prelude::*;
use serde_yaml;
use std::hash::Hash;
use walkdir::WalkDir;

use crate::{
    data_loader::DATA_FILE_DIR, enums::GameSystem, particle::Particle, InternalId,
    LoadedParticleData, LoadedTilesetData, SpellData, Tileset,
};

use super::{
    events::LoadedSpellData,
    header_def::{DataFile, DataFileHeader},
    DataFileHeaderOnly,
};

/// Read in an ingestible file and return the header information from it.
/// This will return None if the file is un-readable or ill-formatted.
/// What this does return on success is the `FileHeader`, mainly the unique ID for the file,
/// the version info, the system it has data for, author, and description.
pub fn read_file_header(path: &str) -> Option<DataFileHeader> {
    // Attempt to open the file passed in
    let f = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("read_file_header: failed to open {}, {}", path, e);
            return None;
        }
    };
    // Attempt to parse the yaml file into a header: Header object
    let scraped: DataFileHeaderOnly = match serde_yaml::from_reader(f) {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("read_file_header: failed to parse {}, {}", path, e);
            return None;
        }
    };
    Some(scraped.header)
}

/// Read in an ingestible file and return it. This is generic because all ingestible files
/// are the same, they only differ in the struct that is returned.
pub fn read_data_file<T: serde::de::DeserializeOwned + Hash + InternalId>(
    path: &str,
) -> Option<DataFile<T>> {
    // Attempt to open the file passed in
    let f = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("read_file_data: failed to open {}, {}", path, e);
            return None;
        }
    };
    // Attempt to parse the yaml file into a header: Header object
    let ingest: DataFile<T> = match serde_yaml::from_reader(f) {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("read_file_data: failed to parse {}, {}", path, e);
            return None;
        }
    };
    Some(ingest)
}

/// Reading in the directory of ingestible files:
/// 1. Should be recursive to get all subdirs
/// 2. Should organize the files before reading them all in
///     i.   Read headers for all files
///     ii.  Discard any that are for wrong game version
///     iii. Store [filename, header] in a list for each system
///     iv.  Sort the ingest data lists using any specified ordinal constraints
/// 3. Add the data to the database in system load order (TDB)
/// 4. Validate skill -> class, magic -> skill,class, and other relationships are valid
///
pub fn load_data_file_dir(
    mut ew_spell_df: EventWriter<LoadedSpellData>,
    mut ew_tileset_df: EventWriter<LoadedTilesetData>,
    mut ew_particle_df: EventWriter<LoadedParticleData>,
) {
    // let start = std::time::Instant::now();

    let mut possible_ingests: Vec<String> = WalkDir::new(DATA_FILE_DIR)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|entry| {
            entry.file_type().is_file()
                && entry
                    .path()
                    .extension()
                    .is_some_and(|ext| ext == "yaml" || ext == "yml")
        })
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();

    let mut spells_read: usize = 0;
    let mut tilesets_read: usize = 0;
    let mut particles_read: usize = 0;

    for d in &mut possible_ingests {
        let filepath = d.as_str();
        let h = read_file_header(filepath);
        if let Some(header) = h {
            tracing::trace!(
                "load_data_file_dir: read header of {} from {}",
                header.unique_id,
                &filepath
            );
            match header.system {
                GameSystem::Spell => {
                    let spell_data: DataFile<SpellData> = if let Some(d) = read_data_file(filepath)
                    {
                        d
                    } else {
                        tracing::debug!(
                            "load_data_file_dir: failed to read spell data from {}",
                            header.unique_id
                        );
                        continue;
                    };
                    ew_spell_df.send(LoadedSpellData { spell_data });
                    spells_read += 1;
                }
                GameSystem::Tileset => {
                    let tileset_data: DataFile<Tileset> = if let Some(d) = read_data_file(filepath)
                    {
                        d
                    } else {
                        tracing::debug!(
                            "load_data_file_dir: failed to read tileset data from {}",
                            header.unique_id
                        );
                        continue;
                    };
                    ew_tileset_df.send(LoadedTilesetData { tileset_data });
                    tilesets_read += 1;
                }
                GameSystem::Particle => {
                    let particle_data: DataFile<Particle> =
                        if let Some(d) = read_data_file(filepath) {
                            d
                        } else {
                            tracing::debug!(
                                "load_data_file_dir: failed to read particle data from {}",
                                header.unique_id
                            );
                            continue;
                        };
                    ew_particle_df.send(LoadedParticleData { particle_data });
                    particles_read += 1;
                }
            }
        }
    }
    // let duration = start.elapsed();
    tracing::info!(
        "loaded {} spells, {} tilesets, {} particles",
        spells_read,
        tilesets_read,
        particles_read
    );
}

/// The tile atlas store is a resource that holds all the tilesets that have been loaded into the game.
///
/// They are stored by their `unique_id`, which is supplied in the header.
#[derive(Resource, Default, Debug, Clone, PartialEq, Eq)]
pub struct TileAtlasStore {
    /// The tilesets that have been loaded into the game.
    pub tilesets: HashMap<String, Handle<TextureAtlas>>,
}
/// The particle asset store is a resource that holds all the particles that have been loaded into the game.
///
/// They are stored by their `unique_id`, which is supplied in the header.
#[derive(Resource, Default, Debug, Clone, PartialEq, Eq)]
pub struct ParticleEffectStore {
    /// The particles that have been loaded into the game.
    pub particles: HashMap<String, Handle<EffectAsset>>,
}

/// Load the tilesets into the game and store a handle under the `unique_id`.
#[allow(clippy::needless_pass_by_value)]
pub fn load_tilesets(
    mut er_tileset_df: EventReader<LoadedTilesetData>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut tile_atlas_store: ResMut<TileAtlasStore>,
    asset_server: Res<AssetServer>,
) {
    for tileset in er_tileset_df.read() {
        let unique_id = &tileset.tileset_data.header.unique_id;
        let tileset = &tileset.tileset_data.data;

        let texture_handle = asset_server.load(&tileset.path);
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(tileset.tile_width, tileset.tile_height),
            tileset.tileset_width,
            tileset.tileset_height,
            None,
            None,
        );

        let atlas_handle = texture_atlases.add(texture_atlas);

        tile_atlas_store
            .tilesets
            .insert(String::from(unique_id), atlas_handle);
    }
}

/// System to load a particle effect.
pub fn load_particle_effects(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut er_particle_df: EventReader<LoadedParticleData>,
    mut particle_effect_store: ResMut<ParticleEffectStore>,
) {
    for data_file in er_particle_df.read() {
        let unique_id = &data_file.particle_data.header.unique_id;
        let particle = &data_file.particle_data.data;

        let mut writer = ExprWriter::new();

        let age = writer.lit(0.).expr();
        let init_age = SetAttributeModifier::new(Attribute::AGE, age);

        let lifetime = writer.lit(particle.particle_lifetime).expr();
        let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let init_pos = SetPositionCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(),
            radius: writer.lit(particle.initial_position.radius).expr(),
            dimension: particle
                .initial_position
                .shape_dimension
                .as_shape_dimension(),
        };

        let init_vel = SetVelocityCircleModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            axis: writer.lit(Vec3::Z).expr(),
            speed: writer.lit(particle.initial_velocity.speed).expr(),
        };

        let spawner = Spawner::rate(particle.spawn_rate.into());
        let effect = effects.add(
            EffectAsset::new(particle.capacity, spawner, writer.finish())
                .with_name(unique_id)
                .init(init_pos)
                .init(init_vel)
                .init(init_age)
                .init(init_lifetime)
                .render(SizeOverLifetimeModifier {
                    gradient: Gradient::constant(Vec2::splat(0.52)),
                    screen_space_size: false,
                })
                .render(ColorOverLifetimeModifier {
                    gradient: Gradient::constant(Vec4::splat(0.5)),
                }),
        );

        particle_effect_store
            .particles
            .insert(String::from(unique_id), effect);
    }
}
