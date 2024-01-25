//! Settings Menu
//!
//! This has the various "scenes" which are part of the settings menu. The idea is for this to be
//! in the middle of the screen, and the background to be blurred out; that way it can be used in
//! any context and still be readable.
//!
//! Settings should be stored and retrieved from the [`bevy_pkv::PkvStore`] resource. Our settings
//! structs should already be serializable, and they should be usable.
//!
//! See [`game_library::settings`] for the settings resources and other things.
//!
//! This module will have to handle drawing things to the screen and sending the appropriate
//! events on setting changes.

mod base;
mod plugin;
mod state;

pub use plugin::SettingsMenuPlugin;
pub use state::MenuState;
