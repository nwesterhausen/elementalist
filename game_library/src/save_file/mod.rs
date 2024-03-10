//! Module for handing save files, saving and loading.
//!
//! Directory for save files is provided from the `platform_dirs` crate.
//!
//! Need to track the following:
//!
//! - created talismans
//! - researched talismans
//! - player skills
//! - unlocked skill perks for each skill
//! - inventory (essences, items, etc.)
//! - name of the save
//! - stat tracking
//!   - time played
//!   - enemies killed (and by what)
//!   - damage dealt, with each spell
//!   - damage taken
//!   - etc...

mod data;
pub mod events;
mod initialize;
mod paths;
mod plugin;
mod save_load;

#[allow(clippy::module_name_repetitions)]
pub use data::SaveFile;
pub use paths::settings_directory;
#[allow(clippy::module_name_repetitions)]
pub use paths::SaveFileDirectories;
#[allow(clippy::module_name_repetitions)]
pub use plugin::SaveFilePlugin;
