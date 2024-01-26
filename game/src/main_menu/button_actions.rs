use bevy::prelude::*;

/// All of the various "buttons" that can be clicked in any of the main menu screens
#[derive(Component, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ButtonAction {
    StartGame,
    Settings,
    Quit,
}
