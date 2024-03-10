use bevy::prelude::*;
use serde_yaml;
use std::{hash::Hash, path::Path};
use walkdir::WalkDir;

use crate::InternalId;

use super::{
    events::DataFileFound,
    header_def::{DataFile, DataFileHeader},
    DataFileDirs, DataFileHeaderOnly,
};

/// Read in an ingestible file and return the header information from it.
/// This will return None if the file is un-readable or ill-formatted.
/// What this does return on success is the `FileHeader`, mainly the unique ID for the file,
/// the version info, the system it has data for, author, and description.
pub(super) fn read_file_header(path: &str) -> Option<DataFileHeader> {
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
pub(super) fn read_data_file<T: serde::de::DeserializeOwned + Hash + InternalId, P: AsRef<Path>>(
    path: &P,
) -> Option<DataFile<T>> {
    let path = path.as_ref();

    // Attempt to open the file passed in
    let f = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("read_file_data: failed to open {}, {}", path.display(), e);
            return None;
        }
    };
    // Attempt to parse the yaml file into a header: Header object
    let ingest: DataFile<T> = match serde_yaml::from_reader(f) {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("read_file_data: failed to parse {}, {}", path.display(), e);
            return None;
        }
    };
    Some(ingest)
}

/// Check for valid data files in the data file directories and fire an event for ones that are valid.
#[allow(clippy::needless_pass_by_value)]
pub(super) fn read_data_file_dirs(
    directories: Res<DataFileDirs>,
    mut ew_df_found: EventWriter<DataFileFound>,
) {
    for d in directories.directories() {
        let possible_ingests: Vec<String> = WalkDir::new(d)
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
        for d in possible_ingests {
            let filepath = d.as_str();
            let h = read_file_header(filepath);
            if let Some(header) = h {
                tracing::debug!(
                    "read_data_file_dirs: read {:?} header of {} from {}",
                    header.system,
                    header.unique_id,
                    &filepath
                );
                ew_df_found.send(DataFileFound {
                    header,
                    filepath: filepath.into(),
                });
            }
        }
    }
}
