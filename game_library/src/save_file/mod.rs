//! Module for handing save files, saving and loading.
//!
//! Directory for save files is provided from the `platform_dirs` crate.

mod initialize;
mod paths;
mod plugin;

pub use paths::settings_directory;
#[allow(clippy::module_name_repetitions)]
pub use plugin::SaveFilePlugin;
