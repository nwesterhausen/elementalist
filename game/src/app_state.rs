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
    /// Has sub-states for each menu screen (audio, video, etc)
    MainMenu,
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
