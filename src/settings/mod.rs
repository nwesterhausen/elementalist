//! All the various settings for the game

mod actions;
pub mod audio;
mod default_controls;
pub mod video;

pub use actions::Action;
pub use audio as AudioSettings;
pub use default_controls::map_controls;
pub use video as VideoSettings;
