/// Data loading events
pub mod events;
mod header_def;
mod loader;

/// The directory where the game data files are stored
pub const DATA_FILE_DIR: &str = "game_data";

pub use header_def::*;
pub use loader::*;
