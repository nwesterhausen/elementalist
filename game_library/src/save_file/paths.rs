//! Provides the directory for either save or settings files.
use bevy::prelude::*;
use platform_dirs::AppDirs;
use std::path::PathBuf;

const APP_PREFIX: &str = "Elementalist";

/// Directory for settings files.
///
/// ## Panics
/// Panics if the directory cannot be found.
pub fn settings_directory() -> PathBuf {
    let Some(app_dirs) = AppDirs::new(Some(APP_PREFIX), true) else {
        tracing::error!("Unable to get settings directory for platform. Quitting.");
        std::process::exit(1);
    };
    app_dirs.config_dir
}

/// Directory for save files.
///
/// ## Panics
/// Panics if the directory cannot be found.
pub fn save_directory() -> PathBuf {
    let Some(app_dirs) = AppDirs::new(Some(APP_PREFIX), true) else {
        tracing::error!("Unable to get save directory for platform. Quitting.");
        std::process::exit(1);
    };
    app_dirs.data_dir
}

/// Resource to hold the directory paths for settings and save files.
#[derive(Debug, Clone, Resource, Component, PartialEq, Eq, Hash)]
pub struct SaveFileDirectories {
    /// Directory for settings files and plugins.
    pub settings: PathBuf,
    /// Directory for save files.
    pub save: PathBuf,
}
