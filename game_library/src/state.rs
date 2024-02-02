//! The states used in the game.
use bevy::prelude::*;

/// The state of the game (broadly)
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
#[allow(clippy::module_name_repetitions)]
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

/// The menu states.
///
/// The menu can be in one of these states at any given time. Each state
/// roughly corresponds to a different screen in the menu.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
#[allow(clippy::module_name_repetitions)]
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
    /// The accessibility menu screen.
    Accessibility,
}

/// State for the different stages of playing the game.
///
/// There is a planned flow for the game, once we get to [`AppState::InGame`]:
///
/// 0. [`SaveState::Loading`] - Loading a save file
/// 1. [`GameState::GameMenu`] - The game menu (preparation area). This has a few different sub-states:
///   - Spell book for choosing your spell load out
///   - Status/Skill screen which is an overview of your skill levels
///   - Skill detail screen is a detailed view of a specific skill, includes the skill's description and skill tree
///   - Spell smithy? Someplace to spend the primal essence you've collected to upgrade your spells / buy new ones
/// 2. [`GameState::LoadingAssets`] - Loading assets for the game (you've chosen your spells and are ready to play)
/// 3. (no state yet..?) - Generating the level
/// 4. [`GameState::Playing`] - Playing the game. This has a few different sub-states:
///     - [`GameState::StatusScreen`] - The status screen ("inventory", "stats display", etc; pauses the game)
///         - Skill overview which can -> skill detail screen
///         - Spell book (can't choose spells but can reference them)
///     - [`GameState::ResultsScreen`] - The results screen (after match summary)
/// 5. [`SaveState::Saving`] - Saving a save file after the match is over. This saves the new loot and skill progression
///     from the run. (This should be in the background and not interrupt the game?)
/// 6. Go back to Step 1 and repeat the loop.
///
/// [`SaveState::Saving`] should save after spending essence, changing spells, choosing skill perks. It should be
/// in the background and not interrupt the game.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States, Reflect)]
#[allow(clippy::module_name_repetitions)]
pub enum GameState {
    /// By default, the GameState is disabled, since it's not used until the game is entered
    #[default]
    Disabled,
    /// The game menu (preparation area)
    GameMenu,
    /// Loading assets for the game
    LoadingAssets,
    /// Playing the game
    Playing,
    /// The status screen ("inventory", etc; pauses the game)
    StatusScreen,
    /// The results screen (after match summary)
    ResultsScreen,
}

/// State for the status of the save file. This can be changed by systems to trigger saving/loading.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States, Reflect)]
#[allow(clippy::module_name_repetitions)]
pub enum SaveState {
    /// By default, the SaveState is disabled, since it's not used until the game is entered
    #[default]
    Disabled,
    /// Loading a save file
    Loading,
    /// Saving a save file
    Saving,
    /// Idle state
    Idle,
}

/// State for the in-game overlay options.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States, Reflect)]
#[allow(clippy::module_name_repetitions)]
pub enum OverlayState {
    /// By default, the OverlayState is disabled, since it's not used until the game is entered
    #[default]
    Disabled,
    /// The normal HUD overlay
    Normal,
    /// The status overlay (shows player's stats and inventory)
    Status,
    /// The spell book overlay (shows the player's spells)
    ///
    /// This is referential while in the game, and can't be used to change spells.
    SpellBook,
    /// The skill book overlay (shows the player's skills)
    ///
    /// This also is just for reference, and no new perks can be chosen.
    SkillBook,
    /// HUD hidden (minimal UI)
    Hidden,
}
