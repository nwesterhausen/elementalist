//! Settings Menu
//!
//! This has the various "scenes" which are part of the settings menu. The idea is for this to be
//! in the middle of the screen, and the background to be blurred out; that way it can be used in
//! any context and still be readable.
//!
//! See [`game_library::settings`] for the settings resources and other things.
//!
//! This module will have to handle drawing things to the screen and sending the appropriate
//! events on setting changes.

mod accessibility;
mod audio;
mod base;
mod button_actions;
mod controls;
mod display;
mod events;
mod gameplay;
mod main;
mod plugin;

pub use plugin::SettingsMenuPlugin;
