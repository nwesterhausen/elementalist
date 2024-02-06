//! The states used in the game.
//!
//! These are a collection of states that the game can be in. They are used to
//! control the flow of the game, and to determine what systems are active at
//! any given time.
//!
//! ## State Flow
//!
//! 1. [`AppState::Startup`] - The game is starting up. The splash screen is shown and
//! data is loaded from `game_data` and assets like fonts, sprites and tilesets are loaded.
//! 2. [`AppState::MainMenu`] - The main menu is shown.
//!     a. Click "Start" to begin playing -> [`AppState::InGame`] and [`GameState::Loading`]
//! are activated.
//!     b. Click "Settings" to open the settings menu -> [`AppState::SettingsMenu`] and [`MenuState::Main`]
//!     c. Click "Quit" to exit the game -> [`AppState::CleanUp`] (which will do any necessary saving and then exit)
//! 3. more stuff
use bevy::prelude::*;

/// The state of the game (broadly)
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
#[allow(clippy::module_name_repetitions)]
pub enum AppState {
    /// The game was just started and is loading.
    ///
    /// Data is read from `game_data` and assets like fonts, sprites and tilesets are loaded.
    #[default]
    Startup,
    /// Main menu screen
    MainMenu,
    /// In game (playing the game)
    InGame,
    /// Clean up and exit (save, etc)
    CleanUp,
}

/// A UI overlay state. This allows settings to be shown, or other overlays.
///
/// An overlay being activated does not necessarily mean the game is paused.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum Overlay {
    /// No overlay is active
    #[default]
    None,
    /// The settings overlay is active
    Settings,
    /// The credits overlay is active
    Credits,
    /// A status screen can only be opened when [`App::InGame`] and [`Game::Playing`]
    /// are active.
    Status,
    /// Spell book overlay. Can only be opened when [`App::InGame`]. If this is opened
    /// when [`Game::Playing`] is active, the game should be paused, and the spells should
    /// not be purchasable or changeable, i.e. informational only.
    SpellBook,
    /// Skill screen overlay. Can only be opened when [`App::InGame`]. If this is opened
    /// when [`Game::Playing`] is active, the game should be paused, and the skills should
    /// not be purchasable or changeable, i.e. informational only.
    SkillScreen,
    /// Control hints overlay. Can only be opened when [`App::InGame`]
    ControlHints,
    /// Results overlay. Opens automatically when [`App::InGame`] and [`Game::Finished`].
    /// The player can transition away when [`Save::Standby`] is active.
    Results,
}

/// The menu states.
///
/// The menu can be in one of these states at any given time. Each state
/// roughly corresponds to a different screen in the menu.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
#[allow(clippy::module_name_repetitions)]
pub enum Settings {
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

/// States of the game itself.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum Game {
    /// Disabled (default) state. The game is not running.
    #[default]
    Disabled,
    /// The game is loading. This triggers [`Save::Loading`] and should be automatically
    /// transitioned to [`Game::Main`] when the loading is complete ([`Save::Standby`]).
    Loading,
    /// The main state is the menu from which a new game can be launched. Also any spells
    /// or skills can be managed here.
    Main,
    /// The game is generating a new elemental realm. This is a transitional state that
    /// will automatically transition to [`Game::Playing`] when the realm is ready.
    Generating,
    /// The player is within a realm and playing the game.
    Playing,
    /// If the player dies or otherwise finishes in a realm, this state is activated. It
    /// pauses the game and shows the player's score and other information. It automatically
    /// calls [`Overlay::Results`] to show results and [`Save::Saving`] to save the game state.
    ///
    /// When the player clicks "Done" or "Restart", this state will automatically transition
    /// to [`Game::Main`] or [`Game::Generating`] respectively. The save system should be able
    /// to operate on its own and not require the game to be in a specific state.
    Finished,
}

/// The state of the save system.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum Save {
    /// The game is loading a save file.
    Loading,
    /// The game was loaded and is ready to play, OR the game has finished saving.
    /// This automatically transitions to [`Save::Standby`] after doing any transitions
    /// to move the [`Game`] state to the correct state.
    Ready,
    /// The game is saving the current state.
    Saving,
    /// The game is ready to save (or load). This is the default state.
    #[default]
    Standby,
}
