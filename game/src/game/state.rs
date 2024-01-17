use bevy::prelude::*;

/// The state of the "game" game
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GameState {
    /// By default, the GameState is disabled, since it's not used until the game is entered
    #[default]
    Disabled,
    // Todo: re-enable these states and have a proper game loop
    // /// Loading a save file
    // Loading,
    // /// Saving a save file
    // Saving,
    // /// The game menu (preparation area)
    // GameMenu,
    // /// Loading assets for the game
    // LoadingAssets,
    // /// Playing the game
    // Playing,
    // /// The status screen ("inventory", etc; pauses the game)
    // StatusScreen,
    // /// The results screen (after match summary)
    // ResultsScreen,
    // /// The settings screen (in-game settings)
    // InGameMenu,
}
