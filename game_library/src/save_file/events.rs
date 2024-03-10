//! Events for loading and saving save files.

use bevy::prelude::*;

/// Types of save file actions that can be taken
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Load a save file
    Load,
    /// Save a save file
    Save,
    /// Update a save file
    Update,
}

/// Event that triggers the loading of a save file.
#[derive(Debug, Event)]
pub struct SaveFileEvent {
    /// Name of the save file to load.
    pub name: String,
    /// Action to take with the save file
    pub action: Action,
}
