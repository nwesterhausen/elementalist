//! Data loading module
//!
//! This module contains all of the data loading code for the game. This includes
//! loading the game data files and sending events with the loaded data.

pub mod events;
pub mod storage;

mod entity_sprites;
mod header_def;
mod loader;
mod particles;
mod plugin;
mod realms;
mod simple_objects;
mod spells;
mod tilesets;
mod resources;

pub use header_def::*;
pub use loader::*;
#[allow(clippy::module_name_repetitions)]
pub use plugin::DataLoaderPlugin;
