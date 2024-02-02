//! Helper to know what state of the app to return to when entering the menu.

use bevy::prelude::*;

use game_library::state::AppState;

/// Helper to know what state of the app to return to when entering the menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Resource)]
pub struct ReturnToState(pub AppState);

impl Default for ReturnToState {
    fn default() -> Self {
        Self(AppState::MainMenu)
    }
}
