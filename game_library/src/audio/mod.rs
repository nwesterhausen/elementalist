//! Audio module for the game library. Handles audio playback and management.
//!
//! Things that this will contain:
//!
//! - Systems to respond to events (cast spell, level up, etc)
//! - Systems to respond to game state (paused, etc)
//!
//! All sounds played will query the settings to determine volume, etc.

mod player_interaction;
mod plugin;

#[allow(clippy::module_name_repetitions)]
pub use plugin::AudioPlugin;
