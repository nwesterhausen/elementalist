//! All the various settings for the game

mod actions;
pub mod audio;
mod default_controls;
mod plugin;
pub mod structs;
pub mod video;

pub use actions::Action;
pub use audio::AudioSettingsBundle;
pub use default_controls::map_controls;
pub use plugin::SettingsPlugin;
pub use structs::*;
pub use video as VideoSettings;
