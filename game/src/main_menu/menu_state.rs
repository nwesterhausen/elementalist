use bevy::prelude::*;

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Settings,
    SettingsDisplay,
    SettingsSound,
    SettingsControls,
    SettingsGameplay,
    #[default]
    Disabled,
}
