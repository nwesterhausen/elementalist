use bevy::prelude::*;

/// The state of the game (broadly)
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum AppState {
    /// First boot, loading assets etc
    ///
    /// Sub-states:
    ///
    /// * LoadingAssets
    /// * AssetsLoaded
    #[default]
    AppLoading,
    /// Main menu screen
    ///
    /// Has buttons for:
    ///
    /// * Play
    /// * Settings
    /// * Exit
    MainMenu,
    /// Settings menu
    ///
    /// Sub-states:
    ///
    /// * Disabled (when the menu is not open)
    /// * Main (the main settings menu)
    /// * Display
    /// * Audio
    /// * Controls
    /// * Gameplay
    SettingsMenu,
    /// In game
    ///
    /// Sub-states:
    ///
    /// * Loading
    /// * Saving
    /// * GameMenu (preparation area)
    /// * Playing
    /// * Status ("inventory", etc; pauses the game)
    /// * Results (after match summary)
    /// * Settings (in-game settings)
    InGame,
    /// Clean up and exit (save, etc)
    ///
    /// Sub-states:
    ///
    /// * Saving
    /// * Exiting
    CleanUp,
}
