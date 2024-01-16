use bevy::ecs::event::EventWriter;
use serde_yaml;
use std::hash::Hash;
use walkdir::WalkDir;

use crate::{data_loader::DATA_FILE_DIR, GameSystem, SpellData};

use super::{
    events::LoadedSpellData,
    header_def::{DataFile, DataFileHeader},
    DataFileHeaderOnly,
};

/// Read in an ingestible file and return the header information from it.
/// This will return None if the file is un-readable or ill-formatted.
/// What this does return on success is the FileHeader, mainly the unique ID for the file,
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
pub fn read_data_file<T: serde::de::DeserializeOwned + Hash>(path: &str) -> Option<DataFile<T>> {
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
pub fn load_data_file_dir(mut ew_df: EventWriter<LoadedSpellData>) {
    let start = std::time::Instant::now();

    let mut possible_ingests: Vec<String> = WalkDir::new(DATA_FILE_DIR)
        .into_iter()
        .filter_map(|file| file.ok())
        .filter(|entry| {
            entry.file_type().is_file() && entry.path().extension().is_some_and(|ext| ext == "yaml")
        })
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();

    let mut spells_read: usize = 0;

    for d in possible_ingests.iter_mut() {
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
                    let spell_data: DataFile<SpellData> = match read_data_file(filepath) {
                        Some(f) => f,
                        None => {
                            tracing::debug!(
                                "load_data_file_dir: failed to read spell data from {}",
                                header.unique_id
                            );
                            continue;
                        }
                    };
                    ew_df.send(LoadedSpellData { spell_data });
                    spells_read += 1;
                }
                _ => {
                    tracing::warn!(
                        "load_data_file_dir: no system match for {:?}",
                        header.system
                    );
                }
            }
        }
    }
    let duration = start.elapsed();
    tracing::info!("loaded {} spells in {:?}", spells_read, duration);
}