use bevy::prelude::*;

/// All of the various "buttons" that can be clicked in any of the main menu screens
#[derive(Component, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum ButtonAction {
    StartGame,
    Settings,
    Quit,
    BackToMenu,
    BackToSettings,
    SettingsAudio,
    SettingsVideo,
    SettingsControls,
    SettingsGameplay,
    // ChangeFont .. maybe we have to include internal values too..
    ChangeFont,
}
