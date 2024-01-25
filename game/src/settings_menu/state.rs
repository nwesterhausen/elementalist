//! The states that the settings menu can be in.

use bevy::prelude::*;

/// The menu states.
///
/// The menu can be in one of these states at any given time. Each state
/// roughly corresponds to a different screen in the menu.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum MenuState {
    /// Disabled (default) state. The menu is not opened.
    #[default]
    Disabled,
    /// The main menu screen. This should have links to the other screens.
    Main,
    /// The graphics/display menu screen.
    Display,
    /// The audio menu screen.
    Audio,
    /// The controls menu screen.
    Controls,
    /// The gameplay menu screen.
    Gameplay,
}
