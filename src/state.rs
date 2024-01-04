use bevy::prelude::*;

/// Global game state enum
#[derive(Clone, Copy, Default, Eq, PartialEq, Hash, Debug, States)]
pub enum GameState {
    #[default]
    /// First launch, show splash screen until assets are loaded
    BootingApp,
    /// The main menu
    MainMenu,
    /// Inside the game (a loaded game)
    Game,
    /// A specific run as part of the game (tracks progression on a wider scale)
    SingleRun,
    /// The in-game menu
    InGameMenu,
    /// Game loading screen (if ever needed)
    GameLoading,
}

impl GameState {
    /// List of non-interactive states (only render the game)
    const NON_INTERACTIVE: [Self; 2] = [Self::BootingApp, Self::GameLoading];

    /// List of menu states (only interact with the UI)
    const UI_STATES: [Self; 3] = [Self::MainMenu, Self::InGameMenu, Self::Game];

    /// Check if the state is non-interactive
    pub fn is_non_interactive(&self) -> bool {
        Self::NON_INTERACTIVE.contains(self)
    }

    /// Check if the state is a menu
    pub fn is_menu(&self) -> bool {
        Self::UI_STATES.contains(self)
    }

    /// Check if normal gameplay is happening
    pub fn is_gameplay(&self) -> bool {
        *self == Self::SingleRun
    }
}
